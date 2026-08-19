use crate::{GearActionType, GearMapping, GearPosition};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum CliProfile {
    ClaudeCode,
    CodexCli,
    OpenCodeCli,
    Pi,
    #[default]
    AgyCli,
    Custom,
}

impl CliProfile {
    pub fn display_name(&self) -> &'static str {
        match self {
            CliProfile::ClaudeCode => "Claude Code",
            CliProfile::CodexCli => "Codex CLI",
            CliProfile::OpenCodeCli => "OpenCode CLI",
            CliProfile::Pi => "Pi Agent",
            CliProfile::AgyCli => "Antigravity (AGY)",
            CliProfile::Custom => "Custom / Multi-Tool",
        }
    }

    pub fn icon_symbol(&self) -> &'static str {
        match self {
            CliProfile::ClaudeCode => "🧠",
            CliProfile::CodexCli => "💻",
            CliProfile::OpenCodeCli => "⚡",
            CliProfile::Pi => "π",
            CliProfile::AgyCli => "🛸",
            CliProfile::Custom => "🎛️",
        }
    }

    pub fn file_name(&self) -> &'static str {
        match self {
            CliProfile::ClaudeCode => "claude.json",
            CliProfile::CodexCli => "codex.json",
            CliProfile::OpenCodeCli => "opencode.json",
            CliProfile::Pi => "pi.json",
            CliProfile::AgyCli => "agy.json",
            CliProfile::Custom => "custom.json",
        }
    }

    pub fn all() -> &'static [CliProfile] {
        &[
            CliProfile::AgyCli,
            CliProfile::ClaudeCode,
            CliProfile::CodexCli,
            CliProfile::OpenCodeCli,
            CliProfile::Pi,
            CliProfile::Custom,
        ]
    }

    pub fn next(&self) -> Self {
        let all = Self::all();
        let idx = all.iter().position(|p| p == self).unwrap_or(0);
        all[(idx + 1) % all.len()]
    }

    pub fn prev(&self) -> Self {
        let all = Self::all();
        let idx = all.iter().position(|p| p == self).unwrap_or(0);
        if idx == 0 {
            all[all.len() - 1]
        } else {
            all[idx - 1]
        }
    }

    pub fn from_keyword(text: &str) -> Option<Self> {
        let t_low = text.to_lowercase();
        if t_low.contains("agy") || t_low.contains("antigravity") || t_low.contains("gemini") {
            Some(CliProfile::AgyCli)
        } else if t_low.contains("claude") {
            Some(CliProfile::ClaudeCode)
        } else if t_low.contains("codex") {
            Some(CliProfile::CodexCli)
        } else if t_low.contains("opencode") {
            Some(CliProfile::OpenCodeCli)
        } else if t_low.contains("pi") || t_low.contains("π") {
            Some(CliProfile::Pi)
        } else {
            None
        }
    }

    pub fn default_mappings(&self) -> Vec<GearMapping> {
        match self {
            CliProfile::ClaudeCode => vec![
                GearMapping::new(
                    GearPosition::Gear1,
                    GearActionType::ClaudeCode,
                    "/model haiku",
                    None::<String>,
                    "Haiku 4.5 (Fast)",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear2,
                    GearActionType::ClaudeCode,
                    "/model sonnet",
                    None::<String>,
                    "Sonnet 5 (Daily)",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear3,
                    GearActionType::ClaudeCode,
                    "/model opus",
                    None::<String>,
                    "Opus 5 (1M Context)",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear4,
                    GearActionType::ClaudeCode,
                    "/model sonnet --thinking",
                    None::<String>,
                    "Sonnet 5 (Thinking)",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear5,
                    GearActionType::ClaudeCode,
                    "/model opus --thinking",
                    None::<String>,
                    "Opus 5 (Thinking)",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear6,
                    GearActionType::ClaudeCode,
                    "/model fable",
                    None::<String>,
                    "Fable 5 (Frontier)",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Reverse,
                    GearActionType::NewSession,
                    "claude",
                    None::<String>,
                    "New Claude Session (Tab)",
                    true,
                ),
            ],
            CliProfile::CodexCli => vec![
                GearMapping::new(
                    GearPosition::Gear1,
                    GearActionType::CodexCli,
                    "/model gpt-5.4-mini",
                    None::<String>,
                    "GPT-5.4 Mini",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear2,
                    GearActionType::CodexCli,
                    "/model gpt-5.4",
                    None::<String>,
                    "GPT-5.4 Everyday",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear3,
                    GearActionType::CodexCli,
                    "/model gpt-5.6-luna",
                    None::<String>,
                    "GPT-5.6 Luna",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear4,
                    GearActionType::CodexCli,
                    "/model gpt-5.6-terra",
                    None::<String>,
                    "GPT-5.6 Terra",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear5,
                    GearActionType::CodexCli,
                    "/model gpt-5.5",
                    None::<String>,
                    "GPT-5.5 Frontier",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear6,
                    GearActionType::CodexCli,
                    "/model gpt-5.5-high",
                    None::<String>,
                    "GPT-5.5 (High Reasoning)",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Reverse,
                    GearActionType::NewSession,
                    "codex",
                    None::<String>,
                    "New Codex Session (Tab)",
                    true,
                ),
            ],
            CliProfile::OpenCodeCli => vec![
                GearMapping::new(
                    GearPosition::Gear1,
                    GearActionType::OpenCodeCli,
                    "/models",
                    Some("nemotron-3.5-lightning-free"),
                    "Nemotron 3.5 Lightning (Free)",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear2,
                    GearActionType::OpenCodeCli,
                    "/models",
                    Some("deepseek-v4-flash-free"),
                    "DeepSeek V4 Flash (Free)",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear3,
                    GearActionType::OpenCodeCli,
                    "/models",
                    Some("laguna-s-2.1-free"),
                    "Laguna S 2.1 (Free)",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear4,
                    GearActionType::OpenCodeCli,
                    "/models",
                    Some("hy3-free"),
                    "Hy3 (Free)",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear5,
                    GearActionType::OpenCodeCli,
                    "/models",
                    Some("nemotron-3-ultra-free"),
                    "Nemotron 3 Ultra (Free)",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear6,
                    GearActionType::OpenCodeCli,
                    "/models",
                    Some("mimo-v2.5-free"),
                    "MiMo V2.5 (Free)",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Reverse,
                    GearActionType::NewSession,
                    "opencode",
                    None::<String>,
                    "New OpenCode Session (Tab)",
                    true,
                ),
            ],
            CliProfile::AgyCli => vec![
                GearMapping::new(
                    GearPosition::Gear1,
                    GearActionType::AgyCli,
                    "",
                    Some("gemini-3.7-flash-low"),
                    "Gemini 3.7 Flash (Low)",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear2,
                    GearActionType::AgyCli,
                    "",
                    Some("gemini-3.7-flash-medium"),
                    "Gemini 3.7 Flash (Medium)",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear3,
                    GearActionType::AgyCli,
                    "",
                    Some("gemini-3.7-flash-high"),
                    "Gemini 3.7 Flash (High)",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear4,
                    GearActionType::AgyCli,
                    "",
                    Some("gemini-3.1-pro-high"),
                    "Gemini 3.1 Pro (High)",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear5,
                    GearActionType::AgyCli,
                    "",
                    Some("claude-sonnet-4-6"),
                    "Claude Sonnet 4.6 (Thinking)",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear6,
                    GearActionType::AgyCli,
                    "",
                    Some("claude-opus-4-6-thinking"),
                    "Claude Opus 4.6 (Thinking)",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Reverse,
                    GearActionType::NewSession,
                    "agy",
                    None::<String>,
                    "New AGY Session (Tab)",
                    true,
                ),
            ],
            CliProfile::Pi => vec![
                GearMapping::new(
                    GearPosition::Gear1,
                    GearActionType::Pi,
                    "/model",
                    Some("gpt-5.4-mini"),
                    "GPT-5.4 Mini",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear2,
                    GearActionType::Pi,
                    "/model",
                    Some("gpt-5.4"),
                    "GPT-5.4 Everyday",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear3,
                    GearActionType::Pi,
                    "/model",
                    Some("gpt-5.6-luna"),
                    "GPT-5.6 Luna",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear4,
                    GearActionType::Pi,
                    "/model",
                    Some("gpt-5.6-terra"),
                    "GPT-5.6 Terra",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear5,
                    GearActionType::Pi,
                    "/model",
                    Some("gpt-5.5"),
                    "GPT-5.5 Frontier",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Gear6,
                    GearActionType::Pi,
                    "/model",
                    Some("claude-sonnet-4-6"),
                    "Claude Sonnet 4.6",
                    true,
                ),
                GearMapping::new(
                    GearPosition::Reverse,
                    GearActionType::NewSession,
                    "pi",
                    None::<String>,
                    "New Pi Session (Tab)",
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
                    "",
                    Some("gemini-3.7-flash-high"),
                    "AGY Flash High",
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
                    GearActionType::NewSession,
                    "",
                    None::<String>,
                    "New Session (Tab)",
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
            "pi" | "piagent" | "pi agent" | "pi coding agent" | "π" => Ok(CliProfile::Pi),
            "agycli" | "agy cli" | "agy" | "antigravity" | "antigravity cli" => {
                Ok(CliProfile::AgyCli)
            }
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
            assert_eq!(
                mappings.len(),
                7,
                "Each profile should have 7 driving gear mappings"
            );
            assert!(!profile.display_name().is_empty());
            assert!(!profile.icon_symbol().is_empty());
            assert!(!profile.file_name().is_empty());
            assert!(!format!("{profile}").is_empty());
        }
    }

    #[test]
    fn test_profile_from_keyword_and_str() {
        assert_eq!(
            CliProfile::from_keyword("gemini 3.7"),
            Some(CliProfile::AgyCli)
        );
        assert_eq!(
            CliProfile::from_keyword("gemini 3.6"),
            Some(CliProfile::AgyCli)
        );
        assert_eq!(
            CliProfile::from_keyword("claude-code"),
            Some(CliProfile::ClaudeCode)
        );
        assert_eq!(
            CliProfile::from_keyword("codex"),
            Some(CliProfile::CodexCli)
        );
        assert_eq!(
            CliProfile::from_keyword("opencode"),
            Some(CliProfile::OpenCodeCli)
        );
        assert_eq!(
            CliProfile::from_keyword("pi - fleet"),
            Some(CliProfile::Pi)
        );
        assert_eq!(
            CliProfile::from_keyword("π"),
            Some(CliProfile::Pi)
        );
        assert_eq!(CliProfile::from_keyword("unknown"), None);

        assert_eq!(
            "claude".parse::<CliProfile>().unwrap(),
            CliProfile::ClaudeCode
        );
        assert_eq!("codex".parse::<CliProfile>().unwrap(), CliProfile::CodexCli);
        assert_eq!(
            "opencode".parse::<CliProfile>().unwrap(),
            CliProfile::OpenCodeCli
        );
        assert_eq!("pi".parse::<CliProfile>().unwrap(), CliProfile::Pi);
        assert_eq!("π".parse::<CliProfile>().unwrap(), CliProfile::Pi);
        assert_eq!("agy".parse::<CliProfile>().unwrap(), CliProfile::AgyCli);
        assert_eq!("custom".parse::<CliProfile>().unwrap(), CliProfile::Custom);
        assert!("invalid".parse::<CliProfile>().is_err());
    }
}
