use crate::{GearActionType, GearMapping, GearPosition};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CliProfile {
    ClaudeCode,
    CodexCli,
    OpenCodeCli,
    AgyCli,
    Custom,
}

impl CliProfile {
    pub fn display_name(&self) -> &'static str {
        match self {
            CliProfile::ClaudeCode => "Claude Code",
            CliProfile::CodexCli => "Codex CLI",
            CliProfile::OpenCodeCli => "OpenCode CLI",
            CliProfile::AgyCli => "Antigravity (AGY)",
            CliProfile::Custom => "Custom / Multi-Tool",
        }
    }

    pub fn icon_symbol(&self) -> &'static str {
        match self {
            CliProfile::ClaudeCode => "🧠",
            CliProfile::CodexCli => "💻",
            CliProfile::OpenCodeCli => "⚡",
            CliProfile::AgyCli => "🛸",
            CliProfile::Custom => "🎛️",
        }
    }

    pub fn all() -> &'static [CliProfile] {
        &[
            CliProfile::ClaudeCode,
            CliProfile::CodexCli,
            CliProfile::OpenCodeCli,
            CliProfile::AgyCli,
            CliProfile::Custom,
        ]
    }

    pub fn default_mappings(&self) -> Vec<GearMapping> {
        match self {
            CliProfile::ClaudeCode => vec![
                GearMapping::new(
                    GearPosition::Gear1,
                    GearActionType::ClaudeCode,
                    "claude",
                    Some("sonnet"),
                    "Claude Sonnet",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear2,
                    GearActionType::ClaudeCode,
                    "claude",
                    Some("haiku"),
                    "Claude Haiku",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear3,
                    GearActionType::ClaudeCode,
                    "claude",
                    Some("opus"),
                    "Claude Opus",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear4,
                    GearActionType::ClaudeCode,
                    "claude",
                    Some("sonnet --thinking"),
                    "Claude Sonnet (Thinking)",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear5,
                    GearActionType::ClaudeCode,
                    "claude --compact",
                    None::<String>,
                    "Compact Context",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear6,
                    GearActionType::ClaudeCode,
                    "claude --resume",
                    None::<String>,
                    "Resume Session",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Reverse,
                    GearActionType::Rollback,
                    "/undo",
                    None::<String>,
                    "Rollback",
                    true,
                ),
            ],
            CliProfile::CodexCli => vec![
                GearMapping::new(
                    GearPosition::Gear1,
                    GearActionType::CodexCli,
                    "codex",
                    None::<String>,
                    "Codex Default",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear2,
                    GearActionType::CodexCli,
                    "codex --model gpt-4o",
                    None::<String>,
                    "GPT-4o",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear3,
                    GearActionType::CodexCli,
                    "codex --model gpt-4o-mini",
                    None::<String>,
                    "GPT-4o Mini",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear4,
                    GearActionType::CodexCli,
                    "codex --model o1",
                    None::<String>,
                    "o1 Reasoning",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear5,
                    GearActionType::CodexCli,
                    "codex --model o3-mini",
                    None::<String>,
                    "o3-mini Reasoning",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear6,
                    GearActionType::CodexCli,
                    "codex --full-auto",
                    None::<String>,
                    "Full Auto Mode",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Reverse,
                    GearActionType::Rollback,
                    "/undo",
                    None::<String>,
                    "Rollback",
                    true,
                ),
            ],
            CliProfile::OpenCodeCli => vec![
                GearMapping::new(
                    GearPosition::Gear1,
                    GearActionType::OpenCodeCli,
                    "opencode",
                    None::<String>,
                    "OpenCode Interactive",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear2,
                    GearActionType::OpenCodeCli,
                    "opencode run",
                    None::<String>,
                    "OpenCode Run",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear3,
                    GearActionType::OpenCodeCli,
                    "opencode review",
                    None::<String>,
                    "OpenCode Review",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear4,
                    GearActionType::OpenCodeCli,
                    "opencode debug",
                    None::<String>,
                    "OpenCode Debug",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear5,
                    GearActionType::OpenCodeCli,
                    "opencode test",
                    None::<String>,
                    "OpenCode Test",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear6,
                    GearActionType::OpenCodeCli,
                    "opencode agent",
                    None::<String>,
                    "OpenCode Agent",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Reverse,
                    GearActionType::Rollback,
                    "/undo",
                    None::<String>,
                    "Rollback",
                    true,
                ),
            ],
            CliProfile::AgyCli => vec![
                GearMapping::new(
                    GearPosition::Gear1,
                    GearActionType::AgyCli,
                    "agy",
                    Some("gemini-3.6-flash"),
                    "Gemini 3.6 Flash",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear2,
                    GearActionType::AgyCli,
                    "agy",
                    Some("gemini-3.6-pro"),
                    "Gemini 3.6 Pro",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear3,
                    GearActionType::AgyCli,
                    "agy",
                    Some("claude-3-7-sonnet"),
                    "Claude 3.7 Sonnet",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear4,
                    GearActionType::AgyCli,
                    "agy",
                    Some("gemini-2.5-flash"),
                    "Gemini 2.5 Flash",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear5,
                    GearActionType::AgyCli,
                    "agy --mode plan",
                    Some("gemini-3.6-pro"),
                    "Plan Mode (Pro)",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear6,
                    GearActionType::AgyCli,
                    "agy --mode accept-edits",
                    Some("gemini-3.6-flash"),
                    "Accept Edits Mode",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Reverse,
                    GearActionType::Rollback,
                    "/undo",
                    None::<String>,
                    "Rollback",
                    true,
                ),
            ],
            CliProfile::Custom => vec![
                GearMapping::new(
                    GearPosition::Gear1,
                    GearActionType::ClaudeCode,
                    "claude",
                    Some("sonnet"),
                    "Claude Sonnet",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear2,
                    GearActionType::AgyCli,
                    "agy",
                    Some("gemini-3.6-flash"),
                    "AGY Flash",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear3,
                    GearActionType::CodexCli,
                    "codex",
                    None::<String>,
                    "Codex CLI",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear4,
                    GearActionType::OpenCodeCli,
                    "opencode",
                    None::<String>,
                    "OpenCode CLI",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear5,
                    GearActionType::CustomCommand,
                    "echo 'Gear 5 custom'",
                    None::<String>,
                    "Custom CLI",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear6,
                    GearActionType::CustomHotkey,
                    "",
                    None::<String>,
                    "Custom Hotkey",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Reverse,
                    GearActionType::Rollback,
                    "/undo",
                    None::<String>,
                    "Rollback",
                    true,
                ),
            ],
        }
    }
}

impl fmt::Display for CliProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl FromStr for CliProfile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "claudecode" | "claude code" | "claude" => Ok(CliProfile::ClaudeCode),
            "codexcli" | "codex cli" | "codex" => Ok(CliProfile::CodexCli),
            "opencodecli" | "opencode cli" | "opencode" => Ok(CliProfile::OpenCodeCli),
            "agycli" | "agy cli" | "agy" | "antigravity" | "antigravity cli" => Ok(CliProfile::AgyCli),
            "custom" | "custom / multi-tool" | "multi-tool" => Ok(CliProfile::Custom),
            _ => Err(format!("Unknown CLI profile: '{s}'")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_defaults() {
        for profile in CliProfile::all() {
            let mappings = profile.default_mappings();
            assert_eq!(mappings.len(), 7, "Each profile should have 7 driving gear mappings");
        }
    }
}
