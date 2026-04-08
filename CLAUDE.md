# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

Tamadoro is a single-binary terminal Pomodoro timer with a Tamagotchi-style pet, written in Rust on top of `ratatui` + `crossterm`. From the README: "vibe coded trash software because all the pomodoros ive used before felt bad".

## Commands

- Build / run: `cargo run` (release: `cargo run --release`)
- Run with debug tab enabled: `cargo run -- --test` (unlocks the Debug screen and its `0`–`9` cheat keys; without `--test` the Debug mode is hidden from the Tab cycle)
- Check / lint: `cargo check`, `cargo clippy`
- There is no test suite. `test_layout.rs` at the repo root is a stray scratch file, not wired into Cargo.

## Architecture

The entire app lives in `src/main.rs` (~1600 lines). It is intentionally one file; don't split it up unless asked. Key pieces, top-to-bottom:

- **`PetType` (enum)** — Blob / Cat / Robot / Ghost. Each variant owns its ASCII art and the per-evolution-stage frames. Adding a pet means extending this enum and its `impl` (art, name, color).
- **`Mode` / `PomodoroState` / `PetMood`** — UI tab + timer state machines. `Mode::Debug` is only reachable when launched with `--test`.
- **`GameData`** — the persisted save: XP/level, streak, today's sessions, pet identity, food (0–100), `hunger_zero_since`, `is_dead`. Serialized as JSON to `dirs::data_local_dir()/tamadoro/save.json`. Newer fields use `#[serde(default)]` so old saves keep loading — preserve that pattern when adding fields. Leveling curve is `100 * 1.5^(level-1)` via `xp_for_level`. Evolution stages are derived from `level` (`evolution_stage` / `stage_name`).
- **`App`** — runtime state that wraps `GameData` plus transient stuff (current `Mode`, pomo timer, ephemeral `message` toast, `test_mode` flag). `App::tick` advances the timer, decays food over wall-clock time, handles death, and triggers saves; `toggle_pomo` / `reset_pomo` are the input entry points.
- **`main`** — sets up the alternate screen + raw mode, then a 250 ms poll loop: draw → handle one key event → `app.tick()`. All keybindings live here in one big `match`. Tab/BackTab cycles modes; Space toggles the timer; `r` resets in Timer mode; `1`–`9`,`0` are debug cheats gated on `Mode::Debug`.
- **`ui` and the `render_*` helpers** — pure ratatui rendering. `render_large_clock` draws the ASCII wall clock; `render_pet_with_speech` / `render_pet` pick frames based on `evolution_stage`, `mood`, and `is_dead`; `render_timer`, `render_stats`, `render_debug` handle the other tabs.

### Things to know before changing behavior

- Food decay and death are computed from real timestamps stored in the save, not from tick counts — be careful when touching `last_food_check` / `hunger_zero_since` so you don't accidentally let pets starve (or revive) across restarts.
- Almost every state mutation must be followed by `app.game.save()` (see the debug handlers for the pattern). The autosave happens inside `tick`/`record_session`; new code paths need to either call `save()` or funnel through those.
- `Mode::Debug` and the `--test` flag are linked: hiding/showing the Debug tab is done by the Tab/BackTab arms in `main`, not in `ui`. If you add another gated mode, mirror that pattern.
- Pet ASCII art is whitespace-sensitive — use raw string literals and don't let an editor trim trailing spaces.
