use clap::Parser;
use realshifter_core::{Config, GearPosition, SessionState};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

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

    println!("Starting RealShifter HID Listener Daemon...");

    let mut last_gear = GearPosition::Neutral;

    // Main daemon event loop
    loop {
        // Read current state from state.json
        let state = SessionState::load();
        let config = Config::load();

        // Check if gear changed in state or simulate polling
        if state.current_gear != last_gear {
            let new_gear = state.current_gear;
            last_gear = new_gear;

            if new_gear.is_driving() {
                if let Some(mapping) = config.active_mapping(new_gear) {
                    if mapping.is_enabled {
                        trigger_action(new_gear);
                    }
                }
            }
        }

        // Sleep to avoid high CPU usage
        thread::sleep(Duration::from_millis(50));
    }
}

fn trigger_action(gear: GearPosition) {
    let action_bin = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join("realshifter-action")))
        .unwrap_or_else(|| "realshifter-action".into());

    let _ = Command::new(action_bin)
        .arg("shift")
        .arg(gear.display_name())
        .spawn();
}
