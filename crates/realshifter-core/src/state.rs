use crate::GearPosition;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub current_gear: GearPosition,
    #[serde(default)]
    pub active_profile: crate::CliProfile,
    /// Explicitly pinned target pane ID — overrides auto-detection when set.
    /// Set via dashboard (P key) or `realshifter-action profile pin-pane <PANE_ID>`.
    #[serde(default)]
    pub pinned_pane_id: Option<String>,
    pub shift_counts: HashMap<GearPosition, u64>,
    pub total_shifts: u64,
    pub last_action: Option<String>,
    pub last_action_timestamp: Option<u64>,
}

impl Default for SessionState {
    fn default() -> Self {
        let mut counts = HashMap::new();
        for gear in GearPosition::all() {
            counts.insert(*gear, 0);
        }
        Self {
            current_gear: GearPosition::Neutral,
            active_profile: crate::CliProfile::AgyCli,
            pinned_pane_id: None,
            shift_counts: counts,
            total_shifts: 0,
            last_action: None,
            last_action_timestamp: None,
        }
    }
}

impl SessionState {
    pub fn state_path() -> PathBuf {
        if let Ok(dir) = std::env::var("HERDR_PLUGIN_STATE_DIR") {
            PathBuf::from(dir).join("state.json")
        } else if let Some(mut dir) = dirs::state_dir() {
            dir.push("realshifter");
            dir.join("state.json")
        } else if let Some(mut dir) = dirs::data_dir() {
            dir.push("realshifter");
            dir.join("state.json")
        } else if let Some(mut dir) = dirs::config_dir() {
            dir.push("realshifter");
            dir.join("state.json")
        } else {
            PathBuf::from("/tmp/realshifter-state.json")
        }
    }

    pub fn load() -> Self {
        let path = Self::state_path();
        if let Ok(contents) = fs::read_to_string(&path) {
            match serde_json::from_str::<SessionState>(&contents) {
                Ok(st) => return st,
                Err(_) => {
                    eprintln!("Warning: Failed to parse state.json, using default session state.")
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::state_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())
    }

    pub fn record_shift(&mut self, gear: GearPosition, action_label: Option<String>) {
        self.current_gear = gear;
        let count = self.shift_counts.entry(gear).or_insert(0);
        *count += 1;
        self.total_shifts += 1;

        if let Some(label) = action_label {
            self.last_action = Some(label);
            self.last_action_timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .ok()
                .map(|d| d.as_secs());
        }
    }

    pub fn update_active_profile(
        &mut self,
        new_profile: crate::CliProfile,
        target_pane: Option<String>,
    ) {
        self.active_profile = new_profile;
        if let Some(target) = target_pane {
            if !target.is_empty() && target != "active" {
                self.pinned_pane_id = Some(target);
            } else {
                self.pinned_pane_id = None;
            }
        } else {
            self.pinned_pane_id = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_state_record_shift() {
        let mut st = SessionState::default();
        assert_eq!(st.current_gear, GearPosition::Neutral);
        st.record_shift(GearPosition::Gear1, Some("Claude Sonnet".to_string()));
        assert_eq!(st.current_gear, GearPosition::Gear1);
        assert_eq!(st.total_shifts, 1);
        assert_eq!(*st.shift_counts.get(&GearPosition::Gear1).unwrap(), 1);
        assert_eq!(st.last_action.as_deref(), Some("Claude Sonnet"));

        st.record_shift(GearPosition::Gear2, None);
        assert_eq!(st.current_gear, GearPosition::Gear2);
        assert_eq!(st.total_shifts, 2);
    }

    #[test]
    fn test_session_state_load_save() {
        let temp_dir = std::env::temp_dir().join(format!("rs_state_test_{}", std::process::id()));
        unsafe {
            std::env::set_var("HERDR_PLUGIN_STATE_DIR", &temp_dir);
        }

        let mut st = SessionState::default();
        st.record_shift(GearPosition::Gear3, Some("Shift 3".to_string()));
        st.save().unwrap();

        let loaded = SessionState::load();
        assert_eq!(loaded.current_gear, GearPosition::Gear3);
        assert_eq!(loaded.total_shifts, 1);

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
