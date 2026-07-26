use crate::CliProfile;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GearActionType {
    ClaudeCode,
    CodexCli,
    OpenCodeCli,
    AgyCli,
    CustomCommand,
    CustomHotkey,
    Rollback,
}

impl GearActionType {
    pub fn display_name(&self) -> &'static str {
        match self {
            GearActionType::ClaudeCode => "Claude Code",
            GearActionType::CodexCli => "Codex CLI",
            GearActionType::OpenCodeCli => "OpenCode CLI",
            GearActionType::AgyCli => "Antigravity CLI",
            GearActionType::CustomCommand => "Custom Command",
            GearActionType::CustomHotkey => "Custom Hotkey",
            GearActionType::Rollback => "Rollback / Undo",
        }
    }

    pub fn icon_symbol(&self) -> &'static str {
        match self {
            GearActionType::ClaudeCode => "🧠",
            GearActionType::CodexCli => "💻",
            GearActionType::OpenCodeCli => "⚡",
            GearActionType::AgyCli => "🛸",
            GearActionType::CustomCommand => "🛠️",
            GearActionType::CustomHotkey => "⌨️",
            GearActionType::Rollback => "↺",
        }
    }

    pub fn default_command(&self) -> &'static str {
        match self {
            GearActionType::ClaudeCode => "claude --model sonnet",
            GearActionType::CodexCli => "codex",
            GearActionType::OpenCodeCli => "opencode",
            GearActionType::AgyCli => "agy --model gemini-3.6-flash",
            GearActionType::CustomCommand => "echo 'Custom command'",
            GearActionType::CustomHotkey => "",
            GearActionType::Rollback => "/undo",
        }
    }

    pub fn all() -> &'static [GearActionType] {
        &[
            GearActionType::AgyCli,
            GearActionType::ClaudeCode,
            GearActionType::CodexCli,
            GearActionType::OpenCodeCli,
            GearActionType::CustomCommand,
            GearActionType::CustomHotkey,
            GearActionType::Rollback,
        ]
    }

    pub fn from_profile(profile: CliProfile) -> GearActionType {
        match profile {
            CliProfile::AgyCli => GearActionType::AgyCli,
            CliProfile::ClaudeCode => GearActionType::ClaudeCode,
            CliProfile::CodexCli => GearActionType::CodexCli,
            CliProfile::OpenCodeCli => GearActionType::OpenCodeCli,
            CliProfile::Custom => GearActionType::CustomCommand,
        }
    }
}

impl fmt::Display for GearActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl FromStr for GearActionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "claudecode" | "claude code" | "claude" => Ok(GearActionType::ClaudeCode),
            "codexcli" | "codex cli" | "codex" => Ok(GearActionType::CodexCli),
            "opencodecli" | "opencode cli" | "opencode" => Ok(GearActionType::OpenCodeCli),
            "agycli" | "agy cli" | "agy" | "antigravity" | "antigravity cli" => Ok(GearActionType::AgyCli),
            "customcommand" | "custom command" | "custom_command" => Ok(GearActionType::CustomCommand),
            "customhotkey" | "custom hotkey" | "custom_hotkey" => Ok(GearActionType::CustomHotkey),
            "rollback" | "rollback / undo" | "undo" => Ok(GearActionType::Rollback),
            _ => Err(format!("Unknown action type: '{s}'")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_type_display_and_parse() {
        let types = [
            GearActionType::ClaudeCode,
            GearActionType::CodexCli,
            GearActionType::OpenCodeCli,
            GearActionType::AgyCli,
            GearActionType::CustomCommand,
            GearActionType::CustomHotkey,
            GearActionType::Rollback,
        ];

        for t in types {
            assert!(!t.display_name().is_empty());
            assert!(!t.icon_symbol().is_empty());
            assert!(!format!("{t}").is_empty());
        }

        assert_eq!(GearActionType::ClaudeCode.default_command(), "claude --model sonnet");
        assert_eq!(GearActionType::CodexCli.default_command(), "codex");
        assert_eq!(GearActionType::OpenCodeCli.default_command(), "opencode");
        assert_eq!(GearActionType::AgyCli.default_command(), "agy --model gemini-3.6-flash");
        assert_eq!(GearActionType::CustomCommand.default_command(), "echo 'Custom command'");
        assert_eq!(GearActionType::CustomHotkey.default_command(), "");
        assert_eq!(GearActionType::Rollback.default_command(), "/undo");

        assert_eq!("claude".parse::<GearActionType>().unwrap(), GearActionType::ClaudeCode);
        assert_eq!("codex".parse::<GearActionType>().unwrap(), GearActionType::CodexCli);
        assert_eq!("opencode".parse::<GearActionType>().unwrap(), GearActionType::OpenCodeCli);
        assert_eq!("agy".parse::<GearActionType>().unwrap(), GearActionType::AgyCli);
        assert_eq!("custom command".parse::<GearActionType>().unwrap(), GearActionType::CustomCommand);
        assert_eq!("custom hotkey".parse::<GearActionType>().unwrap(), GearActionType::CustomHotkey);
        assert_eq!("undo".parse::<GearActionType>().unwrap(), GearActionType::Rollback);

        assert!("invalid_type".parse::<GearActionType>().is_err());
    }
}
