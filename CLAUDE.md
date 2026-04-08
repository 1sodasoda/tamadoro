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

Split across small modules under `src/`:

- **`main.rs`** — entry point, terminal setup, 500 ms poll loop, top-level key dispatch (Tab/BackTab, Space, `r`, Debug cheats `1`–`9`/`0`).
- **`app.rs`** — `App`, `Mode`, `PomodoroState`. `App::tick` advances the timer, decays food, handles death/level-up messaging. `toggle_pomo`/`reset_pomo` are input entry points. `get_pet_phrase` picks flavor speech from pools keyed by `PetType` + `PetMood`.
- **`game.rs`** — persistence layer. `GameData` (the save struct), `PetType`, `PetMood`, XP curve (`xp_for_level`), evolution stages, food decay (`update_food`), notifications. Save path is `dirs::data_local_dir()/tamadoro/save.json`. Newer fields use `#[serde(default)]` so old saves keep loading — preserve that pattern when adding fields.
- **`pets.rs`** — static ASCII art tables and `get_art` / `get_dead_art`. Adding a pet species means extending `PetType` in `game.rs` and adding art + lookup arms here.
- **`ui.rs`** — all ratatui rendering. `ui()` is the entry dispatcher; `render_timer` / `render_pet` / `render_stats` / `render_debug` handle each tab; `render_large_clock` draws the ASCII wall clock; `render_pet_with_speech` handles the speech-bubble layout.
- **`colors.rs`**, **`ascii_digits.rs`** — small data modules.

### Things to know before changing behavior

- Food decay and death are computed from real timestamps in the save, not tick counts — be careful with `last_food_check` / `hunger_zero_since` so pets don't accidentally starve (or revive) across restarts.
- State mutations should be followed by `game.save()` (see the debug handlers and `record_session` for the pattern). Autosave happens inside `tick` via `update_food` and in `record_session`; new code paths need to either call `save()` or funnel through those.
- `Mode::Debug` is gated on `--test` — the Tab/BackTab arms in `main.rs` hide it otherwise. Mirror that pattern for any other gated mode.
- Pet ASCII art is whitespace-sensitive — use raw string literals and don't let an editor trim trailing spaces.

### Things to know before changing behavior

- Food decay and death are computed from real timestamps stored in the save, not from tick counts — be careful when touching `last_food_check` / `hunger_zero_since` so you don't accidentally let pets starve (or revive) across restarts.
- Almost every state mutation must be followed by `app.game.save()` (see the debug handlers for the pattern). The autosave happens inside `tick`/`record_session`; new code paths need to either call `save()` or funnel through those.
- `Mode::Debug` and the `--test` flag are linked: hiding/showing the Debug tab is done by the Tab/BackTab arms in `main`, not in `ui`. If you add another gated mode, mirror that pattern.
- Pet ASCII art is whitespace-sensitive — use raw string literals and don't let an editor trim trailing spaces.
