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
        assert_eq!(GearActionType::ClaudeCode.display_name(), "Claude Code");
        assert_eq!("claude".parse::<GearActionType>().unwrap(), GearActionType::ClaudeCode);
        assert_eq!("codex".parse::<GearActionType>().unwrap(), GearActionType::CodexCli);
        assert_eq!("opencode".parse::<GearActionType>().unwrap(), GearActionType::OpenCodeCli);
        assert_eq!("agy".parse::<GearActionType>().unwrap(), GearActionType::AgyCli);
        assert_eq!("undo".parse::<GearActionType>().unwrap(), GearActionType::Rollback);
    }
}
