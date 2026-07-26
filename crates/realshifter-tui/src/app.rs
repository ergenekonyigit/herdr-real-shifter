use realshifter_core::{Config, GearPosition, SessionState};
use std::process::Command;

pub struct App {
    pub config: Config,
    pub state: SessionState,
    pub should_quit: bool,
    pub status_message: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            config: Config::load(),
            state: SessionState::load(),
            should_quit: false,
            status_message: "Ready".to_string(),
        }
    }

    pub fn refresh(&mut self) {
        self.config = Config::load();
        self.state = SessionState::load();
    }

    pub fn shift_gear(&mut self, gear: GearPosition) {
        self.state.current_gear = gear;

        let label = self
            .config
            .active_mapping(gear)
            .map(|m| m.display_label());

        self.state.record_shift(gear, label);
        let _ = self.state.save();

        // Trigger realshifter-action CLI
        let action_bin = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join("realshifter-action")))
            .unwrap_or_else(|| "realshifter-action".into());

        let _ = Command::new(action_bin)
            .arg("shift")
            .arg(gear.display_name())
            .spawn();

        self.status_message = format!("Shifted to {}", gear.full_name());
    }

    pub fn cycle_profile(&mut self) {
        let new_profile = self.config.cycle_profile();
        let _ = self.config.save();
        self.status_message = format!("Profile changed to {}", new_profile.display_name());
    }
}
