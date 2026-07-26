# Handoff: RealShifter Herdr Plugin (CLI Version)

**Date**: 2026-07-27  
**Goal**: Build a Rust + ratatui Herdr Plugin for RealShifter in a dedicated repository.

---

## 1. Context & Key Architecture Decisions

- **Domain Context**: Summarized in [`CONTEXT.md`](file:///Users/ergenekonyigit/dev/RealShifter/CONTEXT.md). All 8 gear positions, 6 action types, and 4 CLI profiles translate 1:1 from Swift to Rust.
- **Architectural Decision (ADR-0002)**: Documented in [`docs/adr/0002-cli-version-as-herdr-plugin.md`](file:///Users/ergenekonyigit/dev/RealShifter/docs/adr/0002-cli-version-as-herdr-plugin.md).
- **Technology Stack**:
  - **Language**: Rust 2024 (Cargo workspace with 4 crates)
  - **Host Environment**: `herdr` terminal multiplexer plugin framework
  - **TUI Framework**: `ratatui` (overlay pane)
  - **Hardware Input**: macOS `IOKit` HID listener (daemon launched via one-shot `[[startup]]` with `--detach`) + keyboard shortcuts (`1-6`, `r`, `n`)
  - **Command Dispatch**: `HERDR_BIN_PATH pane send-keys` (with fallback to `tmux send-keys`)
  - **Persistence**: `HERDR_PLUGIN_CONFIG_DIR/config.json` and `HERDR_PLUGIN_STATE_DIR/state.json`

---

## 2. Specification & Tickets

- **Spec Document**: [`.scratch/realshifter-cli/spec.md`](file:///Users/ergenekonyigit/dev/RealShifter/.scratch/realshifter-cli/spec.md)
- **Implementation Tickets** (under `.scratch/realshifter-cli/issues/`):
  1. [`01-rust-workspace-setup.md`](file:///Users/ergenekonyigit/dev/RealShifter/.scratch/realshifter-cli/issues/01-rust-workspace-setup.md) — Cargo workspace layout & crate skeletons
  2. [`02-core-domain-models-and-json-config.md`](file:///Users/ergenekonyigit/dev/RealShifter/.scratch/realshifter-cli/issues/02-core-domain-models-and-json-config.md) — Rust domain models & JSON config/state persistence
  3. [`03-daemon-iokit-hid-listener.md`](file:///Users/ergenekonyigit/dev/RealShifter/.scratch/realshifter-cli/issues/03-daemon-iokit-hid-listener.md) — IOKit USB HID listener daemon
  4. [`04-action-executor-herdr-dispatch.md`](file:///Users/ergenekonyigit/dev/RealShifter/.scratch/realshifter-cli/issues/04-action-executor-herdr-dispatch.md) — Action executor & herdr `pane send-keys` dispatch
  5. [`05-ratatui-tui-dashboard.md`](file:///Users/ergenekonyigit/dev/RealShifter/.scratch/realshifter-cli/issues/05-ratatui-tui-dashboard.md) — `ratatui` dashboard overlay pane
  6. [`06-herdr-plugin-manifest-and-integration.md`](file:///Users/ergenekonyigit/dev/RealShifter/.scratch/realshifter-cli/issues/06-herdr-plugin-manifest-and-integration.md) — `herdr-plugin.toml` manifest & end-to-end integration

---

## 3. How to Execute in the New Session

1. Open a new session for the `realshifter-cli` repository (or this repository).
2. Reference this handoff file: `HANDOFF-REALSHIFTER-CLI.md`.
3. Pick up tickets starting with **Ticket 01** and invoke `/implement` for each ticket:
   ```bash
   /implement .scratch/realshifter-cli/issues/01-rust-workspace-setup.md
   ```
4. Each ticket drives `/tdd` internally and runs `/code-review` before committing.
