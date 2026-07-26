use crate::{CliProfile, GearMapping, GearPosition};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub active_profile: CliProfile,
    pub profile_mappings: HashMap<CliProfile, Vec<GearMapping>>,
    pub test_mode: bool,
    pub preferred_terminal: String,
}

impl Default for Config {
    fn default() -> Self {
        let mut mappings = HashMap::new();
        for profile in CliProfile::all() {
            mappings.insert(*profile, profile.default_mappings());
        }
        Self {
            active_profile: CliProfile::ClaudeCode,
            profile_mappings: mappings,
            test_mode: false,
            preferred_terminal: "Terminal".to_string(),
        }
    }
}

impl Config {
    pub fn config_path() -> PathBuf {
        if let Ok(dir) = std::env::var("HERDR_PLUGIN_CONFIG_DIR") {
            PathBuf::from(dir).join("config.json")
        } else if let Some(mut dir) = dirs::config_dir() {
            dir.push("realshifter");
            dir.join("config.json")
        } else {
            PathBuf::from("/tmp/realshifter-config.json")
        }
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if let Ok(contents) = fs::read_to_string(&path) {
            match serde_json::from_str::<Config>(&contents) {
                Ok(cfg) => return cfg,
                Err(_) => eprintln!("Warning: Failed to parse config.json, using defaults."),
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())
    }

    pub fn get_mapping(&self, profile: CliProfile, gear: GearPosition) -> Option<GearMapping> {
        self.profile_mappings
            .get(&profile)?
            .iter()
            .find(|m| m.gear == gear)
            .cloned()
    }

    pub fn active_mapping(&self, gear: GearPosition) -> Option<GearMapping> {
        self.get_mapping(self.active_profile, gear)
    }

    pub fn cycle_profile(&mut self) -> CliProfile {
        let profiles = CliProfile::all();
        let current_idx = profiles
            .iter()
            .position(|p| *p == self.active_profile)
            .unwrap_or(0);
        let next_idx = (current_idx + 1) % profiles.len();
        self.active_profile = profiles[next_idx];
        self.active_profile
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_load_save() {
        let dir = std::env::temp_dir().join(format!("rs_cfg_test_{}", std::process::id()));
        unsafe {
            std::env::set_var("HERDR_PLUGIN_CONFIG_DIR", &dir);
        }

        let mut cfg = Config::default();
        cfg.active_profile = CliProfile::CodexCli;
        cfg.save().unwrap();

        let file_path = dir.join("config.json");
        assert!(file_path.exists());
        let loaded = Config::load();
        assert_eq!(loaded.active_profile, CliProfile::CodexCli);

        let _ = fs::remove_dir_all(&dir);
    }
}
