use clap::{Parser, Subcommand};
use realshifter_core::{Config, GearPosition, PaneAutomationService, SessionState};

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
    /// Set specific CLI profile by name (agy, claude, codex, opencode, custom)
    Set { name: String },
    /// Pin a specific pane ID as the target for gear shifts
    PinPane { pane_id: String },
    /// Clear the pinned pane (resume auto-detection)
    UnpinPane,
    /// Show the currently pinned pane and active profile
    Status,
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
    let service = PaneAutomationService::default();

    let target_pane = service.resolve_target_pane();
    let active_profile = state.active_profile;

    let mapping = match config.get_mapping(active_profile, target_gear) {
        Some(m) if m.is_enabled => m,
        _ => {
            state.record_shift(target_gear, None);
            if let Err(e) = state.save() {
                eprintln!("Failed to save state: {e}");
            }
            println!(
                "Gear {} has no active mapping for profile {}.",
                target_gear.display_name(),
                active_profile.display_name()
            );
            return;
        }
    };

    let label = mapping.display_label();
    let command_str = mapping.effective_command();

    state.record_shift(target_gear, Some(label.clone()));
    if let Err(e) = state.save() {
        eprintln!("Failed to save state: {e}");
    }

    println!(
        "Executing action for {} [{}]: {} -> {} (pane: {})",
        target_gear.full_name(),
        active_profile.display_name(),
        label,
        command_str,
        target_pane
    );

    service.dispatch_action(&target_pane, &mapping.action_type, &command_str);
}

fn handle_profile_action(action: ProfileAction) {
    let mut state = SessionState::load();
    match action {
        ProfileAction::Next => {
            let next_profile = state.active_profile.next();
            switch_to_profile(&mut state, next_profile);
        }
        ProfileAction::Set { name } => {
            if let Ok(p) = name.parse::<realshifter_core::CliProfile>() {
                switch_to_profile(&mut state, p);
            } else {
                eprintln!(
                    "Unknown profile name: '{name}'. Use: agy, claude, codex, opencode, pi, custom"
                );
            }
        }
        ProfileAction::PinPane { pane_id } => {
            state.pinned_pane_id = Some(pane_id.clone());
            if let Err(e) = state.save() {
                eprintln!("Failed to save state: {e}");
            }
            println!(
                "Pinned pane: {pane_id} (profile: {})",
                state.active_profile.display_name()
            );
        }
        ProfileAction::UnpinPane => {
            state.pinned_pane_id = None;
            if let Err(e) = state.save() {
                eprintln!("Failed to save state: {e}");
            }
            println!("Cleared pinned pane — resuming auto-detection.");
        }
        ProfileAction::Status => {
            let pinned = state
                .pinned_pane_id
                .as_deref()
                .unwrap_or("(none — auto-detect)");
            println!("Active profile: {}", state.active_profile.display_name());
            println!("Pinned pane:    {pinned}");
        }
    }
}

fn switch_to_profile(state: &mut SessionState, new_profile: realshifter_core::CliProfile) {
    state.active_profile = new_profile;
    state.pinned_pane_id = None;
    let _ = state.save();

    let service = PaneAutomationService::default();
    let target = service.resolve_target_pane();
    let target_opt = if !target.is_empty() && target != "active" {
        Some(target)
    } else {
        None
    };

    state.update_active_profile(new_profile, target_opt);
    let _ = state.save();

    println!(
        "Active profile: {} (target: {})",
        state.active_profile.display_name(),
        state.pinned_pane_id.as_deref().unwrap_or("auto")
    );
}

fn handle_agent_complete() {
    println!("Herdr agent complete event received.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_target_pane_default() {
        let service = PaneAutomationService::default();
        let pane = service.resolve_target_pane();
        assert!(!pane.is_empty());
    }

    #[test]
    fn test_handle_agent_complete() {
        handle_agent_complete();
    }

    #[test]
    fn test_handle_profile_action() {
        handle_profile_action(ProfileAction::Next);
        handle_profile_action(ProfileAction::Set {
            name: "agy".to_string(),
        });
    }
}
