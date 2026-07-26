# 🏎️ RealShifter

**RealShifter** is a hardware-driven context & model switching plugin designed for [Herdr](https://github.com/ergenekonyigit/herdr) and AI CLI power-users. It maps **USB H-pattern manual gear shifters** (such as Logitech Driving Force Shifter) and configurable hotkeys directly to model switches, effort tiers, and prompt actions across modern AI coding CLIs (AGY / Antigravity, Claude Code, Codex CLI, OpenCode CLI).

---

## 🌟 Key Features

- **🏎️ Hardware H-Pattern Shifting**: Map physical gears (Gear 1–6 & Reverse) on Logitech USB shifters directly to LLM model profiles and reasoning effort tiers.
- **⚡ Hotkey & Action Fallbacks**: Full keyboard shortcut support (`Ctrl+Shift+1..6`, `Ctrl+Shift+R`, `Ctrl+Shift+P`) via `realshifter-action`.
- **🖥️ Terminal UI (TUI) Dashboard**: Interactive Ratatui-based overlay dashboard for live gear visualization, device status monitoring, and profile mapping edits.
- **🔄 Multi-CLI Profile Management**: Seamlessly sync and shift profiles across:
  - **Antigravity (AGY)**
  - **Claude Code**
  - **Codex CLI**
  - **OpenCode CLI**
  - **Custom Shell Actions**
- **🔌 Herdr Plugin Architecture**: Deeply integrated into Herdr with lifecycle management, custom pane placement, keybindings, and event handlers.
- **🤖 AGY Skill Integration**: Built-in AGY skill (`realshifter`) for automated model discovery and profile synchronization.

---

## 🏗️ Architecture & Workspace Structure

RealShifter is built as a high-performance, safe Rust workspace containing modular crates:

```
herdr-real-shifter/
├── crates/
│   ├── realshifter-core/     # Core domain logic, gear mapping, state management, profiles
│   ├── realshifter-daemon/   # HID USB listener & background event loop daemon
│   ├── realshifter-tui/      # Ratatui TUI dashboard interface
│   └── realshifter-action/   # CLI action runner & IPC trigger binary
├── skills/
│   └── realshifter/          # AGY Skill definition for model discovery & auto-sync
└── herdr-plugin.toml         # Herdr plugin manifest declaration
```

### Modular Crates Breakdown

1. **`realshifter-core`**: Defines gear positions (`Gear1`..`Gear6`, `Reverse`, `Neutral`), profile data models, serialization handlers (`config.json`, modular profile JSONs), and theme definitions.
2. **`realshifter-daemon`**: Connects to USB HID devices (e.g. Logitech DFS via `hidapi`), monitors gear shifts in the background, and triggers corresponding profile switches or IPC events.
3. **`realshifter-tui`**: Interactive TUI interface rendered in Herdr overlay panes using `ratatui` and `crossterm`.
4. **`realshifter-action`**: Lightweight CLI tool invoked by Herdr keybindings or external scripts to execute gear actions (`shift 1..6`, `profile next`, `on-agent-complete`).

---

## ⚙️ Configuration & Profiles

Profiles live in `~/.config/realshifter/profiles/` (or `$HERDR_PLUGIN_CONFIG_DIR/profiles/`):

- `config.json` — Global settings (`test_mode`, `preferred_terminal`, active profile)
- `profiles/agy.json` — Antigravity (AGY) model mappings and effort tiers (`low`, `medium`, `high`)
- `profiles/claude.json` — Claude Code profile mappings
- `profiles/codex.json` — Codex CLI profile mappings
- `profiles/opencode.json` — OpenCode CLI profile mappings
- `profiles/custom.json` — User-defined custom command profiles

### Sample Gear Mapping Structure (`agy.json`)
```json
{
  "profile": "AgyCli",
  "_metadata": {
    "description": "RealShifter configuration snapshot of supported CLI models and effort levels.",
    "available_models": [
      {
        "id": "gemini-3.6-flash",
        "name": "Gemini 3.6 Flash",
        "effort_levels": ["low", "medium", "high"]
      }
    ]
  },
  "mappings": [
    {
      "gear": "Gear1",
      "action_type": "AgyCli",
      "command": "",
      "model": "gemini-3.6-flash-low",
      "label": "Gemini 3.6 Flash (Low)",
      "is_enabled": true
    }
  ]
}
```

---

## 🚀 Getting Started

### Prerequisites

- **OS**: macOS (native HID support)
- **Rust Toolchain**: 1.80+ (`cargo`, `rustc`)
- **Hardware**: Logitech Driving Force Shifter (or compatible USB HID shifter) *[Optional: Hotkeys available without hardware]*
- **Herdr**: `v0.7.0` or higher

### Build from Source

```bash
# Clone the repository
git clone https://github.com/ergenekonyigit/herdr-real-shifter.git
cd herdr-real-shifter

# Build release binaries for all workspace crates
cargo build --release
```

The compiled binaries will be placed in `./target/release/`:
- `realshifter-daemon`
- `realshifter-tui`
- `realshifter-action`

---

## 🎮 Usage

### 1. Registering with Herdr
When installed as a Herdr plugin, `herdr-plugin.toml` automatically:
- Launches `realshifter-daemon --detach` on startup.
- Registers the TUI dashboard overlay.
- Maps hotkeys `Ctrl+Shift+1..6`, `Ctrl+Shift+R`, `Ctrl+Shift+P`.

### 2. Manual Action Execution
You can manually run action commands via terminal:

```bash
# Shift to Gear 1 (e.g. Fast Flash Model)
./target/release/realshifter-action shift 1

# Shift to Reverse (e.g. Reset context / High Reasoning mode)
./target/release/realshifter-action shift reverse

# Switch active CLI Profile (AGY -> Claude -> Codex -> OpenCode)
./target/release/realshifter-action profile next
```

### 3. Interactive Dashboard (TUI)
Launch the standalone dashboard or press your Herdr dashboard pane hotkey:

```bash
./target/release/realshifter-tui
```

---

## 🛠️ AGY Skill Integration

RealShifter comes bundled with an **Antigravity (AGY)** skill located under `skills/realshifter/`.

AGY agents can invoke the `realshifter` skill to:
1. Discover active models and effort tiers via `agy models`.
2. Sync the snapshot directly into `~/.config/realshifter/profiles/agy.json`.
3. Trigger instant gear shifts to test model configurations.

---

## 📜 License

Distributed under the MIT License. See `LICENSE` for details.
