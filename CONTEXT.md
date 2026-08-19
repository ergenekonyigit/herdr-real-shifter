# Domain Model: RealShifter

RealShifter is a hardware-driven context and model switching engine for AI coding CLIs inside terminal workspace managers (primarily [Herdr](https://github.com/ergenekonyigit/herdr)).

## Core Concepts

### Shifter
The physical or emulated USB H-pattern controller. It exposes 7 standard HID gamepad buttons:
- Buttons 1–6: Forward drive gears (1st through 6th gear)
- Button 7: Reverse gear (stick pressed down and engaged in bottom-right slot)
- Released state: Neutral position

### Gear
A discrete transmission position representing an operational intent:
- `Gear1`–`Gear6`: Progressive model capabilities and reasoning effort tiers (e.g. fast/lightweight flash models in low gears up to frontier reasoning models in high gears).
- `Reverse`: Opens a new focused tab in Herdr and spawns a fresh session for the currently active AI CLI profile.
- `Neutral`: Idle state between gear shifts.

### Profile
A configuration domain tailored for a specific AI coding assistant CLI. Each profile specifies how gears map to model capabilities and commands:
- `AgyCli`: Antigravity CLI (Gemini 3.7 Flash Low/Med/High, 3.1 Pro, Claude Sonnet/Opus).
- `ClaudeCode`: Claude Code CLI (`/model haiku`, `sonnet`, `opus`, `sonnet --thinking`, `opus --thinking`, `fable`).
- `CodexCli`: OpenAI Codex CLI (`gpt-5.4-mini`, `gpt-5.4`, `gpt-5.6-luna`, `gpt-5.6-terra`, `gpt-5.5`).
- `OpenCodeCli`: OpenCode CLI (Nemotron, DeepSeek, Laguna, Hy3, MiMo).
- `Pi`: Pi Coding Agent.
- `Custom`: User-defined shell script or command actions.

### Gear Mapping
The immutable rule linking a physical `Gear` to a concrete action within a `Profile`. Contains:
- `gear`: The target gear position.
- `action_type`: The CLI or action execution strategy (`AgyCli`, `ClaudeCode`, `CodexCli`, `OpenCodeCli`, `Pi`, `CustomCommand`, `CustomHotkey`, `NewSession`).
- `command`: The specific CLI flag, slash command, or script to run.
- `model_flag`: Optional model identifier string.
- `label`: Human-readable display label shown in the TUI dashboard.
- `is_enabled`: Whether this gear position triggers an action.

### Pane Driver
The boundary module responsible for terminal communication. It resolves target terminal panes in the active workspace, dispatches text or keys via Herdr IPC, and spawns new tab sessions on Reverse gear.

### Daemon
The background USB HID event loop that connects to the shifter hardware, parses raw HID reports into `Gear` transitions, and triggers in-process action dispatching.
