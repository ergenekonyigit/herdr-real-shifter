---
name: realshifter
description: Manage and sync RealShifter CLI gear configurations, available models, and shift actions across Antigravity (AGY), Claude Code, Codex CLI, and OpenCode CLI. Use when the user wants to sync available models, update gear mappings, or execute a gear shift.
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
- `profiles/custom.json` — Custom profile

---

## Workflow

### Step 1 — Model Discovery
Run `agy models` to fetch active models and effort tiers.

- **Completion Criterion**: Live model list and supported effort levels extracted.

### Step 2 — Profile Sync
Update `~/.config/realshifter/profiles/agy.json` with the current ISO timestamp and snapshot:

```json
{
  "profile": "AgyCli",
  "_metadata": {
    "description": "RealShifter configuration snapshot of supported CLI models and effort levels.",
    "generated_at": "<ISO-TIMESTAMP>",
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

- **Completion Criterion**: `profiles/agy.json` contains valid `_metadata` and updated gear 1–6 mappings.

### Step 3 — Execute Shift
Run the action binary to auto-detect the focused pane process and execute the gear shift:

```bash
./target/release/realshifter-action shift <1|2|3|4|5|6|reverse>
```

- **Completion Criterion**: Binary exits with code 0 and target CLI pane receives model switch command.
