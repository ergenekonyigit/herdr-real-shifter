use clap::{Parser, Subcommand};
use realshifter_core::{CliProfile, Config, GearActionType, GearPosition, SessionState};
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
    let target_pane = resolve_target_pane();
    let active_profile = detect_active_profile(&target_pane);

    let mapping = match config.get_mapping(active_profile, target_gear) {
        Some(m) if m.is_enabled => m,
        _ => {
            state.record_shift(target_gear, None);
            if let Err(e) = state.save() {
                eprintln!("Failed to save state: {e}");
            }
            println!("Gear {} has no active mapping for profile {}.", target_gear.display_name(), active_profile.display_name());
            return;
        }
    };

    let label = mapping.display_label();
    let command_str = mapping.effective_command();

    state.record_shift(target_gear, Some(label.clone()));
    if let Err(e) = state.save() {
        eprintln!("Failed to save state: {e}");
    }

    println!("Executing action for {} [{}]: {} -> {}", target_gear.full_name(), active_profile.display_name(), label, command_str);

    dispatch_action(&target_pane, &mapping.action_type, &command_str);
}

fn handle_profile_action(_action: ProfileAction) {
    let target_pane = resolve_target_pane();
    let detected = detect_active_profile(&target_pane);
    println!("Auto-detected active profile for pane '{}': {}", target_pane, detected.display_name());
}

fn handle_agent_complete() {
    println!("Herdr agent complete event received.");
}

fn dispatch_action(target_pane: &str, action_type: &GearActionType, command_str: &str) {
    match action_type {
        GearActionType::AgyCli => {
            dispatch_agy_model_select(target_pane, command_str);
        }
        GearActionType::Rollback => {
            send_interrupt(target_pane);
            thread::sleep(Duration::from_millis(300));
            let rollback_cmd = if command_str.is_empty() {
                "/undo"
            } else {
                command_str
            };
            send_command(target_pane, rollback_cmd);
        }
        _ => {
            if !command_str.is_empty() {
                send_command(target_pane, command_str);
            }
        }
    }
}

fn dispatch_agy_model_select(target_pane: &str, model_spec: &str) {
    let spec = model_spec.trim().to_lowercase();
    let spec = spec.strip_prefix("/model").unwrap_or(&spec).trim();

    let (target_index, effort_opt) = match spec {
        s if s.contains("3.6") && s.contains("low") => (0, Some("low")),
        s if s.contains("3.6") && s.contains("medium") => (0, Some("medium")),
        s if s.contains("3.6") => (0, Some("high")),
        s if s.contains("3.5") && s.contains("low") => (1, Some("low")),
        s if s.contains("3.5") && s.contains("medium") => (1, Some("medium")),
        s if s.contains("3.5") => (1, Some("high")),
        s if s.contains("3.1") && s.contains("low") => (2, Some("low")),
        s if s.contains("3.1") => (2, Some("high")),
        s if s.contains("sonnet") => (3, None),
        s if s.contains("opus") => (4, None),
        s if s.contains("gpt-oss") || s.contains("120b") => (5, Some("medium")),
        _ => (0, None),
    };

    let herdr_bin = std::env::var("HERDR_BIN_PATH").unwrap_or_else(|_| "herdr".to_string());

    // Fallback: settings.json
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let settings_path = format!("{}/.gemini/antigravity-cli/settings.json", home);
    let settings_model = std::fs::read_to_string(&settings_path)
        .ok()
        .and_then(|s| {
            let v: serde_json::Value = serde_json::from_str(&s).ok()?;
            v.get("model").and_then(|m| m.as_str()).map(|m| m.to_lowercase())
        })
        .unwrap_or_default();

    // Read exact TUI state
    let pane_output = std::process::Command::new(&herdr_bin)
        .arg("pane")
        .arg("read")
        .arg(target_pane)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();

    let last_line = pane_output
        .lines()
        .filter(|l| !l.trim().is_empty())
        .last()
        .unwrap_or(&settings_model)
        .to_lowercase();

    let current_index = if last_line.contains("3.6") { 0 }
    else if last_line.contains("3.5") { 1 }
    else if last_line.contains("3.1") { 2 }
    else if last_line.contains("sonnet") { 3 }
    else if last_line.contains("opus") { 4 }
    else if last_line.contains("120b") || last_line.contains("gpt-oss") { 5 }
    else { 0 };

    let down_count = if target_index >= current_index {
        target_index - current_index
    } else {
        6 - current_index + target_index
    };

    let herdr_bin = std::env::var("HERDR_BIN_PATH").unwrap_or_else(|_| "herdr".to_string());

    // 1. Open /model modal
    let _ = Command::new(&herdr_bin)
        .arg("pane")
        .arg("send-text")
        .arg(target_pane)
        .arg("/model")
        .status();
    let _ = Command::new(&herdr_bin)
        .arg("pane")
        .arg("send-keys")
        .arg(target_pane)
        .arg("Enter")
        .status();

    thread::sleep(Duration::from_millis(300));

    // 2. Move down to target item from top (item 0)
    for _ in 0..down_count {
        let _ = Command::new(&herdr_bin)
            .arg("pane")
            .arg("send-keys")
            .arg(target_pane)
            .arg("Down")
            .status();
    }

    // 4. Adjust effort slider if applicable
    if let Some(effort) = effort_opt {
        match effort {
            "low" => {
                for _ in 0..3 {
                    let _ = Command::new(&herdr_bin)
                        .arg("pane")
                        .arg("send-keys")
                        .arg(target_pane)
                        .arg("Left")
                        .status();
                }
            }
            "medium" => {
                for _ in 0..3 {
                    let _ = Command::new(&herdr_bin)
                        .arg("pane")
                        .arg("send-keys")
                        .arg(target_pane)
                        .arg("Left")
                        .status();
                }
                let _ = Command::new(&herdr_bin)
                    .arg("pane")
                    .arg("send-keys")
                    .arg(target_pane)
                    .arg("Right")
                    .status();
            }
            "high" => {
                for _ in 0..3 {
                    let _ = Command::new(&herdr_bin)
                        .arg("pane")
                        .arg("send-keys")
                        .arg(target_pane)
                        .arg("Right")
                        .status();
                }
            }
            _ => {}
        }
    }

    // 5. Submit selection
    let _ = Command::new(&herdr_bin)
        .arg("pane")
        .arg("send-keys")
        .arg(target_pane)
        .arg("Enter")
        .status();
}

#[allow(clippy::collapsible_if)]
fn resolve_target_pane() -> String {
    // 1. Check HERDR_PANE_ID env var
    if let Ok(pane_id) = std::env::var("HERDR_PANE_ID") {
        let trimmed = pane_id.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
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
            if let Ok(val) = serde_json::from_str::<Value>(&stdout) {
                if let Some(panes) = val.get("result").and_then(|r| r.get("panes")).and_then(|p| p.as_array()) {
                    for pane in panes {
                        if pane.get("focused").and_then(|f| f.as_bool()).unwrap_or(false) {
                            if let Some(id) = pane.get("pane_id").and_then(|id| id.as_str()) {
                                return id.to_string();
                            }
                        }
                    }
                    if let Some(first_pane) = panes.first() {
                        if let Some(id) = first_pane.get("pane_id").and_then(|id| id.as_str()) {
                            return id.to_string();
                        }
                    }
                }
            }
        }
    }

    // Default fallback pane
    "active".to_string()
}

fn detect_active_profile(target_pane: &str) -> CliProfile {
    let herdr_bin = std::env::var("HERDR_BIN_PATH").unwrap_or_else(|_| "herdr".to_string());

    if let Ok(output) = Command::new(&herdr_bin).arg("pane").arg("list").output() {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Ok(val) = serde_json::from_str::<Value>(&stdout) {
                if let Some(panes) = val.get("result").and_then(|r| r.get("panes")).and_then(|p| p.as_array()) {
                    for pane in panes {
                        let id = pane.get("pane_id").and_then(|i| i.as_str()).unwrap_or("");
                        let is_focused = pane.get("focused").and_then(|f| f.as_bool()).unwrap_or(false);

                        if id == target_pane || (target_pane == "active" && is_focused) {
                            if let Some(agent) = pane.get("agent").and_then(|a| a.as_str()) {
                                match agent.to_lowercase().as_str() {
                                    "agy" | "antigravity" => return CliProfile::AgyCli,
                                    "claude" | "claude-code" => return CliProfile::ClaudeCode,
                                    "codex" => return CliProfile::CodexCli,
                                    "opencode" => return CliProfile::OpenCodeCli,
                                    _ => {}
                                }
                            }
                            if let Some(title) = pane.get("terminal_title").and_then(|t| t.as_str()) {
                                let t_low = title.to_lowercase();
                                if t_low.contains("agy") { return CliProfile::AgyCli; }
                                if t_low.contains("claude") { return CliProfile::ClaudeCode; }
                                if t_low.contains("codex") { return CliProfile::CodexCli; }
                                if t_low.contains("opencode") { return CliProfile::OpenCodeCli; }
                            }
                        }
                    }
                }
            }
        }
    }

    if let Ok(output) = Command::new(&herdr_bin).arg("pane").arg("read").arg(target_pane).output() {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
            if stdout.contains("antigravity") || stdout.contains("gemini") {
                return CliProfile::AgyCli;
            }
            if stdout.contains("claude") {
                return CliProfile::ClaudeCode;
            }
            if stdout.contains("codex") {
                return CliProfile::CodexCli;
            }
            if stdout.contains("opencode") {
                return CliProfile::OpenCodeCli;
            }
        }
    }

    CliProfile::AgyCli
}

fn send_interrupt(target_pane: &str) {
    let herdr_bin = std::env::var("HERDR_BIN_PATH").unwrap_or_else(|_| "herdr".to_string());

    let _ = Command::new(&herdr_bin)
        .arg("pane")
        .arg("send-keys")
        .arg(target_pane)
        .arg("Ctrl+c")
        .status();
}

fn send_command(target_pane: &str, command: &str) {
    let clean_cmd = command.trim_end_matches('\n');
    let herdr_bin = std::env::var("HERDR_BIN_PATH").unwrap_or_else(|_| "herdr".to_string());

    // 1. Try Herdr send-text + Enter
    let herdr_text_res = Command::new(&herdr_bin)
        .arg("pane")
        .arg("send-text")
        .arg(target_pane)
        .arg(clean_cmd)
        .status();

    if matches!(herdr_text_res, Ok(st) if st.success()) {
        let _ = Command::new(&herdr_bin)
            .arg("pane")
            .arg("send-keys")
            .arg(target_pane)
            .arg("Enter")
            .status();
        return;
    }

    // 2. Fallback: tmux send-keys
    let tmux_res = Command::new("tmux")
        .arg("send-keys")
        .arg("-t")
        .arg(target_pane)
        .arg(&format!("{clean_cmd}\n"))
        .status();

    if matches!(tmux_res, Ok(st) if st.success()) {
        return;
    }

    println!("[Simulated Command Dispatch to pane '{target_pane}']: {clean_cmd}");
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
