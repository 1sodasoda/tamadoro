use std::{
    env, fs,
    io::{self, stdout},
    path::PathBuf,
    time::{Duration, Instant},
};

use chrono::{Local, NaiveDate};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use git2::Repository;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
};
use serde::{Deserialize, Serialize};

// Tokyonight Night colors
mod colors {
    use ratatui::style::Color;
    pub const BG: Color = Color::Rgb(26, 27, 38);
    pub const FG: Color = Color::Rgb(169, 177, 214);
    pub const RED: Color = Color::Rgb(247, 118, 142);
    pub const GREEN: Color = Color::Rgb(158, 206, 106);
    pub const YELLOW: Color = Color::Rgb(224, 175, 104);
    pub const BLUE: Color = Color::Rgb(122, 162, 247);
    pub const MAGENTA: Color = Color::Rgb(187, 154, 247);
    pub const CYAN: Color = Color::Rgb(125, 207, 255);
    pub const COMMENT: Color = Color::Rgb(86, 95, 137);
}

#[derive(PartialEq, Clone, Copy)]
enum Mode {
    Pomodoro,
    Git,
    Stats,
}

#[derive(PartialEq, Clone, Copy)]
enum PomodoroState {
    Work,
    Break,
    Paused,
}

#[derive(Serialize, Deserialize, Default)]
struct Stats {
    total_sessions: u32,
    total_focus_mins: u32,
    last_session_date: Option<NaiveDate>,
    streak_days: u32,
    today_sessions: u32,
    today_focus_mins: u32,
    today_date: Option<NaiveDate>,
}

impl Stats {
    fn data_path() -> PathBuf {
        let data_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("gitomo");
        fs::create_dir_all(&data_dir).ok();
        data_dir.join("stats.json")
    }

    fn load() -> Self {
        let path = Self::data_path();
        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(mut stats) = serde_json::from_str::<Stats>(&data) {
                // Reset today's stats if it's a new day
                let today = Local::now().date_naive();
                if stats.today_date != Some(today) {
                    stats.today_sessions = 0;
                    stats.today_focus_mins = 0;
                    stats.today_date = Some(today);
                }
                return stats;
            }
        }
        Stats {
            today_date: Some(Local::now().date_naive()),
            ..Default::default()
        }
    }

    fn save(&self) {
        let path = Self::data_path();
        if let Ok(data) = serde_json::to_string_pretty(self) {
            fs::write(path, data).ok();
        }
    }

    fn record_session(&mut self) {
        let today = Local::now().date_naive();

        // Update streak
        if let Some(last_date) = self.last_session_date {
            let days_diff = (today - last_date).num_days();
            if days_diff == 1 {
                self.streak_days += 1;
            } else if days_diff > 1 {
                self.streak_days = 1;
            }
            // If same day, streak stays the same
        } else {
            self.streak_days = 1;
        }

        self.last_session_date = Some(today);
        self.total_sessions += 1;
        self.total_focus_mins += 25;

        // Update today's stats
        if self.today_date != Some(today) {
            self.today_sessions = 0;
            self.today_focus_mins = 0;
            self.today_date = Some(today);
        }
        self.today_sessions += 1;
        self.today_focus_mins += 25;

        self.save();
    }
}

struct App {
    mode: Mode,
    // Pomodoro
    pomo_state: PomodoroState,
    pomo_remaining: Duration,
    pomo_total: Duration,
    pomo_sessions: u32,
    last_tick: Instant,
    // Git
    git_repo_path: String,
    git_branch: String,
    git_staged: Vec<String>,
    git_modified: Vec<String>,
    git_untracked: Vec<String>,
    // Stats
    stats: Stats,
}

impl App {
    fn new() -> Self {
        let work_duration = Duration::from_secs(25 * 60);
        let repo_path = env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| ".".to_string());

        let mut app = App {
            mode: Mode::Pomodoro,
            pomo_state: PomodoroState::Paused,
            pomo_remaining: work_duration,
            pomo_total: work_duration,
            pomo_sessions: 0,
            last_tick: Instant::now(),
            git_repo_path: repo_path,
            git_branch: String::new(),
            git_staged: Vec::new(),
            git_modified: Vec::new(),
            git_untracked: Vec::new(),
            stats: Stats::load(),
        };
        app.refresh_git();
        app
    }

    fn tick(&mut self) {
        if self.pomo_state == PomodoroState::Work || self.pomo_state == PomodoroState::Break {
            let elapsed = self.last_tick.elapsed();
            self.last_tick = Instant::now();

            if self.pomo_remaining > elapsed {
                self.pomo_remaining -= elapsed;
            } else {
                // Timer finished
                if self.pomo_state == PomodoroState::Work {
                    self.pomo_sessions += 1;
                    self.stats.record_session();
                    self.pomo_state = PomodoroState::Break;
                    self.pomo_total = Duration::from_secs(5 * 60);
                    self.pomo_remaining = self.pomo_total;
                } else {
                    self.pomo_state = PomodoroState::Work;
                    self.pomo_total = Duration::from_secs(25 * 60);
                    self.pomo_remaining = self.pomo_total;
                }
            }
        }
    }

    fn toggle_pomo(&mut self) {
        match self.pomo_state {
            PomodoroState::Paused => {
                self.pomo_state = PomodoroState::Work;
                self.last_tick = Instant::now();
            }
            _ => {
                self.pomo_state = PomodoroState::Paused;
            }
        }
    }

    fn reset_pomo(&mut self) {
        self.pomo_state = PomodoroState::Paused;
        self.pomo_total = Duration::from_secs(25 * 60);
        self.pomo_remaining = self.pomo_total;
    }

    fn refresh_git(&mut self) {
        self.git_staged.clear();
        self.git_modified.clear();
        self.git_untracked.clear();
        self.git_branch = String::from("not a repo");

        if let Ok(repo) = Repository::discover(&self.git_repo_path) {
            // Get branch
            if let Ok(head) = repo.head() {
                if let Some(name) = head.shorthand() {
                    self.git_branch = name.to_string();
                }
            }

            // Get status
            if let Ok(statuses) = repo.statuses(None) {
                for entry in statuses.iter() {
                    let path = entry.path().unwrap_or("?").to_string();
                    let status = entry.status();

                    if status.is_index_new()
                        || status.is_index_modified()
                        || status.is_index_deleted()
                    {
                        self.git_staged.push(path.clone());
                    }
                    if status.is_wt_modified() || status.is_wt_deleted() {
                        self.git_modified.push(path.clone());
                    }
                    if status.is_wt_new() {
                        self.git_untracked.push(path);
                    }
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app = App::new();
    let tick_rate = Duration::from_millis(200);

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Tab => {
                            app.mode = match app.mode {
                                Mode::Pomodoro => Mode::Git,
                                Mode::Git => Mode::Stats,
                                Mode::Stats => Mode::Pomodoro,
                            };
                            if app.mode == Mode::Git {
                                app.refresh_git();
                            }
                        }
                        KeyCode::Char(' ') if app.mode == Mode::Pomodoro => {
                            app.toggle_pomo();
                        }
                        KeyCode::Char('r') => {
                            if app.mode == Mode::Pomodoro {
                                app.reset_pomo();
                            } else if app.mode == Mode::Git {
                                app.refresh_git();
                            }
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

fn ui(f: &mut Frame, app: &App) {
    let area = f.area();

    // Main block
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(match app.mode {
            Mode::Pomodoro => colors::MAGENTA,
            Mode::Git => colors::CYAN,
            Mode::Stats => colors::GREEN,
        }))
        .style(Style::default().bg(colors::BG));

    f.render_widget(block, area);

    let inner = area.inner(Margin {
        horizontal: 2,
        vertical: 1,
    });

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Mode tabs
            Constraint::Length(1), // Spacer
            Constraint::Min(0),    // Content
            Constraint::Length(2), // Help
        ])
        .split(inner);

    // Mode tabs
    let tabs = Line::from(vec![
        Span::styled(
            " POMO ",
            if app.mode == Mode::Pomodoro {
                Style::default().fg(colors::BG).bg(colors::MAGENTA).bold()
            } else {
                Style::default().fg(colors::COMMENT)
            },
        ),
        Span::raw(" "),
        Span::styled(
            " GIT ",
            if app.mode == Mode::Git {
                Style::default().fg(colors::BG).bg(colors::CYAN).bold()
            } else {
                Style::default().fg(colors::COMMENT)
            },
        ),
        Span::raw(" "),
        Span::styled(
            " STATS ",
            if app.mode == Mode::Stats {
                Style::default().fg(colors::BG).bg(colors::GREEN).bold()
            } else {
                Style::default().fg(colors::COMMENT)
            },
        ),
    ]);
    f.render_widget(Paragraph::new(tabs), chunks[0]);

    // Content
    match app.mode {
        Mode::Pomodoro => render_pomodoro(f, chunks[2], app),
        Mode::Git => render_git(f, chunks[2], app),
        Mode::Stats => render_stats(f, chunks[2], app),
    }

    // Help
    let help = Line::from(vec![
        Span::styled("TAB", Style::default().fg(colors::YELLOW)),
        Span::styled(" mode ", Style::default().fg(colors::COMMENT)),
        Span::styled("SPC", Style::default().fg(colors::YELLOW)),
        Span::styled(" start ", Style::default().fg(colors::COMMENT)),
        Span::styled("r", Style::default().fg(colors::YELLOW)),
        Span::styled(" reset ", Style::default().fg(colors::COMMENT)),
        Span::styled("q", Style::default().fg(colors::YELLOW)),
        Span::styled(" quit", Style::default().fg(colors::COMMENT)),
    ]);
    f.render_widget(Paragraph::new(help), chunks[3]);
}

fn render_pomodoro(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // State
            Constraint::Length(1), // Spacer
            Constraint::Length(3), // Timer
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // Progress
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // Sessions
        ])
        .split(area);

    // State label
    let (state_text, state_color) = match app.pomo_state {
        PomodoroState::Work => ("FOCUS", colors::RED),
        PomodoroState::Break => ("BREAK", colors::GREEN),
        PomodoroState::Paused => ("PAUSED", colors::COMMENT),
    };
    let state = Paragraph::new(state_text)
        .style(Style::default().fg(state_color).bold())
        .alignment(Alignment::Center);
    f.render_widget(state, chunks[0]);

    // Timer
    let mins = app.pomo_remaining.as_secs() / 60;
    let secs = app.pomo_remaining.as_secs() % 60;
    let timer_text = format!("{:02}:{:02}", mins, secs);
    let timer = Paragraph::new(timer_text)
        .style(Style::default().fg(colors::FG).bold())
        .alignment(Alignment::Center);
    f.render_widget(timer, chunks[2]);

    // Progress bar
    let progress = if app.pomo_total.as_secs() > 0 {
        1.0 - (app.pomo_remaining.as_secs_f64() / app.pomo_total.as_secs_f64())
    } else {
        0.0
    };
    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(match app.pomo_state {
            PomodoroState::Work => colors::RED,
            PomodoroState::Break => colors::GREEN,
            PomodoroState::Paused => colors::COMMENT,
        }))
        .ratio(progress)
        .label("");
    f.render_widget(gauge, chunks[4]);

    // Sessions (this session)
    let sessions = Paragraph::new(format!("Session: {}", app.pomo_sessions))
        .style(Style::default().fg(colors::COMMENT))
        .alignment(Alignment::Center);
    f.render_widget(sessions, chunks[6]);
}

fn render_git(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Branch
            Constraint::Length(1), // Spacer
            Constraint::Min(0),    // Files
        ])
        .split(area);

    // Branch
    let branch = Paragraph::new(Line::from(vec![
        Span::styled(" ", Style::default().fg(colors::MAGENTA)),
        Span::styled(&app.git_branch, Style::default().fg(colors::FG).bold()),
    ]));
    f.render_widget(branch, chunks[0]);

    // File lists
    let mut items: Vec<ListItem> = Vec::new();

    if !app.git_staged.is_empty() {
        items.push(ListItem::new(Span::styled(
            "Staged:",
            Style::default().fg(colors::GREEN).bold(),
        )));
        for file in &app.git_staged {
            items.push(ListItem::new(Span::styled(
                format!("  + {}", truncate(file, 20)),
                Style::default().fg(colors::GREEN),
            )));
        }
    }

    if !app.git_modified.is_empty() {
        if !items.is_empty() {
            items.push(ListItem::new(""));
        }
        items.push(ListItem::new(Span::styled(
            "Modified:",
            Style::default().fg(colors::YELLOW).bold(),
        )));
        for file in &app.git_modified {
            items.push(ListItem::new(Span::styled(
                format!("  ~ {}", truncate(file, 20)),
                Style::default().fg(colors::YELLOW),
            )));
        }
    }

    if !app.git_untracked.is_empty() {
        if !items.is_empty() {
            items.push(ListItem::new(""));
        }
        items.push(ListItem::new(Span::styled(
            "Untracked:",
            Style::default().fg(colors::RED).bold(),
        )));
        for file in &app.git_untracked {
            items.push(ListItem::new(Span::styled(
                format!("  ? {}", truncate(file, 20)),
                Style::default().fg(colors::RED),
            )));
        }
    }

    if items.is_empty() {
        items.push(ListItem::new(Span::styled(
            "Clean working tree",
            Style::default().fg(colors::GREEN),
        )));
    }

    let list = List::new(items);
    f.render_widget(list, chunks[2]);
}

fn render_stats(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Header
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // Today sessions
            Constraint::Length(1), // Today time
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // Total sessions
            Constraint::Length(1), // Total time
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // Streak
            Constraint::Min(0),    // Remaining
        ])
        .split(area);

    // Header
    let header = Paragraph::new("Your Progress")
        .style(Style::default().fg(colors::GREEN).bold())
        .alignment(Alignment::Center);
    f.render_widget(header, chunks[0]);

    // Today
    let today_sessions = Paragraph::new(format!("Today: {} sessions", app.stats.today_sessions))
        .style(Style::default().fg(colors::FG))
        .alignment(Alignment::Center);
    f.render_widget(today_sessions, chunks[2]);

    let today_time = Paragraph::new(format!("       {} mins", app.stats.today_focus_mins))
        .style(Style::default().fg(colors::COMMENT))
        .alignment(Alignment::Center);
    f.render_widget(today_time, chunks[3]);

    // Total
    let total_sessions = Paragraph::new(format!("Total: {} sessions", app.stats.total_sessions))
        .style(Style::default().fg(colors::FG))
        .alignment(Alignment::Center);
    f.render_widget(total_sessions, chunks[5]);

    let total_hours = app.stats.total_focus_mins / 60;
    let total_mins = app.stats.total_focus_mins % 60;
    let time_str = if total_hours > 0 {
        format!("       {}h {}m", total_hours, total_mins)
    } else {
        format!("       {} mins", total_mins)
    };
    let total_time = Paragraph::new(time_str)
        .style(Style::default().fg(colors::COMMENT))
        .alignment(Alignment::Center);
    f.render_widget(total_time, chunks[6]);

    // Streak
    let streak_text = if app.stats.streak_days > 0 {
        format!("Streak: {} days", app.stats.streak_days)
    } else {
        "Streak: 0 days".to_string()
    };
    let streak = Paragraph::new(streak_text)
        .style(Style::default().fg(colors::YELLOW).bold())
        .alignment(Alignment::Center);
    f.render_widget(streak, chunks[8]);
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}...", &s[..max - 3])
    } else {
        s.to_string()
    }
}
