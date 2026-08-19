use clap::Parser;
use hidapi::{HidApi, HidDevice};
use realshifter_core::{Config, GearPosition, SessionState};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

const LOGITECH_VENDOR_ID: u16 = 0x046d;
const ARDUINO_VENDOR_ID: u16 = 0x2341;

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
    let candidates: Vec<_> = api
        .device_list()
        .filter(|d| {
            let vid = d.vendor_id();
            vid == LOGITECH_VENDOR_ID || vid == ARDUINO_VENDOR_ID || d.usage_page() == 0x0001
        })
        .collect();

    for dev in &candidates {
        println!(
            "Found HID candidate: {:?} ({:04x}:{:04x}) UsagePage: {:04x}, Usage: {:04x}, Interface: {}",
            dev.product_string().unwrap_or("Unknown"),
            dev.vendor_id(),
            dev.product_id(),
            dev.usage_page(),
            dev.usage(),
            dev.interface_number()
        );
    }

    let device_info = candidates
        .iter()
        .find(|d| {
            d.usage_page() == 0x0001
                && (d.usage() == 0x0004 || d.usage() == 0x0005 || d.usage() == 0x0008)
        })
        .or_else(|| candidates.iter().find(|d| d.interface_number() > 0))
        .or_else(|| candidates.first())
        .copied()
        .ok_or(
            "No supported USB Shifter/Joystick device detected (Logitech, Arduino, or Generic HID)",
        )?;

    println!(
        "Connected to USB Shifter HID Device: {:?} ({:04x}:{:04x}) [UsagePage: {:04x}, Usage: {:04x}, Interface: {}]",
        device_info.product_string().unwrap_or("USB Shifter"),
        device_info.vendor_id(),
        device_info.product_id(),
        device_info.usage_page(),
        device_info.usage(),
        device_info.interface_number()
    );

    let device: HidDevice = device_info.open_device(&api)?;
    device.set_blocking_mode(false)?;

    let mut buf = [0u8; 64];

    loop {
        match device.read_timeout(&mut buf, 50) {
            Ok(size) if size > 0 => {
                let report = &buf[..size];
                let new_gear = parse_shifter_hid_report(report);
                if new_gear != *last_gear {
                    println!(
                        "[HID Report] Raw: {:02x?} -> Parsed: {}",
                        report,
                        new_gear.full_name()
                    );
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
    if report.is_empty() {
        return GearPosition::Neutral;
    }

    // If the first byte is an HID Report ID (e.g. 0x03 from Arduino Joystick or 0x01/0x02),
    // and there is at least a second byte, the button bitmask is in report[1..]
    let button_bytes = if report.len() >= 2 && report[0] == 0x03 {
        &report[1..]
    } else if report.len() >= 2 && (report[0] == 0x01 || report[0] == 0x02) && report[1] != 0 {
        &report[1..]
    } else {
        report
    };

    for btn in 0..7u8 {
        let byte_idx = (btn / 8) as usize;
        let bit_idx = btn % 8;
        if byte_idx < button_bytes.len() && (button_bytes[byte_idx] & (1 << bit_idx)) != 0 {
            return GearPosition::from_hid_button(btn);
        }
    }
    GearPosition::Neutral
}

fn handle_gear_shift(new_gear: GearPosition) {
    let mut state = SessionState::load();
    let config = Config::load();
    let service = realshifter_core::PaneAutomationService::default();

    let active_profile = state.active_profile;
    let target_pane = service.resolve_target_pane();

    if let Some(mapping) = config.get_mapping(active_profile, new_gear) {
        if mapping.is_enabled {
            let label = mapping.display_label();
            let command_str = mapping.effective_command();

            state.record_shift(new_gear, Some(label.clone()));
            if let Err(e) = state.save() {
                eprintln!("Failed to save state: {e}");
            }

            println!(
                "[In-Process Shift] {} [{}] -> {} (pane: {})",
                new_gear.full_name(),
                active_profile.display_name(),
                label,
                target_pane
            );

            service.dispatch_action(&target_pane, &mapping.action_type, &command_str);
            return;
        }
    }

    state.record_shift(new_gear, None);
    if let Err(e) = state.save() {
        eprintln!("Failed to save state: {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_shifter_hid_report() {
        let empty_report = [0u8; 8];
        assert_eq!(
            parse_shifter_hid_report(&empty_report),
            GearPosition::Neutral
        );

        // Button 0 -> Gear 1 (bit 0 of byte 0 set)
        let gear1_report = [0b0000_0001, 0, 0, 0];
        assert_eq!(parse_shifter_hid_report(&gear1_report), GearPosition::Gear1);

        // Button 5 -> Gear 6 (bit 5 of byte 0 set)
        let gear6_report = [0b0010_0000, 0, 0, 0];
        assert_eq!(parse_shifter_hid_report(&gear6_report), GearPosition::Gear6);

        // Button 6 -> Reverse (bit 6 of byte 0 set)
        let reverse_report = [0b0100_0000, 0, 0, 0];
        assert_eq!(
            parse_shifter_hid_report(&reverse_report),
            GearPosition::Reverse
        );

        // Arduino Leonardo Report ID 0x03 tests
        assert_eq!(
            parse_shifter_hid_report(&[0x03, 0x00]),
            GearPosition::Neutral
        );
        assert_eq!(parse_shifter_hid_report(&[0x03, 0x01]), GearPosition::Gear1);
        assert_eq!(parse_shifter_hid_report(&[0x03, 0x02]), GearPosition::Gear2);
        assert_eq!(parse_shifter_hid_report(&[0x03, 0x04]), GearPosition::Gear3);
        assert_eq!(parse_shifter_hid_report(&[0x03, 0x08]), GearPosition::Gear4);
        assert_eq!(parse_shifter_hid_report(&[0x03, 0x10]), GearPosition::Gear5);
        assert_eq!(parse_shifter_hid_report(&[0x03, 0x20]), GearPosition::Gear6);
        assert_eq!(
            parse_shifter_hid_report(&[0x03, 0x40]),
            GearPosition::Reverse
        );
    }

    #[test]
    fn test_handle_gear_shift_and_trigger_action() {
        let temp_dir = std::env::temp_dir().join(format!("rs_daemon_test_{}", std::process::id()));
        unsafe {
            std::env::set_var("HERDR_PLUGIN_STATE_DIR", &temp_dir);
            std::env::set_var("HERDR_ACTION_BIN", "/usr/bin/true");
        }

        handle_gear_shift(GearPosition::Gear1);
        let state = SessionState::load();
        assert_eq!(state.current_gear, GearPosition::Gear1);

        handle_gear_shift(GearPosition::Neutral);
        let state_n = SessionState::load();
        assert_eq!(state_n.current_gear, GearPosition::Neutral);

        unsafe {
            std::env::remove_var("HERDR_ACTION_BIN");
        }
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
