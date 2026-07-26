use clap::Parser;
use hidapi::{HidApi, HidDevice};
use realshifter_core::{Config, GearPosition, SessionState};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

const LOGITECH_VENDOR_ID: u16 = 0x046d;

#[derive(Parser, Debug)]
#[command(author, version, about = "RealShifter IOKit USB HID Listener Daemon")]
struct Args {
    /// Detach into background daemon mode
    #[arg(long)]
    detach: bool,

    /// Internal child mode flag
    #[arg(long, hide = true)]
    child: bool,
}

fn main() {
    let args = Args::parse();

    if args.detach && !args.child {
        // Spawn background child process and exit parent immediately
        let current_exe = match std::env::current_exe() {
            Ok(exe) => exe,
            Err(e) => {
                eprintln!("Failed to get current binary path: {e}");
                std::process::exit(1);
            }
        };

        let result = Command::new(current_exe)
            .arg("--child")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();

        match result {
            Ok(_) => {
                println!("RealShifter daemon detached successfully.");
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Failed to detach RealShifter daemon: {e}");
                std::process::exit(1);
            }
        }
    }

    println!("Starting RealShifter IOKit USB HID Listener Daemon...");

    let mut last_gear = GearPosition::Neutral;

    loop {
        if let Err(err) = listen_hid_loop(&mut last_gear) {
            eprintln!("USB HID Shifter connection status: {err}. Retrying in 2 seconds...");
            thread::sleep(Duration::from_secs(2));
        }
    }
}

fn listen_hid_loop(last_gear: &mut GearPosition) -> Result<(), Box<dyn std::error::Error>> {
    let api = HidApi::new()?;
    let device_info = api
        .device_list()
        .find(|d| d.vendor_id() == LOGITECH_VENDOR_ID)
        .ok_or("No Logitech Driving Force Shifter USB device detected")?;

    println!(
        "Connected to USB Shifter HID Device: {:?} ({:04x}:{:04x})",
        device_info.product_string().unwrap_or("Logitech Shifter"),
        device_info.vendor_id(),
        device_info.product_id()
    );

    let device: HidDevice = device_info.open_device(&api)?;
    device.set_blocking_mode(false)?;

    let mut buf = [0u8; 64];

    loop {
        match device.read_timeout(&mut buf, 50) {
            Ok(size) if size > 0 => {
                let new_gear = parse_shifter_hid_report(&buf[..size]);
                if new_gear != *last_gear {
                    *last_gear = new_gear;
                    handle_gear_shift(new_gear);
                }
            }
            Ok(_) => {}
            Err(e) => return Err(Box::new(e)),
        }

        thread::sleep(Duration::from_millis(10));
    }
}

fn parse_shifter_hid_report(report: &[u8]) -> GearPosition {
    for btn in 0..7u8 {
        let byte_idx = (btn / 8) as usize;
        let bit_idx = btn % 8;
        if byte_idx < report.len() && (report[byte_idx] & (1 << bit_idx)) != 0 {
            return GearPosition::from_hid_button(btn);
        }
    }
    GearPosition::Neutral
}

fn handle_gear_shift(new_gear: GearPosition) {
    let config = Config::load();
    let mut state = SessionState::load();

    let label = config
        .active_mapping(new_gear)
        .map(|m| m.display_label());

    state.record_shift(new_gear, label);
    if let Err(e) = state.save() {
        eprintln!("Failed to save state: {e}");
    }

    if new_gear.is_driving() && config.active_mapping(new_gear).is_some_and(|m| m.is_enabled) {
        trigger_action(new_gear);
    }
}

fn trigger_action(gear: GearPosition) {
    let action_bin = realshifter_core::action_binary_path();

    if let Err(e) = Command::new(action_bin).arg("shift").arg(gear.display_name()).spawn() {
        eprintln!("Failed to spawn action process: {e}");
    }
}
