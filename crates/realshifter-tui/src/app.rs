use realshifter_core::{CliProfile, Config, GearActionType, GearMapping, GearPosition, SessionState};
use std::fs;
use std::process::Command;
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditField {
    ActionType,
    Model,
    Effort,
    CustomCommand,
    Label,
    Save,
    Cancel,
}

impl EditField {
    pub fn next(&self) -> Self {
        match self {
            EditField::ActionType => EditField::Model,
            EditField::Model => EditField::Effort,
            EditField::Effort => EditField::CustomCommand,
            EditField::CustomCommand => EditField::Label,
            EditField::Label => EditField::Save,
            EditField::Save => EditField::Cancel,
            EditField::Cancel => EditField::ActionType,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            EditField::ActionType => EditField::Cancel,
            EditField::Model => EditField::ActionType,
            EditField::Effort => EditField::Model,
            EditField::CustomCommand => EditField::Effort,
            EditField::Label => EditField::CustomCommand,
            EditField::Save => EditField::Label,
            EditField::Cancel => EditField::Save,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EditState {
    pub gear: GearPosition,
    pub action_type: GearActionType,
    pub selected_model_id: String,
    pub selected_effort: String,
    pub custom_command: String,
    pub label: String,
    pub focused_field: EditField,
}

pub struct App {
    pub config: Config,
    pub state: SessionState,
    pub should_quit: bool,
    pub view_profile: CliProfile,
    pub active_profile: CliProfile,
    pub selected_gear_index: usize,
    pub edit_state: Option<EditState>,
    pub show_models_modal: bool,
    pub show_help_modal: bool,
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
            view_profile: CliProfile::AgyCli,
            active_profile: CliProfile::AgyCli,
            selected_gear_index: 0,
            edit_state: None,
            show_models_modal: false,
            show_help_modal: false,
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

    pub fn selected_gear(&self) -> GearPosition {
        GearPosition::all()[self.selected_gear_index]
    }

    pub fn select_next_gear(&mut self) {
        let count = GearPosition::all().len();
        self.selected_gear_index = (self.selected_gear_index + 1) % count;
    }

    pub fn select_prev_gear(&mut self) {
        let count = GearPosition::all().len();
        if self.selected_gear_index == 0 {
            self.selected_gear_index = count - 1;
        } else {
            self.selected_gear_index -= 1;
        }
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

    pub fn prev_view_profile(&mut self) {
        let profiles = CliProfile::all();
        let current_idx = profiles
            .iter()
            .position(|p| *p == self.view_profile)
            .unwrap_or(0);
        let prev_idx = if current_idx == 0 {
            profiles.len() - 1
        } else {
            current_idx - 1
        };
        self.view_profile = profiles[prev_idx];
        self.status_message = format!("Viewing profile {}", self.view_profile.display_name());
    }

    pub fn set_view_as_active_profile(&mut self) {
        self.active_profile = self.view_profile;
        let action_bin = realshifter_core::action_binary_path();
        let _ = Command::new(action_bin)
            .arg("profile")
            .arg("set")
            .arg(self.active_profile.file_name().replace(".json", ""))
            .spawn();

        self.status_message = format!("Active profile set to {}", self.active_profile.display_name());
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

    pub fn start_editing_selected_gear(&mut self) {
        let gear = self.selected_gear();
        let mapping = self.config.get_mapping(self.view_profile, gear);

        let (action_type, model_id, effort, custom_cmd, label) = if let Some(m) = mapping {
            let (m_id, eff) = parse_model_flag_parts(m.model_flag.as_deref().unwrap_or(""));
            (m.action_type, m_id, eff, m.command, m.label)
        } else {
            (
                GearActionType::from_profile(self.view_profile),
                String::new(),
                String::new(),
                String::new(),
                format!("{} {}", self.view_profile.display_name(), gear.display_name()),
            )
        };

        self.edit_state = Some(EditState {
            gear,
            action_type,
            selected_model_id: model_id,
            selected_effort: effort,
            custom_command: custom_cmd,
            label,
            focused_field: EditField::ActionType,
        });
        self.status_message = format!("Editing {} for {}", gear.full_name(), self.view_profile.display_name());
    }

    pub fn cancel_editing(&mut self) {
        self.edit_state = None;
        self.status_message = "Edit cancelled".to_string();
    }

    pub fn save_editing(&mut self) {
        if let Some(es) = self.edit_state.take() {
            let model_flag = if !es.selected_model_id.is_empty() {
                if !es.selected_effort.is_empty() {
                    Some(format!("{}-{}", es.selected_model_id, es.selected_effort))
                } else {
                    Some(es.selected_model_id.clone())
                }
            } else {
                None
            };

            let label = if !es.label.trim().is_empty() {
                es.label
            } else if let Some(ref m) = model_flag {
                format!("{} ({})", es.action_type.display_name(), m)
            } else {
                es.action_type.display_name().to_string()
            };

            let new_mapping = GearMapping::new(
                es.gear,
                es.action_type,
                es.custom_command,
                model_flag,
                label,
                true,
            );

            self.config.update_mapping(self.view_profile, new_mapping);
            if let Err(e) = self.config.save() {
                self.status_message = format!("Failed to save config: {e}");
            } else {
                self.status_message = format!("Saved mapping for {}", es.gear.full_name());
            }
        }
    }

    pub fn cycle_edit_action_type(&mut self) {
        if let Some(ref mut es) = self.edit_state {
            let actions = GearActionType::all();
            let idx = actions.iter().position(|a| *a == es.action_type).unwrap_or(0);
            es.action_type = actions[(idx + 1) % actions.len()];
        }
    }

    pub fn cycle_edit_model(&mut self) {
        let models: Vec<String> = self
            .config
            .available_models(self.view_profile)
            .iter()
            .map(|m| m.id.clone())
            .collect();

        if let Some(ref mut es) = self.edit_state {
            if models.is_empty() {
                es.selected_model_id = String::new();
                return;
            }
            if es.selected_model_id.is_empty() {
                es.selected_model_id = models[0].clone();
            } else if let Some(idx) = models.iter().position(|m| m == &es.selected_model_id) {
                let next_idx = (idx + 1) % (models.len() + 1);
                if next_idx < models.len() {
                    es.selected_model_id = models[next_idx].clone();
                } else {
                    es.selected_model_id = String::new();
                }
            } else {
                es.selected_model_id = models[0].clone();
            }
        }
    }

    pub fn cycle_edit_effort(&mut self) {
        if let Some(ref mut es) = self.edit_state {
            let levels = ["", "low", "medium", "high"];
            let idx = levels.iter().position(|l| *l == es.selected_effort).unwrap_or(0);
            es.selected_effort = levels[(idx + 1) % levels.len()].to_string();
        }
    }

    pub fn handle_edit_char(&mut self, c: char) {
        if let Some(ref mut es) = self.edit_state {
            match es.focused_field {
                EditField::CustomCommand => es.custom_command.push(c),
                EditField::Label => es.label.push(c),
                _ => {}
            }
        }
    }

    pub fn handle_edit_backspace(&mut self) {
        if let Some(ref mut es) = self.edit_state {
            match es.focused_field {
                EditField::CustomCommand => { es.custom_command.pop(); },
                EditField::Label => { es.label.pop(); },
                _ => {}
            }
        }
    }

    pub fn toggle_models_modal(&mut self) {
        self.show_models_modal = !self.show_models_modal;
        if self.show_models_modal {
            self.show_help_modal = false;
        }
    }

    pub fn toggle_help_modal(&mut self) {
        self.show_help_modal = !self.show_help_modal;
        if self.show_help_modal {
            self.show_models_modal = false;
        }
    }
}

fn parse_model_flag_parts(flag: &str) -> (String, String) {
    if flag.is_empty() {
        return (String::new(), String::new());
    }
    if let Some((base, eff)) = flag.rsplit_once('-') {
        if matches!(eff, "low" | "medium" | "high") {
            return (base.to_string(), eff.to_string());
        }
    }
    (flag.to_string(), String::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_navigation_and_editing() {
        let temp_dir = std::env::temp_dir().join(format!("rs_tui_app_test_{}", std::process::id()));
        unsafe {
            std::env::set_var("HERDR_PLUGIN_CONFIG_DIR", &temp_dir);
            std::env::set_var("HERDR_PLUGIN_STATE_DIR", &temp_dir);
            std::env::set_var("HERDR_ACTION_BIN", "/usr/bin/true");
        }

        let mut app = App::new();
        assert_eq!(app.selected_gear_index, 0);
        assert_eq!(app.selected_gear(), GearPosition::Neutral);

        app.select_next_gear();
        assert_eq!(app.selected_gear(), GearPosition::Gear1);
        app.select_prev_gear();
        assert_eq!(app.selected_gear(), GearPosition::Neutral);

        app.start_editing_selected_gear();
        assert!(app.edit_state.is_some());
        let es = app.edit_state.as_mut().unwrap();
        es.selected_model_id = "gemini-3.6-flash".to_string();
        es.selected_effort = "high".to_string();

        app.save_editing();
        assert!(app.edit_state.is_none());

        let mapping = app.config.get_mapping(CliProfile::AgyCli, GearPosition::Neutral).unwrap();
        assert_eq!(mapping.model_flag.as_deref(), Some("gemini-3.6-flash-high"));

        app.toggle_help_modal();
        assert!(app.show_help_modal);
        app.toggle_models_modal();
        assert!(app.show_models_modal);
        assert!(!app.show_help_modal);

        unsafe {
            std::env::remove_var("HERDR_ACTION_BIN");
        }
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
