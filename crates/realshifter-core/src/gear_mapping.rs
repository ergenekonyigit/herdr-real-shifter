use crate::{GearActionType, GearPosition};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GearMapping {
    pub gear: GearPosition,
    pub action_type: GearActionType,
    pub command: String,
    pub model_flag: Option<String>,
    pub label: String,
    pub is_enabled: bool,
}

impl GearMapping {
    pub fn new(
        gear: GearPosition,
        action_type: GearActionType,
        command: impl Into<String>,
        model_flag: Option<impl Into<String>>,
        label: impl Into<String>,
        is_enabled: bool,
    ) -> Self {
        Self {
            gear,
            action_type,
            command: command.into(),
            model_flag: model_flag.map(|s| s.into()),
            label: label.into(),
            is_enabled,
        }
    }

    pub fn display_label(&self) -> String {
        if !self.label.trim().is_empty() {
            self.label.clone()
        } else {
            self.action_type.display_name().to_string()
        }
    }

    pub fn effective_command(&self) -> String {
        match self.action_type {
            GearActionType::AgyCli => {
                let cmd_trimmed = self.command.trim();
                if let Some(ref flag) = self.model_flag {
                    let flag_trimmed = flag.trim();
                    if cmd_trimmed.is_empty() || cmd_trimmed == "/model" {
                        format!("/model {flag_trimmed}")
                    } else if !cmd_trimmed.contains("--model") {
                        format!("{cmd_trimmed} --model {flag_trimmed}")
                    } else {
                        cmd_trimmed.to_string()
                    }
                } else if !cmd_trimmed.is_empty() {
                    cmd_trimmed.to_string()
                } else {
                    self.action_type.default_command().to_string()
                }
            }
            GearActionType::ClaudeCode => {
                let base_cmd = if self.command.trim().is_empty() {
                    "claude"
                } else {
                    self.command.trim()
                };
                if let Some(ref flag) = self.model_flag {
                    let flag_trimmed = flag.trim();
                    if !flag_trimmed.is_empty() && !base_cmd.contains("--model") {
                        format!("{base_cmd} --model {flag_trimmed}")
                    } else {
                        base_cmd.to_string()
                    }
                } else {
                    base_cmd.to_string()
                }
            }
            GearActionType::CustomHotkey => self.command.trim().to_string(),
            _ => {
                if !self.command.trim().is_empty() {
                    self.command.trim().to_string()
                } else {
                    self.action_type.default_command().to_string()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effective_command() {
        let mapping = GearMapping::new(
            GearPosition::Gear1,
            GearActionType::ClaudeCode,
            "claude",
            Some("sonnet"),
            "Claude Sonnet",
            true,
        );
        assert_eq!(mapping.effective_command(), "claude --model sonnet");

        let agy_slash_mapping = GearMapping::new(
            GearPosition::Gear1,
            GearActionType::AgyCli,
            "",
            Some("gemini-3.6-flash-low"),
            "Gemini 3.6 Flash (Low)",
            true,
        );
        assert_eq!(agy_slash_mapping.effective_command(), "/model gemini-3.6-flash-low");

        let agy_cli_mapping = GearMapping::new(
            GearPosition::Gear1,
            GearActionType::AgyCli,
            "agy",
            Some("gemini-3.6-flash-low"),
            "Gemini 3.6 Flash (Low)",
            true,
        );
        assert_eq!(agy_cli_mapping.effective_command(), "agy --model gemini-3.6-flash-low");

        let custom_cmd = GearMapping::new(
            GearPosition::Gear5,
            GearActionType::CustomCommand,
            "echo hello",
            None::<String>,
            "Echo",
            true,
        );
        assert_eq!(custom_cmd.effective_command(), "echo hello");

        let default_codex = GearMapping::new(
            GearPosition::Gear1,
            GearActionType::CodexCli,
            "",
            None::<String>,
            "",
            true,
        );
        assert_eq!(default_codex.effective_command(), "codex");
    }
}
