use crate::{CliProfile, GearActionType, GearMapping, GearPosition};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub effort_levels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConfigMetadata {
    pub description: String,
    pub generated_at: String,
    pub available_models: Vec<ModelInfo>,
}

impl Default for ConfigMetadata {
    fn default() -> Self {
        Self {
            description:
                "RealShifter configuration snapshot of supported CLI models and effort levels."
                    .to_string(),
            generated_at: "2026-08-19T22:35:00+03:00".to_string(),
            available_models: vec![
                ModelInfo {
                    id: "gemini-3.7-flash".to_string(),
                    name: "Gemini 3.7 Flash".to_string(),
                    effort_levels: vec![
                        "low".to_string(),
                        "medium".to_string(),
                        "high".to_string(),
                    ],
                },
                ModelInfo {
                    id: "gemini-3.5-flash".to_string(),
                    name: "Gemini 3.5 Flash".to_string(),
                    effort_levels: vec![
                        "low".to_string(),
                        "medium".to_string(),
                        "high".to_string(),
                    ],
                },
                ModelInfo {
                    id: "gemini-3.1-pro".to_string(),
                    name: "Gemini 3.1 Pro".to_string(),
                    effort_levels: vec!["low".to_string(), "high".to_string()],
                },
                ModelInfo {
                    id: "claude-sonnet-4-6".to_string(),
                    name: "Claude Sonnet 4.6 (Thinking)".to_string(),
                    effort_levels: vec![],
                },
                ModelInfo {
                    id: "claude-opus-4-6-thinking".to_string(),
                    name: "Claude Opus 4.6 (Thinking)".to_string(),
                    effort_levels: vec![],
                },
                ModelInfo {
                    id: "gpt-oss-120b-medium".to_string(),
                    name: "GPT-OSS 120B (Medium)".to_string(),
                    effort_levels: vec!["medium".to_string()],
                },
            ],
        }
    }
}

impl ConfigMetadata {
    pub fn default_for(profile: CliProfile) -> Self {
        match profile {
            CliProfile::CodexCli => Self {
                description: "OpenAI Codex CLI model presets".to_string(),
                generated_at: "2026-08-19".to_string(),
                available_models: vec![
                    ModelInfo {
                        id: "gpt-5.6-terra".to_string(),
                        name: "GPT-5.6 Terra (Balanced)".to_string(),
                        effort_levels: vec![
                            "low".to_string(),
                            "medium".to_string(),
                            "high".to_string(),
                        ],
                    },
                    ModelInfo {
                        id: "gpt-5.6-luna".to_string(),
                        name: "GPT-5.6 Luna (Fast & Affordable)".to_string(),
                        effort_levels: vec![
                            "low".to_string(),
                            "medium".to_string(),
                            "high".to_string(),
                        ],
                    },
                    ModelInfo {
                        id: "gpt-5.5".to_string(),
                        name: "GPT-5.5 (Frontier Complex Coding)".to_string(),
                        effort_levels: vec!["medium".to_string(), "high".to_string()],
                    },
                    ModelInfo {
                        id: "gpt-5.4".to_string(),
                        name: "GPT-5.4 (Strong Everyday Coding)".to_string(),
                        effort_levels: vec![
                            "low".to_string(),
                            "medium".to_string(),
                            "high".to_string(),
                        ],
                    },
                    ModelInfo {
                        id: "gpt-5.4-mini".to_string(),
                        name: "GPT-5.4 Mini (Cost-efficient)".to_string(),
                        effort_levels: vec!["low".to_string(), "medium".to_string()],
                    },
                ],
            },
            CliProfile::OpenCodeCli => Self {
                description: "OpenCode free model presets".to_string(),
                generated_at: "2026-08-20".to_string(),
                available_models: vec![
                    ModelInfo {
                        id: "nemotron-3.5-lightning-free".to_string(),
                        name: "Nemotron 3.5 Lightning (Free)".to_string(),
                        effort_levels: vec![],
                    },
                    ModelInfo {
                        id: "deepseek-v4-flash-free".to_string(),
                        name: "DeepSeek V4 Flash (Free)".to_string(),
                        effort_levels: vec![
                            "default".to_string(),
                            "low".to_string(),
                            "high".to_string(),
                            "max".to_string(),
                        ],
                    },
                    ModelInfo {
                        id: "laguna-s-2.1-free".to_string(),
                        name: "Laguna S 2.1 (Free)".to_string(),
                        effort_levels: vec![],
                    },
                    ModelInfo {
                        id: "hy3-free".to_string(),
                        name: "Hy3 (Free)".to_string(),
                        effort_levels: vec![],
                    },
                    ModelInfo {
                        id: "nemotron-3-ultra-free".to_string(),
                        name: "Nemotron 3 Ultra (Free)".to_string(),
                        effort_levels: vec![],
                    },
                    ModelInfo {
                        id: "mimo-v2.5-free".to_string(),
                        name: "MiMo V2.5 (Free)".to_string(),
                        effort_levels: vec![],
                    },
                    ModelInfo {
                        id: "big-pickle".to_string(),
                        name: "Big Pickle (Free)".to_string(),
                        effort_levels: vec![],
                    },
                ],
            },
            CliProfile::Pi => Self {
                description: "Pi Coding Agent model presets".to_string(),
                generated_at: "2026-08-20".to_string(),
                available_models: vec![
                    ModelInfo {
                        id: "gpt-5.4-mini".to_string(),
                        name: "GPT-5.4 Mini (Fast & Efficient)".to_string(),
                        effort_levels: vec!["low".to_string(), "medium".to_string(), "high".to_string()],
                    },
                    ModelInfo {
                        id: "gpt-5.4".to_string(),
                        name: "GPT-5.4 (Everyday Coding)".to_string(),
                        effort_levels: vec!["low".to_string(), "medium".to_string(), "high".to_string()],
                    },
                    ModelInfo {
                        id: "gpt-5.6-luna".to_string(),
                        name: "GPT-5.6 Luna (Fast & Affordable)".to_string(),
                        effort_levels: vec!["low".to_string(), "medium".to_string(), "high".to_string()],
                    },
                    ModelInfo {
                        id: "gpt-5.6-terra".to_string(),
                        name: "GPT-5.6 Terra (Balanced)".to_string(),
                        effort_levels: vec!["low".to_string(), "medium".to_string(), "high".to_string()],
                    },
                    ModelInfo {
                        id: "gpt-5.5".to_string(),
                        name: "GPT-5.5 (Frontier Complex Coding)".to_string(),
                        effort_levels: vec!["medium".to_string(), "high".to_string()],
                    },
                    ModelInfo {
                        id: "claude-sonnet-4-6".to_string(),
                        name: "Claude Sonnet 4.6 (Thinking)".to_string(),
                        effort_levels: vec!["high".to_string(), "max".to_string()],
                    },
                ],
            },
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub profile: CliProfile,
    #[serde(rename = "_metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ConfigMetadata>,
    pub mappings: Vec<GearMapping>,
}

impl ProfileConfig {
    pub fn default_for(profile: CliProfile) -> Self {
        let metadata = match profile {
            CliProfile::AgyCli
            | CliProfile::CodexCli
            | CliProfile::OpenCodeCli
            | CliProfile::Pi => {
                Some(ConfigMetadata::default_for(profile))
            }
            _ => None,
        };
        Self {
            profile,
            metadata,
            mappings: profile.default_mappings(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub test_mode: bool,
    pub preferred_terminal: String,
    #[serde(default)]
    pub theme_mode: crate::theme::ThemeMode,
    #[serde(skip)]
    pub profiles: HashMap<CliProfile, ProfileConfig>,
}

impl Default for Config {
    fn default() -> Self {
        let mut profiles = HashMap::new();
        for profile in CliProfile::all() {
            profiles.insert(*profile, ProfileConfig::default_for(*profile));
        }
        Self {
            test_mode: false,
            preferred_terminal: "Terminal".to_string(),
            theme_mode: crate::theme::ThemeMode::Auto,
            profiles,
        }
    }
}

impl Config {
    pub fn config_dir() -> PathBuf {
        if let Ok(dir) = std::env::var("HERDR_PLUGIN_CONFIG_DIR") {
            PathBuf::from(dir)
        } else if let Some(mut dir) = dirs::config_dir() {
            dir.push("realshifter");
            dir
        } else {
            PathBuf::from("/tmp/realshifter")
        }
    }

    pub fn config_path() -> PathBuf {
        Self::config_dir().join("config.json")
    }

    pub fn profiles_dir() -> PathBuf {
        Self::config_dir().join("profiles")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        let mut cfg = if let Ok(contents) = fs::read_to_string(&path) {
            serde_json::from_str::<Config>(&contents).unwrap_or_default()
        } else {
            Config::default()
        };

        let profiles_dir = Self::profiles_dir();
        let _ = fs::create_dir_all(&profiles_dir);

        let mut loaded_profiles = HashMap::new();
        for profile in CliProfile::all() {
            let p_path = profiles_dir.join(profile.file_name());
            let mut p_cfg = if let Ok(contents) = fs::read_to_string(&p_path) {
                match serde_json::from_str::<ProfileConfig>(&contents) {
                    Ok(c) => c,
                    Err(_) => ProfileConfig::default_for(*profile),
                }
            } else {
                ProfileConfig::default_for(*profile)
            };

            if *profile == CliProfile::AgyCli {
                for m in p_cfg.mappings.iter_mut() {
                    if m.action_type == GearActionType::AgyCli && m.command.trim() == "agy" {
                        m.command = String::new();
                    }
                }
            }

            loaded_profiles.insert(*profile, p_cfg);
        }

        cfg.profiles = loaded_profiles;
        let _ = cfg.save();
        cfg
    }

    pub fn save(&self) -> Result<(), String> {
        let dir = Self::config_dir();
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(Self::config_path(), json).map_err(|e| e.to_string())?;

        let profiles_dir = Self::profiles_dir();
        fs::create_dir_all(&profiles_dir).map_err(|e| e.to_string())?;

        for (profile, p_cfg) in &self.profiles {
            let p_path = profiles_dir.join(profile.file_name());
            let p_json = serde_json::to_string_pretty(p_cfg).map_err(|e| e.to_string())?;
            fs::write(p_path, p_json).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    pub fn get_mapping(&self, profile: CliProfile, gear: GearPosition) -> Option<GearMapping> {
        if let Some(p_cfg) = self.profiles.get(&profile) {
            p_cfg.mappings.iter().find(|m| m.gear == gear).cloned()
        } else {
            profile
                .default_mappings()
                .into_iter()
                .find(|m| m.gear == gear)
        }
    }

    pub fn update_mapping(&mut self, profile: CliProfile, new_mapping: GearMapping) {
        let p_cfg = self
            .profiles
            .entry(profile)
            .or_insert_with(|| ProfileConfig::default_for(profile));

        if let Some(existing) = p_cfg
            .mappings
            .iter_mut()
            .find(|m| m.gear == new_mapping.gear)
        {
            *existing = new_mapping;
        } else {
            p_cfg.mappings.push(new_mapping);
        }
    }

    pub fn available_models(&self, profile: CliProfile) -> &[ModelInfo] {
        if let Some(meta) = self
            .profiles
            .get(&profile)
            .and_then(|p| p.metadata.as_ref())
        {
            &meta.available_models
        } else if let Some(meta) = self
            .profiles
            .get(&CliProfile::AgyCli)
            .and_then(|p| p.metadata.as_ref())
        {
            &meta.available_models
        } else {
            &[]
        }
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
        cfg.test_mode = true;
        cfg.save().unwrap();

        let file_path = dir.join("config.json");
        assert!(file_path.exists());

        let agy_file = dir.join("profiles").join("agy.json");
        assert!(agy_file.exists());

        let agy_json = fs::read_to_string(&agy_file).unwrap();
        assert!(agy_json.contains("\"_metadata\""));
        assert!(agy_json.contains("\"available_models\""));

        let loaded = Config::load();
        assert!(loaded.test_mode);
        assert_eq!(loaded.profiles.len(), 6);

        let agy_p = loaded.profiles.get(&CliProfile::AgyCli).unwrap();
        let agy_meta = agy_p.metadata.as_ref().unwrap();
        assert_eq!(agy_meta.available_models.len(), 6);
        assert_eq!(agy_meta.available_models[0].id, "gemini-3.7-flash");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_config_update_mapping() {
        let mut cfg = Config::default();
        let new_m = GearMapping::new(
            GearPosition::Gear1,
            GearActionType::AgyCli,
            "",
            Some("gemini-3.7-flash-high"),
            "Gemini 3.7 Flash High",
            true,
        );
        cfg.update_mapping(CliProfile::AgyCli, new_m.clone());
        let fetched = cfg
            .get_mapping(CliProfile::AgyCli, GearPosition::Gear1)
            .unwrap();
        assert_eq!(fetched.label, "Gemini 3.7 Flash High");
        assert_eq!(fetched.model_flag.as_deref(), Some("gemini-3.7-flash-high"));
    }
}
