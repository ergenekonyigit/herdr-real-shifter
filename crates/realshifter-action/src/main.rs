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

fn resolve_active_context(service: &PaneAutomationService) -> (String, realshifter_core::CliProfile) {
    let pane = service.resolve_target_pane();
    let profile = service.detect_active_profile(&pane);
    (pane, profile)
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
    let (target_pane, active_profile) = resolve_active_context(&service);

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

    service.dispatch_action(&target_pane, &mapping.action_type, &command_str);
}

fn handle_profile_action(_action: ProfileAction) {
    let service = PaneAutomationService::default();
    let (target_pane, detected) = resolve_active_context(&service);
    println!("Auto-detected active profile for pane '{}': {}", target_pane, detected.display_name());
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
        let (pane, profile) = resolve_active_context(&service);
        assert!(!pane.is_empty());
        assert_eq!(profile, realshifter_core::CliProfile::AgyCli);
    }

    #[test]
    fn test_handle_agent_complete() {
        handle_agent_complete();
    }

    #[test]
    fn test_handle_profile_action() {
        handle_profile_action(ProfileAction::Next);
        handle_profile_action(ProfileAction::Set { name: "agy".to_string() });
    }
}
