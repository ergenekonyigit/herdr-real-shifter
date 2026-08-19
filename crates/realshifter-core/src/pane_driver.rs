use crate::{CliProfile, GearActionType, GearMapping, SessionState};
use serde_json::Value;
use std::process::Command;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ReasoningEffort {
    #[default]
    High,
    Medium,
    Low,
}

impl ReasoningEffort {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReasoningEffort::Low => "low",
            ReasoningEffort::Medium => "medium",
            ReasoningEffort::High => "high",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            ReasoningEffort::Low => "Low (Fast)",
            ReasoningEffort::Medium => "Medium (Balanced)",
            ReasoningEffort::High => "High (Deep Reasoning)",
        }
    }

    pub fn all() -> &'static [ReasoningEffort] {
        &[
            ReasoningEffort::Low,
            ReasoningEffort::Medium,
            ReasoningEffort::High,
        ]
    }

    pub fn next(&self) -> Self {
        match self {
            ReasoningEffort::Low => ReasoningEffort::Medium,
            ReasoningEffort::Medium => ReasoningEffort::High,
            ReasoningEffort::High => ReasoningEffort::Low,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            ReasoningEffort::Low => ReasoningEffort::High,
            ReasoningEffort::Medium => ReasoningEffort::Low,
            ReasoningEffort::High => ReasoningEffort::Medium,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "low" => Some(ReasoningEffort::Low),
            "medium" | "med" => Some(ReasoningEffort::Medium),
            "high" => Some(ReasoningEffort::High),
            _ => None,
        }
    }
}

impl std::fmt::Display for ReasoningEffort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgyModelTarget {
    pub target_index: usize,
    pub search_term: &'static str,
    pub effort: Option<ReasoningEffort>,
}

impl AgyModelTarget {
    pub fn parse(spec: &str) -> Self {
        let s = spec.trim().to_lowercase();
        let s = s.strip_prefix("/model").unwrap_or(&s).trim();

        match s {
            s if (s.contains("3.7") || s.contains("3.6")) && s.contains("low") => Self {
                target_index: 0,
                search_term: "3.7",
                effort: Some(ReasoningEffort::Low),
            },
            s if (s.contains("3.7") || s.contains("3.6")) && s.contains("medium") => Self {
                target_index: 0,
                search_term: "3.7",
                effort: Some(ReasoningEffort::Medium),
            },
            s if s.contains("3.7") || s.contains("3.6") => Self {
                target_index: 0,
                search_term: "3.7",
                effort: Some(ReasoningEffort::High),
            },
            s if s.contains("3.5") && s.contains("low") => Self {
                target_index: 1,
                search_term: "3.5",
                effort: Some(ReasoningEffort::Low),
            },
            s if s.contains("3.5") && s.contains("medium") => Self {
                target_index: 1,
                search_term: "3.5",
                effort: Some(ReasoningEffort::Medium),
            },
            s if s.contains("3.5") => Self {
                target_index: 1,
                search_term: "3.5",
                effort: Some(ReasoningEffort::High),
            },
            s if s.contains("3.1") && s.contains("low") => Self {
                target_index: 2,
                search_term: "3.1",
                effort: Some(ReasoningEffort::Low),
            },
            s if s.contains("3.1") => Self {
                target_index: 2,
                search_term: "3.1",
                effort: Some(ReasoningEffort::High),
            },
            s if s.contains("sonnet") => Self {
                target_index: 3,
                search_term: "sonnet",
                effort: None,
            },
            s if s.contains("opus") => Self {
                target_index: 4,
                search_term: "opus",
                effort: None,
            },
            s if s.contains("gpt-oss") || s.contains("120b") => Self {
                target_index: 5,
                search_term: "oss",
                effort: Some(ReasoningEffort::Medium),
            },
            _ => Self {
                target_index: 0,
                search_term: "3.7",
                effort: None,
            },
        }
    }

    pub fn index_from_text(text: &str) -> usize {
        let t = text.to_lowercase();
        if t.contains("3.7") || t.contains("3.6") {
            0
        } else if t.contains("3.5") {
            1
        } else if t.contains("3.1") {
            2
        } else if t.contains("sonnet") {
            3
        } else if t.contains("opus") {
            4
        } else if t.contains("120b") || t.contains("gpt-oss") {
            5
        } else {
            0
        }
    }

    pub fn calculate_down_steps(from_index: usize, to_index: usize, total_items: usize) -> usize {
        if to_index >= from_index {
            to_index - from_index
        } else {
            total_items - from_index + to_index
        }
    }
}

/// Boundary trait for interacting with terminal workspaces, panes, and tabs.
pub trait PaneDriver: Send + Sync {
    fn resolve_target_pane(&self) -> String;
    fn detect_active_profile(&self, target_pane: &str) -> CliProfile;
    fn send_text(&self, target_pane: &str, text: &str) -> Result<(), String>;
    fn send_keys(&self, target_pane: &str, keys: &[&str]) -> Result<(), String>;
    fn send_interrupt(&self, target_pane: &str) -> Result<(), String>;
    fn read_pane_output(&self, target_pane: &str) -> Result<String, String>;
    fn spawn_tab_session(&self, profile: CliProfile, custom_cmd: &str) -> Result<String, String>;
    fn sleep_ms(&self, ms: u64);
}

/// Strategy trait for CLI-specific automation logic.
pub trait CliAutomationAdapter: Send + Sync {
    fn dispatch(
        &self,
        driver: &dyn PaneDriver,
        target_pane: &str,
        command_str: &str,
        mapping: Option<&GearMapping>,
    );
}

// ---------------------------------------------------------------------------
// Concrete CLI Adapters
// ---------------------------------------------------------------------------

#[derive(Debug, Default, Clone, Copy)]
pub struct ClaudeAdapter;

impl CliAutomationAdapter for ClaudeAdapter {
    fn dispatch(
        &self,
        driver: &dyn PaneDriver,
        target_pane: &str,
        command_str: &str,
        _mapping: Option<&GearMapping>,
    ) {
        if command_str.starts_with("/model")
            || command_str.contains("sonnet")
            || command_str.contains("haiku")
            || command_str.contains("opus")
            || command_str.contains("fable")
        {
            let s = command_str.to_lowercase();
            let search_term = if s.contains("fable") {
                "fable"
            } else if s.contains("sonnet") && s.contains("thinking") {
                "sonnet --thinking"
            } else if s.contains("opus") && s.contains("thinking") {
                "opus --thinking"
            } else if s.contains("sonnet") {
                "sonnet"
            } else if s.contains("haiku") {
                "haiku"
            } else if s.contains("opus") {
                "opus"
            } else {
                s.strip_prefix("/models")
                    .or_else(|| s.strip_prefix("/model"))
                    .unwrap_or(&s)
                    .trim()
            };

            let cmd = if search_term.starts_with('/') {
                search_term.to_string()
            } else {
                format!("/model {search_term}")
            };
            send_terminal_command(driver, target_pane, &cmd);
        } else if !command_str.is_empty() {
            send_terminal_command(driver, target_pane, command_str);
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct AgyAdapter;

impl CliAutomationAdapter for AgyAdapter {
    fn dispatch(
        &self,
        driver: &dyn PaneDriver,
        target_pane: &str,
        command_str: &str,
        _mapping: Option<&GearMapping>,
    ) {
        let target = AgyModelTarget::parse(command_str);

        // 1. Open /model picker
        if driver.send_text(target_pane, "/model").is_err() {
            println!(
                "[Simulated Model Switch to pane '{target_pane}']: /model -> search '{}', effort {:?}",
                target.search_term, target.effort
            );
            return;
        }
        let _ = driver.send_keys(target_pane, &["enter"]);
        driver.sleep_ms(300);

        // 2. Type search filter to focus the exact model unambiguously
        let _ = driver.send_text(target_pane, target.search_term);
        driver.sleep_ms(100);

        // 3. Adjust reasoning effort slider if applicable
        if let Some(effort) = target.effort {
            match effort {
                ReasoningEffort::Low => {
                    let _ = driver.send_keys(target_pane, &["left", "left", "left", "left"]);
                }
                ReasoningEffort::Medium => {
                    let _ = driver.send_keys(target_pane, &["left", "left", "left", "left"]);
                    driver.sleep_ms(40);
                    let _ = driver.send_keys(target_pane, &["right"]);
                }
                ReasoningEffort::High => {
                    let _ = driver.send_keys(target_pane, &["right", "right", "right", "right"]);
                }
            }
            driver.sleep_ms(50);
        }

        // 4. Submit selection
        let _ = driver.send_keys(target_pane, &["enter"]);
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CodexAdapter;

impl CliAutomationAdapter for CodexAdapter {
    fn dispatch(
        &self,
        driver: &dyn PaneDriver,
        target_pane: &str,
        command_str: &str,
        _mapping: Option<&GearMapping>,
    ) {
        if command_str.starts_with("/model") || command_str.contains("gpt-5") {
            let s = command_str.to_lowercase();
            let option_key = if s.contains("5.4-mini") || s.contains("mini") {
                "5"
            } else if s.contains("5.4") {
                "4"
            } else if s.contains("5.6-luna") || s.contains("luna") {
                "2"
            } else if s.contains("5.6-terra") || s.contains("terra") {
                "1"
            } else if s.contains("5.5") {
                "3"
            } else {
                "1"
            };

            let _ = driver.send_keys(target_pane, &["esc"]);
            driver.sleep_ms(150);

            if driver.send_text(target_pane, "/model").is_err() {
                println!(
                    "[Simulated Codex Model Switch to pane '{target_pane}']: /model -> option {option_key}"
                );
                return;
            }
            driver.sleep_ms(100);
            let _ = driver.send_keys(target_pane, &["enter"]);
            driver.sleep_ms(600);

            let _ = driver.send_keys(target_pane, &[option_key]);
            driver.sleep_ms(400);

            let _ = driver.send_keys(target_pane, &["enter"]);
        } else if !command_str.is_empty() {
            send_terminal_command(driver, target_pane, command_str);
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct OpenCodeAdapter;

impl CliAutomationAdapter for OpenCodeAdapter {
    fn dispatch(
        &self,
        driver: &dyn PaneDriver,
        target_pane: &str,
        command_str: &str,
        _mapping: Option<&GearMapping>,
    ) {
        let s = command_str.to_lowercase();
        let search_term = if s.contains("deepseek") {
            "deepseek"
        } else if s.contains("nemotron") && (s.contains("3.5") || s.contains("lightning")) {
            "nemotron 3.5"
        } else if s.contains("nemotron") && (s.contains("ultra") || s.contains("3")) {
            "nemotron 3 ultra"
        } else if s.contains("laguna") {
            "laguna"
        } else if s.contains("hy3") {
            "hy3"
        } else if s.contains("mimo") {
            "mimo"
        } else if s.contains("pickle") {
            "big pickle"
        } else {
            s.strip_prefix("/models")
                .or_else(|| s.strip_prefix("/model"))
                .unwrap_or(&s)
                .trim()
        };

        let _ = driver.send_keys(target_pane, &["esc"]);
        driver.sleep_ms(150);

        if driver.send_text(target_pane, "/models").is_err() {
            println!(
                "[Simulated OpenCode Model Switch to pane '{target_pane}']: /models -> search '{search_term}'"
            );
            return;
        }
        driver.sleep_ms(100);
        let _ = driver.send_keys(target_pane, &["enter"]);
        driver.sleep_ms(300);

        let _ = driver.send_text(target_pane, search_term);
        driver.sleep_ms(100);
        let _ = driver.send_keys(target_pane, &["enter"]);
        driver.sleep_ms(200);

        let _ = driver.send_keys(target_pane, &["enter"]);
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PiAdapter;

impl CliAutomationAdapter for PiAdapter {
    fn dispatch(
        &self,
        driver: &dyn PaneDriver,
        target_pane: &str,
        command_str: &str,
        _mapping: Option<&GearMapping>,
    ) {
        let s = command_str.to_lowercase();
        let search_term = if s.contains("5.4-mini") || (s.contains("5.4") && s.contains("mini")) {
            "gpt-5.4-mini"
        } else if s.contains("5.4") {
            "gpt-5.4"
        } else if s.contains("5.6-luna") || s.contains("luna") {
            "gpt-5.6-luna"
        } else if s.contains("5.6-terra") || s.contains("terra") {
            "gpt-5.6-terra"
        } else if s.contains("5.5") {
            "gpt-5.5"
        } else if s.contains("sonnet") {
            "claude-sonnet"
        } else if s.contains("opus") {
            "claude-opus"
        } else {
            s.strip_prefix("/models")
                .or_else(|| s.strip_prefix("/model"))
                .unwrap_or(&s)
                .trim()
        };

        let _ = driver.send_keys(target_pane, &["esc"]);
        driver.sleep_ms(150);

        if driver.send_text(target_pane, "/model").is_err() {
            println!(
                "[Simulated Pi Model Switch to pane '{target_pane}']: /model -> search '{search_term}'"
            );
            return;
        }
        driver.sleep_ms(100);
        let _ = driver.send_keys(target_pane, &["enter"]);
        driver.sleep_ms(300);

        let _ = driver.send_text(target_pane, search_term);
        driver.sleep_ms(100);
        let _ = driver.send_keys(target_pane, &["enter"]);
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct NewSessionAdapter;

impl CliAutomationAdapter for NewSessionAdapter {
    fn dispatch(
        &self,
        driver: &dyn PaneDriver,
        _target_pane: &str,
        command_str: &str,
        _mapping: Option<&GearMapping>,
    ) {
        let state = SessionState::load();
        let profile = state.active_profile;
        let _ = driver.spawn_tab_session(profile, command_str);
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CustomShellAdapter;

impl CliAutomationAdapter for CustomShellAdapter {
    fn dispatch(
        &self,
        driver: &dyn PaneDriver,
        target_pane: &str,
        command_str: &str,
        _mapping: Option<&GearMapping>,
    ) {
        if !command_str.is_empty() {
            send_terminal_command(driver, target_pane, command_str);
        }
    }
}

fn send_terminal_command(driver: &dyn PaneDriver, target_pane: &str, command: &str) {
    let clean_cmd = command.trim_end_matches('\n');
    if driver.send_text(target_pane, clean_cmd).is_err() {
        println!("[Simulated Command Dispatch to pane '{target_pane}']: {clean_cmd}");
        return;
    }
    let _ = driver.send_keys(target_pane, &["enter"]);
}

// ---------------------------------------------------------------------------
// System Herdr Driver with 3-Tier Deterministic Resolution
// ---------------------------------------------------------------------------

pub struct SystemHerdrPaneDriver;

impl SystemHerdrPaneDriver {
    fn herdr_bin() -> String {
        std::env::var("HERDR_BIN_PATH").unwrap_or_else(|_| "herdr".to_string())
    }

    fn fetch_pane_list(&self) -> Vec<Value> {
        let herdr_bin = Self::herdr_bin();
        let output = match Command::new(&herdr_bin).arg("pane").arg("list").output() {
            Ok(out) if out.status.success() => out,
            _ => return Vec::new(),
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        serde_json::from_str::<Value>(&stdout)
            .ok()
            .and_then(|val| val.get("result")?.get("panes")?.as_array().cloned())
            .unwrap_or_default()
    }
}

impl PaneDriver for SystemHerdrPaneDriver {
    fn resolve_target_pane(&self) -> String {
        // Tier 1: Explicit Override or Pinned Pane
        if let Ok(pane_id) = std::env::var("HERDR_TARGET_PANE_ID") {
            let trimmed = pane_id.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }

        if let Ok(ctx_json) = std::env::var("HERDR_PLUGIN_CONTEXT_JSON") {
            if let Ok(val) = serde_json::from_str::<Value>(&ctx_json) {
                if let Some(pane_id) = val.get("active_pane_id").and_then(|v| v.as_str()) {
                    return pane_id.to_string();
                }
            }
        }

        let state = SessionState::load();
        if let Some(ref pinned) = state.pinned_pane_id {
            if !pinned.is_empty() {
                return pinned.clone();
            }
        }

        let panes = self.fetch_pane_list();
        let active_profile = state.active_profile;

        // Tier 2: Herdr Focused Active Pane
        for pane in &panes {
            if pane
                .get("focused")
                .and_then(|f| f.as_bool())
                .unwrap_or(false)
            {
                if let Some(id) = pane.get("pane_id").and_then(|i| i.as_str()) {
                    return id.to_string();
                }
            }
        }

        // Tier 3: Matching Agent CLI Pane
        for pane in &panes {
            let id = pane.get("pane_id").and_then(|i| i.as_str()).unwrap_or("");
            if self.detect_active_profile(id) == active_profile {
                return id.to_string();
            }
        }

        // Fallback: First available pane or "active"
        if let Some(first_pane) = panes.first() {
            if let Some(id) = first_pane.get("pane_id").and_then(|id| id.as_str()) {
                return id.to_string();
            }
        }

        "active".to_string()
    }

    fn detect_active_profile(&self, target_pane: &str) -> CliProfile {
        let panes = self.fetch_pane_list();
        for pane in &panes {
            let id = pane.get("pane_id").and_then(|i| i.as_str()).unwrap_or("");
            let is_focused = pane
                .get("focused")
                .and_then(|f| f.as_bool())
                .unwrap_or(false);

            if id == target_pane || (target_pane == "active" && is_focused) {
                if let Some(agent) = pane.get("agent").and_then(|a| a.as_str()) {
                    if let Some(profile) = CliProfile::from_keyword(agent) {
                        return profile;
                    }
                }
                if let Some(title) = pane.get("terminal_title").and_then(|t| t.as_str()) {
                    if let Some(profile) = CliProfile::from_keyword(title) {
                        return profile;
                    }
                }
            }
        }

        if let Ok(output) = self.read_pane_output(target_pane) {
            if let Some(profile) = CliProfile::from_keyword(&output) {
                return profile;
            }
        }

        CliProfile::AgyCli
    }

    fn send_text(&self, target_pane: &str, text: &str) -> Result<(), String> {
        let herdr_bin = Self::herdr_bin();
        let res = Command::new(&herdr_bin)
            .arg("pane")
            .arg("send-text")
            .arg(target_pane)
            .arg(text)
            .status();

        if matches!(res, Ok(st) if st.success()) {
            return Ok(());
        }

        let tmux_res = Command::new("tmux")
            .arg("send-keys")
            .arg("-t")
            .arg(target_pane)
            .arg(text)
            .status();

        if matches!(tmux_res, Ok(st) if st.success()) {
            return Ok(());
        }

        Err(format!("Failed to send text to pane {target_pane}"))
    }

    fn send_keys(&self, target_pane: &str, keys: &[&str]) -> Result<(), String> {
        let herdr_bin = Self::herdr_bin();
        let mut cmd = Command::new(&herdr_bin);
        cmd.arg("pane").arg("send-keys").arg(target_pane);
        for key in keys {
            cmd.arg(key);
        }
        let res = cmd.status();

        if matches!(res, Ok(st) if st.success()) {
            return Ok(());
        }

        for key in keys {
            let _ = Command::new("tmux")
                .arg("send-keys")
                .arg("-t")
                .arg(target_pane)
                .arg(key)
                .status();
        }
        Ok(())
    }

    fn send_interrupt(&self, target_pane: &str) -> Result<(), String> {
        self.send_keys(target_pane, &["ctrl+c"])
    }

    fn read_pane_output(&self, target_pane: &str) -> Result<String, String> {
        let herdr_bin = Self::herdr_bin();
        let output = Command::new(&herdr_bin)
            .arg("pane")
            .arg("read")
            .arg(target_pane)
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    fn spawn_tab_session(&self, profile: CliProfile, custom_cmd: &str) -> Result<String, String> {
        let herdr_bin = Self::herdr_bin();
        let label = profile.display_name();
        let cmd = if !custom_cmd.trim().is_empty() {
            custom_cmd.trim()
        } else {
            match profile {
                CliProfile::AgyCli => "agy",
                CliProfile::ClaudeCode => "claude",
                CliProfile::CodexCli => "codex",
                CliProfile::OpenCodeCli => "opencode",
                CliProfile::Pi => "pi",
                CliProfile::Custom => "",
            }
        };

        // 1. Try Herdr tab create
        let mut tab_cmd = Command::new(&herdr_bin);
        tab_cmd.arg("tab").arg("create").arg("--label").arg(label).arg("--focus");

        if let Ok(output) = tab_cmd.output() {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let target_pane_id = serde_json::from_str::<Value>(&stdout)
                    .ok()
                    .and_then(|v| {
                        v.get("result")?
                            .get("root_pane")?
                            .get("pane_id")?
                            .as_str()
                            .map(|s| s.to_string())
                    })
                    .unwrap_or_else(|| self.resolve_target_pane());

                if !cmd.is_empty() && !target_pane_id.is_empty() && target_pane_id != "active" {
                    let run_res = Command::new(&herdr_bin)
                        .arg("pane")
                        .arg("run")
                        .arg(&target_pane_id)
                        .arg(cmd)
                        .status();

                    if !matches!(run_res, Ok(st) if st.success()) {
                        thread::sleep(Duration::from_millis(300));
                        let _ = self.send_text(&target_pane_id, cmd);
                        let _ = self.send_keys(&target_pane_id, &["enter"]);
                    }
                }
                return Ok(stdout.to_string());
            }
        }

        // 2. Fallback to tmux
        if !cmd.is_empty() {
            let _ = Command::new("tmux")
                .arg("new-window")
                .arg("-n")
                .arg(label)
                .arg(cmd)
                .status();
        } else {
            let _ = Command::new("tmux")
                .arg("new-window")
                .arg("-n")
                .arg(label)
                .status();
        }

        Ok("spawned".to_string())
    }

    fn sleep_ms(&self, ms: u64) {
        thread::sleep(Duration::from_millis(ms));
    }
}

// ---------------------------------------------------------------------------
// Deep Automation Service Facade
// ---------------------------------------------------------------------------

pub struct PaneAutomationService<D: PaneDriver = SystemHerdrPaneDriver> {
    driver: D,
}

impl Default for PaneAutomationService<SystemHerdrPaneDriver> {
    fn default() -> Self {
        Self::new(SystemHerdrPaneDriver)
    }
}

impl<D: PaneDriver> PaneAutomationService<D> {
    pub fn new(driver: D) -> Self {
        Self { driver }
    }

    pub fn driver(&self) -> &D {
        &self.driver
    }

    pub fn resolve_target_pane(&self) -> String {
        self.driver.resolve_target_pane()
    }

    pub fn detect_active_profile(&self, target_pane: &str) -> CliProfile {
        self.driver.detect_active_profile(target_pane)
    }

    pub fn dispatch_action(
        &self,
        target_pane: &str,
        action_type: &GearActionType,
        command_str: &str,
    ) {
        match action_type {
            GearActionType::AgyCli => {
                AgyAdapter.dispatch(&self.driver, target_pane, command_str, None);
            }
            GearActionType::ClaudeCode => {
                ClaudeAdapter.dispatch(&self.driver, target_pane, command_str, None);
            }
            GearActionType::CodexCli => {
                CodexAdapter.dispatch(&self.driver, target_pane, command_str, None);
            }
            GearActionType::OpenCodeCli => {
                OpenCodeAdapter.dispatch(&self.driver, target_pane, command_str, None);
            }
            GearActionType::Pi => {
                PiAdapter.dispatch(&self.driver, target_pane, command_str, None);
            }
            GearActionType::NewSession => {
                NewSessionAdapter.dispatch(&self.driver, target_pane, command_str, None);
            }
            GearActionType::CustomCommand | GearActionType::CustomHotkey => {
                CustomShellAdapter.dispatch(&self.driver, target_pane, command_str, None);
            }
        }
    }

    pub fn send_command(&self, target_pane: &str, command: &str) {
        send_terminal_command(&self.driver, target_pane, command);
    }

    pub fn select_agy_model(&self, target_pane: &str, model_spec: &str) {
        AgyAdapter.dispatch(&self.driver, target_pane, model_spec, None);
    }

    pub fn select_claude_model(&self, target_pane: &str, model_spec: &str) {
        ClaudeAdapter.dispatch(&self.driver, target_pane, model_spec, None);
    }

    pub fn select_codex_model(&self, target_pane: &str, model_spec: &str) {
        CodexAdapter.dispatch(&self.driver, target_pane, model_spec, None);
    }

    pub fn select_opencode_model(&self, target_pane: &str, model_spec: &str) {
        OpenCodeAdapter.dispatch(&self.driver, target_pane, model_spec, None);
    }

    pub fn select_pi_model(&self, target_pane: &str, model_spec: &str) {
        PiAdapter.dispatch(&self.driver, target_pane, model_spec, None);
    }
}

// ---------------------------------------------------------------------------
// Mock Pane Driver for Unit Testing
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct MockPaneDriver {
    pub target_pane: String,
    pub profile: CliProfile,
    pub pane_output: String,
    pub recorded_texts: Mutex<Vec<(String, String)>>,
    pub recorded_keys: Mutex<Vec<(String, String)>>,
    pub recorded_interrupts: Mutex<Vec<String>>,
    pub spawned_tabs: Mutex<Vec<(CliProfile, String)>>,
    pub slept_ms: Mutex<Vec<u64>>,
}

impl PaneDriver for MockPaneDriver {
    fn resolve_target_pane(&self) -> String {
        if self.target_pane.is_empty() {
            "mock-pane-1".to_string()
        } else {
            self.target_pane.clone()
        }
    }

    fn detect_active_profile(&self, _target_pane: &str) -> CliProfile {
        self.profile
    }

    fn send_text(&self, target_pane: &str, text: &str) -> Result<(), String> {
        self.recorded_texts
            .lock()
            .unwrap()
            .push((target_pane.to_string(), text.to_string()));
        Ok(())
    }

    fn send_keys(&self, target_pane: &str, keys: &[&str]) -> Result<(), String> {
        let mut guard = self.recorded_keys.lock().unwrap();
        for k in keys {
            guard.push((target_pane.to_string(), k.to_string()));
        }
        Ok(())
    }

    fn send_interrupt(&self, target_pane: &str) -> Result<(), String> {
        self.recorded_interrupts
            .lock()
            .unwrap()
            .push(target_pane.to_string());
        Ok(())
    }

    fn read_pane_output(&self, _target_pane: &str) -> Result<String, String> {
        Ok(self.pane_output.clone())
    }

    fn spawn_tab_session(&self, profile: CliProfile, custom_cmd: &str) -> Result<String, String> {
        self.spawned_tabs
            .lock()
            .unwrap()
            .push((profile, custom_cmd.to_string()));
        Ok("mock-tab-ok".to_string())
    }

    fn sleep_ms(&self, ms: u64) {
        self.slept_ms.lock().unwrap().push(ms);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agy_model_target_parse() {
        let target = AgyModelTarget::parse("gemini-3.5-flash-low");
        assert_eq!(target.target_index, 1);
        assert_eq!(target.search_term, "3.5");
        assert_eq!(target.effort, Some(ReasoningEffort::Low));

        let down_steps = AgyModelTarget::calculate_down_steps(0, 1, 6);
        assert_eq!(down_steps, 1);
    }

    #[test]
    fn test_mock_driver_agy_model_select() {
        let mock = MockPaneDriver::default();
        let service = PaneAutomationService::new(mock);

        service.dispatch_action("pane-1", &GearActionType::AgyCli, "gemini-3.7-flash-high");
        let texts = service.driver().recorded_texts.lock().unwrap();
        assert!(texts.iter().any(|(_, t)| t == "/model"));
        assert!(texts.iter().any(|(_, t)| t == "3.7"));
    }

    #[test]
    fn test_mock_driver_claude_code_model_select() {
        let mock = MockPaneDriver::default();
        let service = PaneAutomationService::new(mock);

        service.dispatch_action("pane-1", &GearActionType::ClaudeCode, "/model sonnet");
        let texts = service.driver().recorded_texts.lock().unwrap();
        assert!(texts.iter().any(|(_, t)| t == "/model sonnet"));
    }

    #[test]
    fn test_mock_driver_codex_model_select() {
        let mock = MockPaneDriver::default();
        let service = PaneAutomationService::new(mock);

        service.dispatch_action("pane-1", &GearActionType::CodexCli, "gpt-5.4-mini");
        let texts = service.driver().recorded_texts.lock().unwrap();
        assert!(texts.iter().any(|(_, t)| t == "/model"));
    }

    #[test]
    fn test_mock_driver_opencode_model_select() {
        let mock = MockPaneDriver::default();
        let service = PaneAutomationService::new(mock);

        service.dispatch_action("pane-1", &GearActionType::OpenCodeCli, "deepseek-v4-flash-free");
        let texts = service.driver().recorded_texts.lock().unwrap();
        assert!(texts.iter().any(|(_, t)| t == "/models"));
        assert!(texts.iter().any(|(_, t)| t == "deepseek"));
    }

    #[test]
    fn test_mock_driver_pi_model_select() {
        let mock = MockPaneDriver::default();
        let service = PaneAutomationService::new(mock);

        service.dispatch_action("pane-1", &GearActionType::Pi, "gpt-5.6-terra");
        let texts = service.driver().recorded_texts.lock().unwrap();
        assert!(texts.iter().any(|(_, t)| t == "/model"));
        assert!(texts.iter().any(|(_, t)| t == "gpt-5.6-terra"));
    }

    #[test]
    fn test_mock_driver_new_session_dispatch() {
        let mock = MockPaneDriver {
            profile: CliProfile::ClaudeCode,
            ..Default::default()
        };
        let service = PaneAutomationService::new(mock);

        service.dispatch_action("pane-1", &GearActionType::NewSession, "claude");
        let spawned = service.driver().spawned_tabs.lock().unwrap();
        assert_eq!(spawned.len(), 1);
        assert_eq!(spawned[0].1, "claude");
    }

    #[test]
    fn test_mock_driver_custom_command() {
        let mock = MockPaneDriver::default();
        let service = PaneAutomationService::new(mock);

        service.dispatch_action("pane-1", &GearActionType::CustomCommand, "echo 'custom'");
        let texts2 = service.driver().recorded_texts.lock().unwrap();
        assert!(texts2.iter().any(|(_, t)| t == "echo 'custom'"));
    }
}
