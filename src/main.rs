mod app;
mod ascii_digits;
mod colors;
mod game;
mod pets;
mod ui;

use std::{
    env,
    io::{self, stdout},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, Terminal};

use crate::app::{App, Mode};
use crate::game::{GameData, PetType};

fn main() -> io::Result<()> {
    // Check for --test flag
    let test_mode = env::args().any(|arg| arg == "--test");

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app = App::new(test_mode);
    let tick_rate = Duration::from_millis(500);

    loop {
        terminal.draw(|f| ui::ui(f, &app))?;

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Tab => {
                            app.mode = match app.mode {
                                Mode::Timer => Mode::Pet,
                                Mode::Pet => Mode::Stats,
                                Mode::Stats => {
                                    if app.test_mode {
                                        Mode::Debug
                                    } else {
                                        Mode::Timer
                                    }
                                }
                                Mode::Debug => Mode::Timer,
                            };
                        }
                        KeyCode::BackTab => {
                            app.mode = match app.mode {
                                Mode::Timer => {
                                    if app.test_mode {
                                        Mode::Debug
                                    } else {
                                        Mode::Stats
                                    }
                                }
                                Mode::Pet => Mode::Timer,
                                Mode::Stats => Mode::Pet,
                                Mode::Debug => Mode::Stats,
                            };
                        }
                        KeyCode::Char(' ') => {
                            app.toggle_pomo();
                        }
                        KeyCode::Char('r') if app.mode == Mode::Timer => {
                            app.reset_pomo();
                        }
                        // Debug controls
                        KeyCode::Char('1') if app.mode == Mode::Debug => {
                            app.game.add_xp(50);
                            app.game.save();
                            app.message = Some(("+50 XP".to_string(), Instant::now()));
                        }
                        KeyCode::Char('2') if app.mode == Mode::Debug => {
                            app.game.add_xp(500);
                            app.game.save();
                            app.message = Some(("+500 XP".to_string(), Instant::now()));
                        }
                        KeyCode::Char('3') if app.mode == Mode::Debug => {
                            app.game.xp = 0;
                            app.game.level += 1;
                            app.game.save();
                            app.message =
                                Some((format!("Level -> {}", app.game.level), Instant::now()));
                        }
                        KeyCode::Char('4') if app.mode == Mode::Debug => {
                            // Jump to next evolution stage
                            let next_level = match app.game.evolution_stage() {
                                1 => 5,
                                2 => 15,
                                3 => 30,
                                _ => app.game.level + 10,
                            };
                            app.game.level = next_level;
                            app.game.xp = 0;
                            app.game.save();
                            app.message = Some((
                                format!("Evolved to {}!", app.game.stage_name()),
                                Instant::now(),
                            ));
                        }
                        KeyCode::Char('5') if app.mode == Mode::Debug => {
                            app.game.streak_days = (app.game.streak_days + 1) % 15;
                            app.game.save();
                            app.message = Some((
                                format!("Streak -> {}", app.game.streak_days),
                                Instant::now(),
                            ));
                        }
                        KeyCode::Char('6') if app.mode == Mode::Debug => {
                            app.game.pet_type = match app.game.pet_type {
                                PetType::Blob => PetType::Cat,
                                PetType::Cat => PetType::Robot,
                                PetType::Robot => PetType::Ghost,
                                PetType::Ghost => PetType::Blob,
                            };
                            app.game.save();
                            app.message = Some((
                                format!("Pet -> {}", app.game.pet_type.name()),
                                Instant::now(),
                            ));
                        }
                        KeyCode::Char('7') if app.mode == Mode::Debug => {
                            app.game.feed(25);
                            app.game.save();
                            app.message =
                                Some((format!("Food -> {}", app.game.food), Instant::now()));
                        }
                        KeyCode::Char('8') if app.mode == Mode::Debug => {
                            app.game.food = app.game.food.saturating_sub(25);
                            app.game.save();
                            app.message =
                                Some((format!("Food -> {}", app.game.food), Instant::now()));
                        }
                        KeyCode::Char('9') if app.mode == Mode::Debug => {
                            app.game.is_dead = !app.game.is_dead;
                            if !app.game.is_dead {
                                app.game.food = 50; // Revive with some food
                                app.game.hunger_zero_since = None;
                            }
                            app.game.save();
                            app.message = Some((
                                if app.game.is_dead {
                                    "Pet died!".to_string()
                                } else {
                                    "Pet revived!".to_string()
                                },
                                Instant::now(),
                            ));
                        }
                        KeyCode::Char('0') if app.mode == Mode::Debug => {
                            app.game = GameData::default();
                            app.game.save();
                            app.pomo_sessions = 0;
                            app.message = Some(("Reset all data!".to_string(), Instant::now()));
                        }
                        _ => {}
                    }
                }
            }
        }

        app.tick();
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
