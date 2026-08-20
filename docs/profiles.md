# Profile Reference: Gear-to-Model Mappings

RealShifter maps physical H-pattern shifter gears (`1`–`6` and `Reverse`) to progressively more capable AI frontier models and actions across your AI coding CLIs.

Profiles are stored as JSON files under `~/.config/realshifter/profiles/` (or `$HERDR_PLUGIN_CONFIG_DIR/profiles/`).

---

## Gear Progression Philosophy

```
  [1st] ──► [2nd] ──► [3rd] ──► [4th] ──► [5th] ──► [6th] ──► [Reverse]
  Turbo     Daily     Deep      Arch      Frontier  Max       New Tab
  Edits     Code      Debug     Design    Reasoning Flagship  Session
```

- **Gear 1 (Turbo / Fast)** — Ultra-low latency frontier models for instant inline edits, syntax checks, and rapid unit tests.
- **Gear 2 (Everyday Driver)** — Fast and balanced daily workhorses for routine feature development and refactoring.
- **Gear 3 (Deep Logic)** — High reasoning effort tiers for tricky bugs, edge cases, and complex multi-file logic.
- **Gear 4 (Architecture & Design)** — Large-context (1M+) multimodal models for repository-wide analysis and system design.
- **Gear 5 (Frontier Reasoning)** — Cutting-edge frontier reasoning models with extended thinking budgets for deep refactoring.
- **Gear 6 (Maximum Flagship)** — State-of-the-art flagship reasoning models for long-horizon autonomous tasks and critical architecture.
- **Reverse (R)** — Spawns a new focused Herdr tab with a fresh CLI session for the active profile.

---

## 1. Antigravity (AGY)

`~/.config/realshifter/profiles/agy.json`

| Gear | Model Identifier | Effort / Mode | Role & Description |
| :---: | :--- | :---: | :--- |
| **1** | `gemini-3.7-flash-low` | Low | Ultra-fast lightweight assistance & instant inline edits |
| **2** | `gemini-3.7-flash-medium` | Medium | Balanced everyday coding & routine refactoring |
| **3** | `gemini-3.7-flash-high` | High | Deep reasoning & multi-step debugging loop |
| **4** | `gemini-3.1-pro-high` | High | Large-context multimodal reasoning & codebase architecture |
| **5** | `claude-sonnet-5` | Extended Thinking | Claude Sonnet 5 hybrid reasoning & synthesis |
| **6** | `claude-opus-5-thinking` | Extended Thinking | Claude Opus 5 flagship reasoning & autonomous planning |
| **R** | *New Tab Session* | `agy` | Spawns a new focused AGY tab session in Herdr |

---

## 2. OpenAI Codex CLI

`~/.config/realshifter/profiles/codex.json`

| Gear | Model Identifier | Effort / Mode | Role & Description |
| :---: | :--- | :---: | :--- |
| **1** | `gpt-5.6-luna` | Fast | Fast & budget-friendly model for rapid iterations |
| **2** | `gpt-5.4` | Standard | Strong workhorse frontier model for everyday coding |
| **3** | `gpt-5.6-terra` | Agentic Loop | Balanced multi-turn agentic coding model |
| **4** | `gpt-5.5` | High Reasoning | Deep reasoning for complex coding & system architecture |
| **5** | `gpt-5.6-sol` | High Reasoning | Frontier flagship model for long-horizon coding & research |
| **6** | `gpt-5.6-sol-high` | Maximum Effort | Flagship reasoning model for long-horizon autonomous tasks |
| **R** | *New Tab Session* | `codex` | Spawns a new focused Codex tab session in Herdr |

---

## 3. Claude Code

`~/.config/realshifter/profiles/claude.json`

| Gear | Model Name | Command | Role & Description |
| :---: | :--- | :--- | :--- |
| **1** | Haiku 4.5 | `/model haiku` | Fastest model for quick completions & syntax checks |
| **2** | Sonnet 5 | `/model sonnet` | Efficient daily coding workhorse & routine development |
| **3** | Opus 5 (1M) | `/model opus` | 1M context flagship for complex repo-scale tasks |
| **4** | Sonnet 5 (Thinking) | `/model sonnet --thinking` | Extended reasoning budget for multi-step debugging |
| **5** | Opus 5 (Thinking) | `/model opus --thinking` | Flagship reasoning with deep architectural reflection |
| **6** | Fable 5 | `/model fable` | Frontier autonomous agent model for long-horizon work |
| **R** | *New Tab Session* | `claude` | Spawns a new focused Claude Code tab session in Herdr |

---

## 4. OpenCode CLI

`~/.config/realshifter/profiles/opencode.json`

| Gear | Model Identifier | Provider | Role & Description |
| :---: | :--- | :---: | :--- |
| **1** | `deepseek-v4-flash` | DeepSeek | High-speed 284B MoE frontier coding model for rapid completions |
| **2** | `qwen-3.8-27b` | Qwen / Alibaba | Dense 1M-context model for precise code generation & refactoring |
| **3** | `deepseek-v4-pro` | DeepSeek | Flagship 1.6T MoE frontier model with 1M context & advanced tool use |
| **4** | `qwen-3.8-max` | Qwen / Alibaba | Massive 2.4T parameter flagship coding & multi-file reasoning model |
| **5** | `grok-4.6` | xAI | Long-horizon agentic stability & deep multi-turn problem solving |
| **6** | `claude-opus-5` | Anthropic | Maximum reasoning flagship for complex repository-wide refactoring |
| **R** | *New Tab Session* | `opencode` | Spawns a new focused OpenCode tab session in Herdr |

---

## 5. Pi Coding Agent

`~/.config/realshifter/profiles/pi.json`

| Gear | Model Identifier | Effort / Mode | Role & Description |
| :---: | :--- | :---: | :--- |
| **1** | `gpt-5.6-luna` | Fast | Fast, agile & instant agentic completions |
| **2** | `gpt-5.4` | Medium | Strong everyday coding assistant |
| **3** | `deepseek-v4-pro` | Frontier | Flagship 1.6T MoE high-efficiency coding model |
| **4** | `gpt-5.6-terra` | Medium | Balanced multi-turn agentic task solving |
| **5** | `gpt-5.6-sol` | Frontier | Complex coding, research & deep reasoning |
| **6** | `claude-opus-5` | Maximum Thinking | Frontier flagship reasoning & cross-repo synthesis |
| **R** | *New Tab Session* | `pi` | Spawns a new focused Pi tab session in Herdr |

---

## 6. Custom / Multi-Tool Profile

`~/.config/realshifter/profiles/custom.json`

The Custom profile allows mixing and matching top frontier models, specialized agents, and automated build pipelines across your gears:

| Gear | Target Action | Command / Payload | Role & Description |
| :---: | :--- | :--- | :--- |
| **1** | Claude Code | `/model sonnet` | Claude Sonnet 5 daily coding |
| **2** | Antigravity (AGY) | `gemini-3.7-flash-high` | Gemini 3.7 Flash deep reasoning |
| **3** | OpenAI Codex | `gpt-5.6-sol` | GPT-5.6 Sol frontier reasoning agent |
| **4** | OpenCode CLI | `deepseek-v4-pro` | DeepSeek V4-Pro flagship coding engine |
| **5** | Claude Code (Thinking) | `/model opus --thinking` | Flagship Claude Opus 5 extended reasoning |
| **6** | Custom Pipeline | `cargo test && cargo clippy` | Project automated build, test & lint runner |
| **R** | *New Tab Session* | Custom Tab | Spawns a new focused session in Herdr |

---

## Profile JSON Schema Example

Each profile JSON file adheres to the following structure:

```json
{
  "profile": "AgyCli",
  "_metadata": {
    "description": "RealShifter configuration snapshot of supported CLI models and effort levels.",
    "generated_at": "2026-08-20T22:35:00+03:00",
    "available_models": [
      {
        "id": "gemini-3.7-flash",
        "name": "Gemini 3.7 Flash",
        "effort_levels": ["low", "medium", "high"]
      },
      {
        "id": "gemini-3.1-pro",
        "name": "Gemini 3.1 Pro",
        "effort_levels": ["low", "high"]
      },
      {
        "id": "claude-sonnet-5",
        "name": "Claude Sonnet 5 (Thinking)",
        "effort_levels": []
      },
      {
        "id": "claude-opus-5-thinking",
        "name": "Claude Opus 5 (Thinking)",
        "effort_levels": []
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
    },
    {
      "gear": "Gear2",
      "action_type": "AgyCli",
      "command": "",
      "model_flag": "gemini-3.7-flash-medium",
      "label": "Gemini 3.7 Flash (Medium)",
      "is_enabled": true
    },
    {
      "gear": "Gear3",
      "action_type": "AgyCli",
      "command": "",
      "model_flag": "gemini-3.7-flash-high",
      "label": "Gemini 3.7 Flash (High)",
      "is_enabled": true
    },
    {
      "gear": "Gear4",
      "action_type": "AgyCli",
      "command": "",
      "model_flag": "gemini-3.1-pro-high",
      "label": "Gemini 3.1 Pro (High)",
      "is_enabled": true
    },
    {
      "gear": "Gear5",
      "action_type": "AgyCli",
      "command": "",
      "model_flag": "claude-sonnet-5",
      "label": "Claude Sonnet 5 (Thinking)",
      "is_enabled": true
    },
    {
      "gear": "Gear6",
      "action_type": "AgyCli",
      "command": "",
      "model_flag": "claude-opus-5-thinking",
      "label": "Claude Opus 5 (Thinking)",
      "is_enabled": true
    },
    {
      "gear": "Reverse",
      "action_type": "NewSession",
      "command": "agy",
      "model_flag": null,
      "label": "New AGY Session (Tab)",
      "is_enabled": true
    }
  ]
}
```

---

## Switching Profiles

You can cycle between active CLI profiles at any time:

1. **Hotkey**: Press `Ctrl+Shift+P` in Herdr to cycle to the next profile.
2. **CLI**: Run `realshifter-action profile next` (or `realshifter-action profile <name>`).
3. **TUI Dashboard**: Press `p` in the `realshifter-tui` dashboard.
