---
name: realshifter
description: Manage and sync RealShifter CLI gear configurations, available models, and shift actions across Antigravity (AGY), Claude Code, Codex CLI, OpenCode CLI, and Pi Coding Agent. Use when the user wants to sync available models, update gear mappings, or execute a gear shift.
---

# RealShifter

Manage and sync RealShifter hardware gear configurations, available LLM models, and shift actions.

## Information Architecture

Configurations live in modular profile files under `~/.config/realshifter/profiles/` (or `$HERDR_PLUGIN_CONFIG_DIR/profiles/`):
- `config.json` — Global settings (`test_mode`, `preferred_terminal`)
- `profiles/agy.json` — Antigravity (AGY) profile & model snapshot metadata
- `profiles/claude.json` — Claude Code profile
- `profiles/codex.json` — Codex CLI profile
- `profiles/opencode.json` — OpenCode CLI profile
- `profiles/pi.json` — Pi Coding Agent profile
- `profiles/custom.json` — Custom profile

---

## Workflow

### Step 1 — Model Discovery
Fetch active models depending on the CLI:

- **Antigravity (AGY)**: Run `agy models` to fetch active models and effort tiers.
- **OpenCode CLI**: Run `opencode models` to list available free and connected provider models.
- **Pi Coding Agent**: Run `pi --list-models` or inspect active provider catalogs.
- **Codex CLI**: Inspect active configuration and supported OpenAI coding models (e.g. `gpt-5.4-mini`, `gpt-5.4`, `gpt-5.6-luna`, `gpt-5.6-terra`, `gpt-5.5`).

- **Completion Criterion**: Live model list and supported effort levels/variants extracted.

### Step 2 — Profile Sync
Update the respective profile file under `~/.config/realshifter/profiles/<cli>.json` with the current ISO timestamp and snapshot:

#### For OpenCode (`profiles/opencode.json`):
```json
{
  "profile": "OpenCodeCli",
  "_metadata": {
    "description": "OpenCode free model presets",
    "generated_at": "<ISO-TIMESTAMP>",
    "available_models": [
      {
        "id": "nemotron-3.5-lightning-free",
        "name": "Nemotron 3.5 Lightning (Free)",
        "effort_levels": []
      },
      {
        "id": "deepseek-v4-flash-free",
        "name": "DeepSeek V4 Flash (Free)",
        "effort_levels": ["default", "low", "high", "max"]
      }
    ]
  },
  "mappings": [
    {
      "gear": "Gear1",
      "action_type": "OpenCodeCli",
      "command": "/models",
      "model_flag": "nemotron-3.5-lightning-free",
      "label": "Nemotron 3.5 Lightning (Free)",
      "is_enabled": true
    }
  ]
}
```

#### For AGY (`profiles/agy.json`):
```json
{
  "profile": "AgyCli",
  "_metadata": {
    "description": "RealShifter configuration snapshot of supported CLI models and effort levels.",
    "generated_at": "<ISO-TIMESTAMP>",
    "available_models": [
      {
        "id": "gemini-3.7-flash",
        "name": "Gemini 3.7 Flash",
        "effort_levels": ["low", "medium", "high"]
      }
    ]
  },
  "mappings": [
    {
      "gear": "Gear1",
      "action_type": "AgyCli",
      "command": "",
      "model_flag": "gemini-3.7-flash-low",
      "label": "Gemini 3.7 Flash (Low)",
      "is_enabled": true
    }
  ]
}
```

- **Completion Criterion**: Target profile JSON contains valid `_metadata` and updated gear 1–6 mappings.

### Step 3 — Execute Shift
Run the action binary to auto-detect the focused pane process and execute the gear shift:

```bash
./target/release/realshifter-action shift <1|2|3|4|5|6|reverse>
```

- **Completion Criterion**: Binary exits with code 0 and target CLI pane receives model switch command.
