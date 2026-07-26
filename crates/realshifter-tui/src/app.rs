use realshifter_core::{Config, GearPosition, SessionState};
use std::fs;
use std::process::Command;
use std::time::SystemTime;

pub struct App {
    pub config: Config,
    pub state: SessionState,
    pub should_quit: bool,
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
            .active_mapping(gear)
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

    pub fn cycle_profile(&mut self) {
        let new_profile = self.config.cycle_profile();
        if let Err(e) = self.config.save() {
            eprintln!("Failed to save config: {e}");
        }
        self.status_message = format!("Profile changed to {}", new_profile.display_name());
    }
}
