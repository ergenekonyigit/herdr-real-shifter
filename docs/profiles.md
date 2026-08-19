# Profile Reference: Gear-to-Model Mappings

Each profile maps Gears 1–6 to progressively more capable models, and Reverse to a new Herdr tab session. Profiles are stored in `~/.config/realshifter/profiles/` as JSON files.

---

## Antigravity (AGY)

| Gear | Model | Effort | Description |
| :---: | :--- | :---: | :--- |
| **1** | `gemini-3.7-flash-low` | Low | Ultra-fast lightweight assistance & quick edits |
| **2** | `gemini-3.7-flash-medium` | Medium | Balanced everyday coding & refactoring |
| **3** | `gemini-3.7-flash-high` | High | Deep reasoning & multi-step debugging |
| **4** | `gemini-3.1-pro-high` | High | Complex architecture & large codebase reasoning |
| **5** | `claude-sonnet-4-6` | Thinking | Claude Sonnet reasoning |
| **6** | `claude-opus-4-6-thinking` | Thinking | Claude Opus flagship reasoning |
| **R** | New Tab Session | `agy` | Spawns a new focused AGY tab session in Herdr |

## OpenAI Codex CLI

| Gear | Model | Effort | Description |
| :---: | :--- | :---: | :--- |
| **1** | `gpt-5.4-mini` | Medium | Small, fast, and cost-efficient model |
| **2** | `gpt-5.4` | Medium | Strong model for everyday coding |
| **3** | `gpt-5.6-luna` | Medium | Fast & affordable agentic coding model |
| **4** | `gpt-5.6-terra` | Medium | Balanced agentic coding model for everyday work |
| **5** | `gpt-5.5` | Medium | Frontier: Complex coding, research & deep reasoning |
| **6** | `gpt-5.5-high` | High | Complex high-effort reasoning |
| **R** | New Tab Session | `codex` | Spawns a new focused Codex tab session in Herdr |

## Claude Code

| Gear | Model | Command | Description |
| :---: | :--- | :--- | :--- |
| **1** | Haiku 4.5 | `/model haiku` | Fastest model for quick answers ($1/$5 per Mtok) |
| **2** | Sonnet 5 | `/model sonnet` | Efficient daily coding & routine tasks ($2/$10 promo) |
| **3** | Opus 5 (1M) | `/model opus` | 1M context flagship for complex tasks ($5/$25 per Mtok) |
| **4** | Sonnet 5 (Thinking) | `/model sonnet --thinking` | Sonnet with extended reasoning budget |
| **5** | Opus 5 (Thinking) | `/model opus --thinking` | Opus 1M flagship with extended reasoning |
| **6** | Fable 5 | `/model fable` | Most capable for hardest & longest tasks ($10/$50 per Mtok) |
| **R** | New Tab Session | `claude` | Spawns a new focused Claude Code tab session in Herdr |

## OpenCode CLI (Free Models)

| Gear | Model | Variant | Description |
| :---: | :--- | :---: | :--- |
| **1** | `nemotron-3.5-lightning-free` | Default | Ultra-fast lightweight free assistance |
| **2** | `deepseek-v4-flash-free` | Default | Fast & intelligent free coding model |
| **3** | `laguna-s-2.1-free` | Default | Balanced everyday free coding & refactoring |
| **4** | `hy3-free` | Default | Alternative general-purpose free coding model |
| **5** | `nemotron-3-ultra-free` | Default | High-capacity reasoning & complex problem solving |
| **6** | `mimo-v2.5-free` | Default | Extended free reasoning & coding model |
| **R** | New Tab Session | `opencode` | Spawns a new focused OpenCode tab session in Herdr |

## Pi Coding Agent

| Gear | Model | Effort | Description |
| :---: | :--- | :---: | :--- |
| **1** | `gpt-5.4-mini` | Fast | Lightweight, fast & cost-efficient model |
| **2** | `gpt-5.4` | Medium | Strong everyday coding assistant |
| **3** | `gpt-5.6-luna` | Fast | Fast & capable agentic coding model |
| **4** | `gpt-5.6-terra` | Medium | Balanced multi-turn coding model |
| **5** | `gpt-5.5` | Frontier | Complex coding, research & deep reasoning |
| **6** | `claude-sonnet-4-6` | Thinking | Claude Sonnet reasoning & synthesis |
| **R** | New Tab Session | `pi` | Spawns a new focused Pi tab session in Herdr |
