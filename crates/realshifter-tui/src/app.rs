use realshifter_core::{CliProfile, Config, GearPosition, SessionState};
use std::fs;
use std::process::Command;
use std::time::SystemTime;

pub struct App {
    pub config: Config,
    pub state: SessionState,
    pub should_quit: bool,
    pub show_models_modal: bool,
    pub view_profile: CliProfile,
    pub status_message: String,
    last_state_mtime: Option<SystemTime>,
    last_config_mtime: Option<SystemTime>,
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            config: Config::default(),
            state: SessionState::default(),
            should_quit: false,
            show_models_modal: false,
            view_profile: CliProfile::AgyCli,
            status_message: "Ready".to_string(),
            last_state_mtime: None,
            last_config_mtime: None,
        };
        app.refresh();
        app
    }

    pub fn refresh(&mut self) {
        let cfg_path = Config::config_path();
        let cfg_mtime = fs::metadata(&cfg_path).and_then(|m| m.modified()).ok();
        if cfg_mtime != self.last_config_mtime || self.last_config_mtime.is_none() {
            self.config = Config::load();
            self.last_config_mtime = cfg_mtime;
        }

        let st_path = SessionState::state_path();
        let st_mtime = fs::metadata(&st_path).and_then(|m| m.modified()).ok();
        if st_mtime != self.last_state_mtime || self.last_state_mtime.is_none() {
            self.state = SessionState::load();
            self.last_state_mtime = st_mtime;
        }
    }

    pub fn shift_gear(&mut self, gear: GearPosition) {
        self.state.current_gear = gear;

        let label = self
            .config
            .get_mapping(self.view_profile, gear)
            .map(|m| m.display_label());

        self.state.record_shift(gear, label);
        if let Err(e) = self.state.save() {
            eprintln!("Failed to save state: {e}");
        }

        let action_bin = realshifter_core::action_binary_path();
        let _ = Command::new(action_bin)
            .arg("shift")
            .arg(gear.display_name())
            .spawn();

        self.status_message = format!("Shifted to {}", gear.full_name());
    }

    pub fn cycle_view_profile(&mut self) {
        let profiles = CliProfile::all();
        let current_idx = profiles
            .iter()
            .position(|p| *p == self.view_profile)
            .unwrap_or(0);
        let next_idx = (current_idx + 1) % profiles.len();
        self.view_profile = profiles[next_idx];
        self.status_message = format!("Viewing profile {}", self.view_profile.display_name());
    }

    pub fn toggle_models_modal(&mut self) {
        self.show_models_modal = !self.show_models_modal;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_lifecycle_and_methods() {
        let temp_dir = std::env::temp_dir().join(format!("rs_tui_test_{}", std::process::id()));
        unsafe {
            std::env::set_var("HERDR_PLUGIN_CONFIG_DIR", &temp_dir);
            std::env::set_var("HERDR_PLUGIN_STATE_DIR", &temp_dir);
            std::env::set_var("HERDR_ACTION_BIN", "/usr/bin/true");
        }

        let mut app = App::new();
        assert_eq!(app.view_profile, CliProfile::AgyCli);
        assert!(!app.should_quit);
        assert!(!app.show_models_modal);

        app.cycle_view_profile();
        assert_ne!(app.view_profile, CliProfile::AgyCli);

        app.toggle_models_modal();
        assert!(app.show_models_modal);

        app.shift_gear(GearPosition::Gear1);
        assert_eq!(app.state.current_gear, GearPosition::Gear1);
        assert!(app.status_message.contains("Shifted to"));

        app.refresh();

        unsafe {
            std::env::remove_var("HERDR_ACTION_BIN");
        }
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
