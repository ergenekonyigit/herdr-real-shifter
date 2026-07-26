#![allow(clippy::collapsible_if)]

use crate::{CliProfile, GearActionType};
use serde_json::Value;
use std::process::Command;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasoningEffort {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgyModelTarget {
    pub target_index: usize,
    pub effort: Option<ReasoningEffort>,
}

impl AgyModelTarget {
    pub fn parse(spec: &str) -> Self {
        let s = spec.trim().to_lowercase();
        let s = s.strip_prefix("/model").unwrap_or(&s).trim();

        match s {
            s if s.contains("3.6") && s.contains("low") => Self { target_index: 0, effort: Some(ReasoningEffort::Low) },
            s if s.contains("3.6") && s.contains("medium") => Self { target_index: 0, effort: Some(ReasoningEffort::Medium) },
            s if s.contains("3.6") => Self { target_index: 0, effort: Some(ReasoningEffort::High) },
            s if s.contains("3.5") && s.contains("low") => Self { target_index: 1, effort: Some(ReasoningEffort::Low) },
            s if s.contains("3.5") && s.contains("medium") => Self { target_index: 1, effort: Some(ReasoningEffort::Medium) },
            s if s.contains("3.5") => Self { target_index: 1, effort: Some(ReasoningEffort::High) },
            s if s.contains("3.1") && s.contains("low") => Self { target_index: 2, effort: Some(ReasoningEffort::Low) },
            s if s.contains("3.1") => Self { target_index: 2, effort: Some(ReasoningEffort::High) },
            s if s.contains("sonnet") => Self { target_index: 3, effort: None },
            s if s.contains("opus") => Self { target_index: 4, effort: None },
            s if s.contains("gpt-oss") || s.contains("120b") => Self { target_index: 5, effort: Some(ReasoningEffort::Medium) },
            _ => Self { target_index: 0, effort: None },
        }
    }

    pub fn index_from_text(text: &str) -> usize {
        let t = text.to_lowercase();
        if t.contains("3.6") { 0 }
        else if t.contains("3.5") { 1 }
        else if t.contains("3.1") { 2 }
        else if t.contains("sonnet") { 3 }
        else if t.contains("opus") { 4 }
        else if t.contains("120b") || t.contains("gpt-oss") { 5 }
        else { 0 }
    }

    pub fn calculate_down_steps(from_index: usize, to_index: usize, total_items: usize) -> usize {
        if to_index >= from_index {
            to_index - from_index
        } else {
            total_items - from_index + to_index
        }
    }
}

pub trait PaneDriver {
    fn resolve_target_pane(&self) -> String;
    fn detect_active_profile(&self, target_pane: &str) -> CliProfile;
    fn send_text(&self, target_pane: &str, text: &str) -> Result<(), String>;
    fn send_keys(&self, target_pane: &str, keys: &[&str]) -> Result<(), String>;
    fn send_interrupt(&self, target_pane: &str) -> Result<(), String>;
    fn read_pane_output(&self, target_pane: &str) -> Result<String, String>;
    fn sleep_ms(&self, ms: u64);
}

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
        if let Ok(pane_id) = std::env::var("HERDR_PANE_ID") {
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

        let panes = self.fetch_pane_list();
        for pane in &panes {
            if pane.get("focused").and_then(|f| f.as_bool()).unwrap_or(false) {
                if let Some(id) = pane.get("pane_id").and_then(|id| id.as_str()) {
                    return id.to_string();
                }
            }
        }

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
            let is_focused = pane.get("focused").and_then(|f| f.as_bool()).unwrap_or(false);

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
        for key in keys {
            let res = Command::new(&herdr_bin)
                .arg("pane")
                .arg("send-keys")
                .arg(target_pane)
                .arg(key)
                .status();

            if !matches!(res, Ok(st) if st.success()) {
                let _ = Command::new("tmux")
                    .arg("send-keys")
                    .arg("-t")
                    .arg(target_pane)
                    .arg(key);
            }
        }
        Ok(())
    }

    fn send_interrupt(&self, target_pane: &str) -> Result<(), String> {
        self.send_keys(target_pane, &["Ctrl+c"])
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

    fn sleep_ms(&self, ms: u64) {
        thread::sleep(Duration::from_millis(ms));
    }
}

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

    pub fn dispatch_action(&self, target_pane: &str, action_type: &GearActionType, command_str: &str) {
        match action_type {
            GearActionType::AgyCli => {
                self.select_agy_model(target_pane, command_str);
            }
            GearActionType::Rollback => {
                let _ = self.driver.send_interrupt(target_pane);
                self.driver.sleep_ms(300);
                let rollback_cmd = if command_str.is_empty() {
                    "/undo"
                } else {
                    command_str
                };
                self.send_command(target_pane, rollback_cmd);
            }
            _ => {
                if !command_str.is_empty() {
                    self.send_command(target_pane, command_str);
                }
            }
        }
    }

    pub fn send_command(&self, target_pane: &str, command: &str) {
        let clean_cmd = command.trim_end_matches('\n');
        if self.driver.send_text(target_pane, clean_cmd).is_err() {
            println!("[Simulated Command Dispatch to pane '{target_pane}']: {clean_cmd}");
            return;
        }
        let _ = self.driver.send_keys(target_pane, &["Enter"]);
    }

    pub fn select_agy_model(&self, target_pane: &str, model_spec: &str) {
        let target = AgyModelTarget::parse(model_spec);

        // Fallback: settings.json
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let settings_path = format!("{}/.gemini/antigravity-cli/settings.json", home);
        let settings_model = std::fs::read_to_string(&settings_path)
            .ok()
            .and_then(|s| {
                let v: serde_json::Value = serde_json::from_str(&s).ok()?;
                v.get("model").and_then(|m| m.as_str()).map(|m| m.to_lowercase())
            })
            .unwrap_or_default();

        let pane_output = self.driver.read_pane_output(target_pane).unwrap_or_default();
        let last_line = pane_output
            .lines()
            .rfind(|l| !l.trim().is_empty())
            .unwrap_or(&settings_model);

        let current_index = AgyModelTarget::index_from_text(last_line);
        let down_count = AgyModelTarget::calculate_down_steps(current_index, target.target_index, 6);

        // 1. Open /model modal
        let _ = self.driver.send_text(target_pane, "/model");
        let _ = self.driver.send_keys(target_pane, &["Enter"]);

        self.driver.sleep_ms(300);

        // 2. Move down to target item
        for _ in 0..down_count {
            let _ = self.driver.send_keys(target_pane, &["Down"]);
        }

        // 3. Adjust effort slider if applicable
        if let Some(effort) = target.effort {
            match effort {
                ReasoningEffort::Low => {
                    for _ in 0..3 {
                        let _ = self.driver.send_keys(target_pane, &["Left"]);
                    }
                }
                ReasoningEffort::Medium => {
                    for _ in 0..3 {
                        let _ = self.driver.send_keys(target_pane, &["Left"]);
                    }
                    let _ = self.driver.send_keys(target_pane, &["Right"]);
                }
                ReasoningEffort::High => {
                    for _ in 0..3 {
                        let _ = self.driver.send_keys(target_pane, &["Right"]);
                    }
                }
            }
        }

        // 4. Submit selection
        let _ = self.driver.send_keys(target_pane, &["Enter"]);
    }
}

#[derive(Default)]
pub struct MockPaneDriver {
    pub target_pane: String,
    pub profile: CliProfile,
    pub pane_output: String,
    pub recorded_texts: Mutex<Vec<(String, String)>>,
    pub recorded_keys: Mutex<Vec<(String, String)>>,
    pub recorded_interrupts: Mutex<Vec<String>>,
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
        assert_eq!(target.effort, Some(ReasoningEffort::Low));

        let down_steps = AgyModelTarget::calculate_down_steps(0, 1, 6);
        assert_eq!(down_steps, 1);
    }

    #[test]
    fn test_mock_driver_agy_model_select() {
        let mock = MockPaneDriver {
            target_pane: "pane-42".to_string(),
            profile: CliProfile::AgyCli,
            pane_output: "Gemini 3.6 Flash".to_string(),
            ..Default::default()
        };

        let service = PaneAutomationService::new(mock);
        service.select_agy_model("pane-42", "gemini-3.5-flash-low");

        let driver = service.driver();
        let texts = driver.recorded_texts.lock().unwrap();
        let keys = driver.recorded_keys.lock().unwrap();

        // /model text sent
        assert_eq!(texts.len(), 1);
        assert_eq!(texts[0], ("pane-42".to_string(), "/model".to_string()));

        // Keys sequence should open modal (Enter), move down 1 to 3.5, hit Left 3 times for low, hit Enter
        assert_eq!(keys[0].1, "Enter");
        assert_eq!(keys[1].1, "Down");
        assert_eq!(keys[2].1, "Left");
        assert_eq!(keys[3].1, "Left");
        assert_eq!(keys[4].1, "Left");
        assert_eq!(keys[5].1, "Enter");
    }

    #[test]
    fn test_mock_driver_rollback_dispatch() {
        let mock = MockPaneDriver::default();
        let service = PaneAutomationService::new(mock);

        service.dispatch_action("mock-pane-1", &GearActionType::Rollback, "");

        let driver = service.driver();
        let interrupts = driver.recorded_interrupts.lock().unwrap();
        let texts = driver.recorded_texts.lock().unwrap();
        let keys = driver.recorded_keys.lock().unwrap();

        assert_eq!(interrupts.len(), 1);
        assert_eq!(texts[0], ("mock-pane-1".to_string(), "/undo".to_string()));
        assert_eq!(keys[0], ("mock-pane-1".to_string(), "Enter".to_string()));
    }
}
