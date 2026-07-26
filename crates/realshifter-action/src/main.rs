use clap::{Parser, Subcommand};
use realshifter_core::{Config, GearActionType, GearPosition, SessionState};
use serde_json::Value;
use std::process::Command;
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about = "RealShifter Action Executor")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Shift to specified gear position
    Shift {
        /// Target gear position (1-6, r/reverse, n/neutral)
        gear: String,
    },
    /// Profile management
    Profile {
        #[command(subcommand)]
        action: ProfileAction,
    },
    /// Event hook called when agent completes work
    OnAgentComplete,
}

#[derive(Subcommand, Debug)]
enum ProfileAction {
    /// Switch to the next CLI profile
    Next,
    /// Set specific CLI profile
    Set { name: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Shift { gear } => handle_shift(&gear),
        Commands::Profile { action } => handle_profile_action(action),
        Commands::OnAgentComplete => handle_agent_complete(),
    }
}

fn handle_shift(gear_str: &str) {
    let target_gear: GearPosition = match gear_str.parse() {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Invalid gear parameter '{gear_str}': {e}");
            std::process::exit(1);
        }
    };

    let config = Config::load();
    let mut state = SessionState::load();

    if !target_gear.is_driving() {
        state.record_shift(target_gear, None);
        let _ = state.save();
        println!("Shifted to {}", target_gear.full_name());
        return;
    }

    let mapping = match config.active_mapping(target_gear) {
        Some(m) if m.is_enabled => m,
        _ => {
            state.record_shift(target_gear, None);
            let _ = state.save();
            println!("Gear {} has no active/enabled mapping.", target_gear.display_name());
            return;
        }
    };

    let label = mapping.display_label();
    let command_str = mapping.effective_command();

    state.record_shift(target_gear, Some(label.clone()));
    let _ = state.save();

    println!("Executing action for {}: {} -> {}", target_gear.full_name(), label, command_str);

    dispatch_action(&mapping.action_type, &command_str);
}

fn handle_profile_action(action: ProfileAction) {
    let mut config = Config::load();
    match action {
        ProfileAction::Next => {
            let new_profile = config.cycle_profile();
            let _ = config.save();
            println!("Switched to active profile: {}", new_profile.display_name());
        }
        ProfileAction::Set { name } => match name.parse() {
            Ok(profile) => {
                config.active_profile = profile;
                let _ = config.save();
                println!("Switched to active profile: {}", profile.display_name());
            }
            Err(e) => {
                eprintln!("Unknown profile name '{name}': {e}");
                std::process::exit(1);
            }
        },
    }
}

fn handle_agent_complete() {
    println!("Herdr agent complete event received.");
}

fn dispatch_action(action_type: &GearActionType, command_str: &str) {
    let target_pane = resolve_target_pane();

    match action_type {
        GearActionType::Rollback => {
            // Send Interrupt (Ctrl+C), pause 300ms, then send undo command
            send_keys(&target_pane, "\x03");
            thread::sleep(Duration::from_millis(300));
            let rollback_cmd = if command_str.is_empty() {
                "/undo"
            } else {
                command_str
            };
            send_keys(&target_pane, &format!("{rollback_cmd}\n"));
        }
        _ => {
            if !command_str.is_empty() {
                send_keys(&target_pane, &format!("{command_str}\n"));
            }
        }
    }
}

fn resolve_target_pane() -> String {
    // 1. Check HERDR_PANE_ID env var
    if let Ok(pane_id) = std::env::var("HERDR_PANE_ID") {
        if !pane_id.trim().is_empty() {
            return pane_id;
        }
    }

    // 2. Parse HERDR_PLUGIN_CONTEXT_JSON env var
    if let Ok(ctx_json) = std::env::var("HERDR_PLUGIN_CONTEXT_JSON") {
        if let Ok(val) = serde_json::from_str::<Value>(&ctx_json) {
            if let Some(pane_id) = val.get("active_pane_id").and_then(|v| v.as_str()) {
                return pane_id.to_string();
            }
        }
    }

    // 3. Query $HERDR_BIN_PATH pane list for focused/active pane
    let herdr_bin = std::env::var("HERDR_BIN_PATH").unwrap_or_else(|_| "herdr".to_string());
    if let Ok(output) = Command::new(&herdr_bin).arg("pane").arg("list").output() {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.contains("active") || line.contains("*") || line.contains("focused") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if let Some(first) = parts.first() {
                        return first.trim_matches(':').to_string();
                    }
                }
            }
        }
    }

    // Default fallback pane
    "active".to_string()
}

fn send_keys(target_pane: &str, keys: &str) {
    let herdr_bin = std::env::var("HERDR_BIN_PATH").unwrap_or_else(|_| "herdr".to_string());

    // Try sending keys via Herdr
    let status = Command::new(&herdr_bin)
        .arg("pane")
        .arg("send-keys")
        .arg("--pane")
        .arg(target_pane)
        .arg(keys)
        .status();

    if let Ok(st) = status {
        if st.success() {
            return;
        }
    }

    // Fallback: Try tmux send-keys
    let tmux_status = Command::new("tmux")
        .arg("send-keys")
        .arg("-t")
        .arg(target_pane)
        .arg(keys)
        .status();

    if let Ok(st) = tmux_status {
        if st.success() {
            return;
        }
    }

    println!("[Simulated Key Dispatch to pane '{target_pane}']: {}", keys.trim());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_target_pane_default() {
        let pane = resolve_target_pane();
        assert!(!pane.is_empty());
    }
}
