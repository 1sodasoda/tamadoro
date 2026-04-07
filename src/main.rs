use std::{
    env,
    fs,
    io::{self, stdout},
    path::PathBuf,
    process::Command,
    time::{Duration, Instant},
};

use chrono::{Local, NaiveDate, Timelike};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rand::{seq::SliceRandom, Rng};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge, Paragraph},
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

// Large ASCII digits for clock display (tty-clock style, all 7 chars wide)
mod ascii_digits {
    pub const DIGIT_0: &[&str] = &[
        "███████",
        "██   ██",
        "██   ██",
        "██   ██",
        "███████",
    ];
    pub const DIGIT_1: &[&str] = &[
        "     ██",
        "     ██",
        "     ██",
        "     ██",
        "     ██",
    ];
    pub const DIGIT_2: &[&str] = &[
        "███████",
        "     ██",
        "███████",
        "██     ",
        "███████",
    ];
    pub const DIGIT_3: &[&str] = &[
        "███████",
        "     ██",
        "███████",
        "     ██",
        "███████",
    ];
    pub const DIGIT_4: &[&str] = &[
        "██   ██",
        "██   ██",
        "███████",
        "     ██",
        "     ██",
    ];
    pub const DIGIT_5: &[&str] = &[
        "███████",
        "██     ",
        "███████",
        "     ██",
        "███████",
    ];
    pub const DIGIT_6: &[&str] = &[
        "███████",
        "██     ",
        "███████",
        "██   ██",
        "███████",
    ];
    pub const DIGIT_7: &[&str] = &[
        "███████",
        "     ██",
        "     ██",
        "     ██",
        "     ██",
    ];
    pub const DIGIT_8: &[&str] = &[
        "███████",
        "██   ██",
        "███████",
        "██   ██",
        "███████",
    ];
    pub const DIGIT_9: &[&str] = &[
        "███████",
        "██   ██",
        "███████",
        "     ██",
        "███████",
    ];
    pub const COLON: &[&str] = &[
        " ",
        "█",
        " ",
        "█",
        " ",
    ];

    pub fn get_digit(d: char) -> &'static [&'static str] {
        match d {
            '0' => DIGIT_0,
            '1' => DIGIT_1,
            '2' => DIGIT_2,
            '3' => DIGIT_3,
            '4' => DIGIT_4,
            '5' => DIGIT_5,
            '6' => DIGIT_6,
            '7' => DIGIT_7,
            '8' => DIGIT_8,
            '9' => DIGIT_9,
            ':' => COLON,
            _ => DIGIT_0,
        }
    }
}

// Pet types
#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
enum PetType {
    Blob,
    Cat,
    Robot,
    Ghost,
}

impl PetType {
    fn random() -> Self {
        match rand::thread_rng().gen_range(0..4) {
            0 => PetType::Blob,
            1 => PetType::Cat,
            2 => PetType::Robot,
            _ => PetType::Ghost,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            PetType::Blob => "Blob",
            PetType::Cat => "Cat",
            PetType::Robot => "Robot",
            PetType::Ghost => "Ghost",
        }
    }
}

// Pet evolution stages with ASCII art
mod pet {
    use super::PetType;

    // === EGGS (all types share similar eggs with slight variation) ===
    pub const EGG_BLOB: &[&str] = &["  ___  ", " /   \\ ", "|  ·  |", " \\___/ "];
    pub const EGG_BLOB_WOBBLE: &[&str] = &["  ___  ", " /   \\ ", "|  *  |", " \\___/ "];
    pub const EGG_CAT: &[&str] = &["  ___  ", " /^ ^\\ ", "|     |", " \\___/ "];
    pub const EGG_CAT_WOBBLE: &[&str] = &["  ___  ", " /^ ^\\ ", "|  ~  |", " \\___/ "];
    pub const EGG_ROBOT: &[&str] = &["  ___  ", " [   ] ", "|  #  |", " [___] "];
    pub const EGG_ROBOT_WOBBLE: &[&str] = &["  ___  ", " [   ] ", "|  @  |", " [___] "];
    pub const EGG_GHOST: &[&str] = &["  ___  ", " (   ) ", "|  ~  |", " \\^_^/ "];
    pub const EGG_GHOST_WOBBLE: &[&str] = &["  ___  ", " (   ) ", "|  o  |", " \\^_^/ "];

    // === BLOB ===
    pub const BLOB_BABY: &[&str] = &[" (^_^) ", "  /|\\  ", "   |   "];
    pub const BLOB_BABY_HAPPY: &[&str] = &[" (^o^) ", " \\|^|/ ", "   |   "];
    pub const BLOB_BABY_WORK: &[&str] = &[" (>_<) ", "  /|\\  ", "  /|   "];
    pub const BLOB_BABY_SLEEP: &[&str] = &[" (-_-) ", "  /|\\  ", "  z z  "];

    pub const BLOB_TEEN: &[&str] = &["  \\o/  ", "   |   ", "  / \\  "];
    pub const BLOB_TEEN_HAPPY: &[&str] = &[" \\(^o^)/", "   |   ", "  / \\  "];
    pub const BLOB_TEEN_WORK: &[&str] = &["  (•_•)", "  <|>  ", "  / \\  "];
    pub const BLOB_TEEN_SLEEP: &[&str] = &["  (_ _)", "  /|\\  ", " z/ \\z "];

    pub const BLOB_ADULT: &[&str] = &[" ★\\o/★ ", "   |   ", "  / \\  "];
    pub const BLOB_ADULT_HAPPY: &[&str] = &["★(^o^)★", " \\|^|/ ", "  / \\  "];
    pub const BLOB_ADULT_WORK: &[&str] = &[" ★•_•★ ", "  <|>  ", "  / \\  "];
    pub const BLOB_ADULT_SLEEP: &[&str] = &[" ★-_-★ ", "  /|\\  ", " z   z "];

    // === CAT ===
    pub const CAT_BABY: &[&str] = &[" /\\_/\\ ", "( o.o )", " > ^ < "];
    pub const CAT_BABY_HAPPY: &[&str] = &[" /\\_/\\ ", "( ^w^ )", " > ~ < "];
    pub const CAT_BABY_WORK: &[&str] = &[" /\\_/\\ ", "( -.- )", " > n < "];
    pub const CAT_BABY_SLEEP: &[&str] = &[" /\\_/\\ ", "( -.- )", "  zZz  "];

    pub const CAT_TEEN: &[&str] = &["  /\\_/\\  ", " ( o.o ) ", "  />~<\\  "];
    pub const CAT_TEEN_HAPPY: &[&str] = &["  /\\_/\\  ", " ( ^o^ ) ", " ~/>~<\\~ "];
    pub const CAT_TEEN_WORK: &[&str] = &["  /\\_/\\  ", " ( -_- ) ", "  />o<\\  "];
    pub const CAT_TEEN_SLEEP: &[&str] = &["  /\\_/\\  ", " ( -.- ) ", " z/>~<\\z "];

    pub const CAT_ADULT: &[&str] = &[" ★/\\_/\\★", "  (o.o) ", "  />~<\\ "];
    pub const CAT_ADULT_HAPPY: &[&str] = &[" ★/\\_/\\★", "  (^ω^) ", " ★/>~<\\★"];
    pub const CAT_ADULT_WORK: &[&str] = &[" ★/\\_/\\★", "  (•_•) ", "  />o<\\ "];
    pub const CAT_ADULT_SLEEP: &[&str] = &[" ★/\\_/\\★", "  (-.-) ", " z/>~<\\z"];

    // === ROBOT ===
    pub const ROBOT_BABY: &[&str] = &[" [o_o] ", "  ]|[  ", "  d b  "];
    pub const ROBOT_BABY_HAPPY: &[&str] = &[" [^_^] ", " \\]|[/ ", "  d b  "];
    pub const ROBOT_BABY_WORK: &[&str] = &[" [0_0] ", "  ]|[  ", "  d|b  "];
    pub const ROBOT_BABY_SLEEP: &[&str] = &[" [-_-] ", "  ]|[  ", "  d b  "];

    pub const ROBOT_TEEN: &[&str] = &[" ┌[o_o]┐", "  ╠═╣  ", "  /I\\  "];
    pub const ROBOT_TEEN_HAPPY: &[&str] = &[" ┌[^_^]┐", " \\╠═╣/ ", "  /I\\  "];
    pub const ROBOT_TEEN_WORK: &[&str] = &[" ┌[0_0]┐", "  ╠═╣  ", "  /Y\\  "];
    pub const ROBOT_TEEN_SLEEP: &[&str] = &[" ┌[-_-]┐", "  ╠═╣  ", " z/I\\z "];

    pub const ROBOT_ADULT: &[&str] = &["★┌[o_o]┐★", "  ╠═╣  ", "  /Π\\  "];
    pub const ROBOT_ADULT_HAPPY: &[&str] = &["★┌[^▽^]┐★", " \\╠═╣/ ", "  /Π\\  "];
    pub const ROBOT_ADULT_WORK: &[&str] = &["★┌[◉_◉]┐★", "  ╠═╣  ", "  /Y\\  "];
    pub const ROBOT_ADULT_SLEEP: &[&str] = &["★┌[-_-]┐★", "  ╠═╣  ", " z/Π\\z "];

    // === GHOST ===
    pub const GHOST_BABY: &[&str] = &[" .---. ", "( o o )", " \\~~~/ "];
    pub const GHOST_BABY_HAPPY: &[&str] = &[" .---. ", "( ^o^ )", " \\^^^/ "];
    pub const GHOST_BABY_WORK: &[&str] = &[" .---. ", "( o_o )", " \\.../ "];
    pub const GHOST_BABY_SLEEP: &[&str] = &[" .---. ", "( -_- )", " \\zzz/ "];

    pub const GHOST_TEEN: &[&str] = &["  .--.  ", " ( oo ) ", " /|~~|\\ "];
    pub const GHOST_TEEN_HAPPY: &[&str] = &["  .--.  ", " ( ^^ ) ", "~/|~~|\\~"];
    pub const GHOST_TEEN_WORK: &[&str] = &["  .--.  ", " ( o_o) ", " /|..|\\"];
    pub const GHOST_TEEN_SLEEP: &[&str] = &["  .--.  ", " ( -.- ) ", "z/|~~|\\z"];

    pub const GHOST_ADULT: &[&str] = &[" ★.---.★", "  (o o) ", " ★\\~~~/★"];
    pub const GHOST_ADULT_HAPPY: &[&str] = &[" ★.---.★", "  (^o^) ", " ★\\^^^/★"];
    pub const GHOST_ADULT_WORK: &[&str] = &[" ★.---.★", "  (◉_◉) ", " ★\\_.~/★"];
    pub const GHOST_ADULT_SLEEP: &[&str] = &[" ★.---.★", "  (-_-) ", "z★\\~~~/★"];

    // === DEAD ===
    pub const DEAD_BLOB: &[&str] = &[" (x_x) ", "  /|\\  ", "  RIP  "];
    pub const DEAD_CAT: &[&str] = &[" /\\_/\\ ", "( x.x )", "  RIP  "];
    pub const DEAD_ROBOT: &[&str] = &[" [x_x] ", "  ]|[  ", "  RIP  "];
    pub const DEAD_GHOST: &[&str] = &[" .---. ", "( x x )", "  RIP  "];

    pub fn get_dead_art(pet_type: PetType) -> &'static [&'static str] {
        match pet_type {
            PetType::Blob => DEAD_BLOB,
            PetType::Cat => DEAD_CAT,
            PetType::Robot => DEAD_ROBOT,
            PetType::Ghost => DEAD_GHOST,
        }
    }

    pub fn get_art(pet_type: PetType, stage: u32, mood: super::PetMood, frame: usize) -> &'static [&'static str] {
        use super::PetMood;

        // Egg stage - all pets
        if stage == 1 {
            let wobble = frame % 4 == 0;
            return match pet_type {
                PetType::Blob => if wobble { EGG_BLOB_WOBBLE } else { EGG_BLOB },
                PetType::Cat => if wobble { EGG_CAT_WOBBLE } else { EGG_CAT },
                PetType::Robot => if wobble { EGG_ROBOT_WOBBLE } else { EGG_ROBOT },
                PetType::Ghost => if wobble { EGG_GHOST_WOBBLE } else { EGG_GHOST },
            };
        }

        match pet_type {
            PetType::Blob => match stage {
                2 => match mood {
                    PetMood::Working => BLOB_BABY_WORK,
                    PetMood::Happy => BLOB_BABY_HAPPY,
                    PetMood::Resting => BLOB_BABY_SLEEP,
                    PetMood::Idle => BLOB_BABY,
                },
                3 => match mood {
                    PetMood::Working => BLOB_TEEN_WORK,
                    PetMood::Happy => BLOB_TEEN_HAPPY,
                    PetMood::Resting => BLOB_TEEN_SLEEP,
                    PetMood::Idle => BLOB_TEEN,
                },
                _ => match mood {
                    PetMood::Working => BLOB_ADULT_WORK,
                    PetMood::Happy => BLOB_ADULT_HAPPY,
                    PetMood::Resting => BLOB_ADULT_SLEEP,
                    PetMood::Idle => BLOB_ADULT,
                },
            },
            PetType::Cat => match stage {
                2 => match mood {
                    PetMood::Working => CAT_BABY_WORK,
                    PetMood::Happy => CAT_BABY_HAPPY,
                    PetMood::Resting => CAT_BABY_SLEEP,
                    PetMood::Idle => CAT_BABY,
                },
                3 => match mood {
                    PetMood::Working => CAT_TEEN_WORK,
                    PetMood::Happy => CAT_TEEN_HAPPY,
                    PetMood::Resting => CAT_TEEN_SLEEP,
                    PetMood::Idle => CAT_TEEN,
                },
                _ => match mood {
                    PetMood::Working => CAT_ADULT_WORK,
                    PetMood::Happy => CAT_ADULT_HAPPY,
                    PetMood::Resting => CAT_ADULT_SLEEP,
                    PetMood::Idle => CAT_ADULT,
                },
            },
            PetType::Robot => match stage {
                2 => match mood {
                    PetMood::Working => ROBOT_BABY_WORK,
                    PetMood::Happy => ROBOT_BABY_HAPPY,
                    PetMood::Resting => ROBOT_BABY_SLEEP,
                    PetMood::Idle => ROBOT_BABY,
                },
                3 => match mood {
                    PetMood::Working => ROBOT_TEEN_WORK,
                    PetMood::Happy => ROBOT_TEEN_HAPPY,
                    PetMood::Resting => ROBOT_TEEN_SLEEP,
                    PetMood::Idle => ROBOT_TEEN,
                },
                _ => match mood {
                    PetMood::Working => ROBOT_ADULT_WORK,
                    PetMood::Happy => ROBOT_ADULT_HAPPY,
                    PetMood::Resting => ROBOT_ADULT_SLEEP,
                    PetMood::Idle => ROBOT_ADULT,
                },
            },
            PetType::Ghost => match stage {
                2 => match mood {
                    PetMood::Working => GHOST_BABY_WORK,
                    PetMood::Happy => GHOST_BABY_HAPPY,
                    PetMood::Resting => GHOST_BABY_SLEEP,
                    PetMood::Idle => GHOST_BABY,
                },
                3 => match mood {
                    PetMood::Working => GHOST_TEEN_WORK,
                    PetMood::Happy => GHOST_TEEN_HAPPY,
                    PetMood::Resting => GHOST_TEEN_SLEEP,
                    PetMood::Idle => GHOST_TEEN,
                },
                _ => match mood {
                    PetMood::Working => GHOST_ADULT_WORK,
                    PetMood::Happy => GHOST_ADULT_HAPPY,
                    PetMood::Resting => GHOST_ADULT_SLEEP,
                    PetMood::Idle => GHOST_ADULT,
                },
            },
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Mode {
    Timer,
    Pet,
    Stats,
    Debug,
}

#[derive(PartialEq, Clone, Copy)]
enum PomodoroState {
    Work,
    Break,
    Paused,
}

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
enum PetMood {
    Idle,
    Working,
    Happy,
    Resting,
}

#[derive(Serialize, Deserialize)]
struct GameData {
    // XP and leveling
    xp: u32,
    level: u32,
    // Stats
    total_sessions: u32,
    total_focus_mins: u32,
    last_session_date: Option<NaiveDate>,
    streak_days: u32,
    today_sessions: u32,
    today_date: Option<NaiveDate>,
    // Pet
    pet_name: String,
    pet_type: PetType,
    mood: PetMood,
    // Food system (0-100)
    food: u32,
    #[serde(default)]
    last_food_check: Option<i64>, // Unix timestamp
    #[serde(default)]
    hunger_zero_since: Option<i64>, // When food hit 0
    #[serde(default)]
    is_dead: bool,
}

impl Default for GameData {
    fn default() -> Self {
        let pet_type = PetType::random();
        Self {
            xp: 0,
            level: 1,
            total_sessions: 0,
            total_focus_mins: 0,
            last_session_date: None,
            streak_days: 0,
            today_sessions: 0,
            today_date: Some(Local::now().date_naive()),
            pet_name: "Tomo".to_string(),
            pet_type,
            mood: PetMood::Idle,
            food: 100,
            last_food_check: Some(Local::now().timestamp()),
            hunger_zero_since: None,
            is_dead: false,
        }
    }
}

impl GameData {
    fn data_path() -> PathBuf {
        let data_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("tamadoro");
        fs::create_dir_all(&data_dir).ok();
        data_dir.join("save.json")
    }

    fn load() -> Self {
        let path = Self::data_path();
        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(mut game) = serde_json::from_str::<GameData>(&data) {
                let today = Local::now().date_naive();
                if game.today_date != Some(today) {
                    game.today_sessions = 0;
                    game.today_date = Some(today);
                }
                return game;
            }
        }
        GameData::default()
    }

    fn save(&self) {
        let path = Self::data_path();
        if let Ok(data) = serde_json::to_string_pretty(self) {
            fs::write(path, data).ok();
        }
    }

    fn xp_for_level(level: u32) -> u32 {
        // XP needed: 100, 150, 225, 337, ...
        (100.0 * 1.5_f64.powi((level - 1) as i32)) as u32
    }

    fn xp_to_next_level(&self) -> u32 {
        Self::xp_for_level(self.level)
    }

    fn add_xp(&mut self, amount: u32) {
        self.xp += amount;
        while self.xp >= self.xp_to_next_level() {
            self.xp -= self.xp_to_next_level();
            self.level += 1;
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
        } else {
            self.streak_days = 1;
        }

        self.last_session_date = Some(today);
        self.total_sessions += 1;
        self.total_focus_mins += 25;

        if self.today_date != Some(today) {
            self.today_sessions = 0;
            self.today_date = Some(today);
        }
        self.today_sessions += 1;

        // Award XP with streak bonus
        let base_xp = 25;
        let streak_bonus = (self.streak_days.min(7) * 5) as u32;
        self.add_xp(base_xp + streak_bonus);

        // Replenish food
        self.feed(25);

        self.mood = PetMood::Happy;
        self.save();
    }

    fn evolution_stage(&self) -> u32 {
        if self.level < 5 {
            1 // Egg
        } else if self.level < 15 {
            2 // Baby
        } else if self.level < 30 {
            3 // Teen
        } else {
            4 // Adult
        }
    }

    fn get_pet_art(&self, frame: usize) -> &'static [&'static str] {
        if self.is_dead {
            pet::get_dead_art(self.pet_type)
        } else {
            pet::get_art(self.pet_type, self.evolution_stage(), self.mood, frame)
        }
    }

    fn stage_name(&self) -> &'static str {
        match self.evolution_stage() {
            1 => "Egg",
            2 => "Baby",
            3 => "Teen",
            _ => "Master",
        }
    }

    fn is_daytime() -> bool {
        let hour = Local::now().hour();
        hour >= 6 && hour < 22 // 6am to 10pm
    }

    fn hunger_cry(&self) -> &'static str {
        match self.pet_type {
            PetType::Blob => "I'm so hungry... please help me! I need food! 😢",
            PetType::Cat => "Meow... *weak* ...please... food... I'm starving... 🐱",
            PetType::Robot => "CRITICAL: Energy depleted... shutting down... need fuel... 🤖",
            PetType::Ghost => "I'm fading away... so hungry... please don't let me disappear... 👻",
        }
    }

    fn death_cry(&self) -> &'static str {
        match self.pet_type {
            PetType::Blob => "Goodbye... I waited but you never came... 💔",
            PetType::Cat => "Meow... *silence* ...goodbye friend... 🐱💔",
            PetType::Robot => "SYSTEM FAILURE... memory banks... fading... goodbye... 🤖💔",
            PetType::Ghost => "I'm disappearing forever... I'll miss you... 👻💔",
        }
    }

    fn send_notification(title: &str, message: &str) {
        let script = format!(
            "display notification \"{}\" with title \"{}\"",
            message.replace('"', "\\\""),
            title.replace('"', "\\\"")
        );
        Command::new("osascript")
            .args(["-e", &script])
            .spawn()
            .ok();
    }

    fn update_food(&mut self) {
        // Don't update if already dead
        if self.is_dead {
            return;
        }

        let now = Local::now().timestamp();

        // Initialize if not set
        if self.last_food_check.is_none() {
            self.last_food_check = Some(now);
            return;
        }

        let last_check = self.last_food_check.unwrap();
        let elapsed_mins = ((now - last_check) / 60) as u32;

        // Only decrease during daytime, ~1 food per 10 minutes (lasts ~16 hours)
        if Self::is_daytime() && elapsed_mins >= 10 {
            let decrease = elapsed_mins / 10;
            let was_fed = self.food > 0;
            self.food = self.food.saturating_sub(decrease);
            self.last_food_check = Some(now);

            // Food just hit 0 - send notification and start death timer
            if was_fed && self.food == 0 {
                self.hunger_zero_since = Some(now);
                Self::send_notification("Tamadoro - HUNGRY!", self.hunger_cry());
            }

            self.save();
        } else if !Self::is_daytime() {
            // Update timestamp during night so we don't accumulate
            self.last_food_check = Some(now);
        }

        // Check for death (3 hours = 10800 seconds at 0 food)
        if self.food == 0 {
            if let Some(zero_since) = self.hunger_zero_since {
                if now - zero_since >= 10800 {
                    self.is_dead = true;
                    Self::send_notification("Tamadoro - 💀", self.death_cry());
                    self.save();
                }
            }
        } else {
            // Food > 0, reset death timer
            self.hunger_zero_since = None;
        }
    }

    fn feed(&mut self, amount: u32) {
        self.food = (self.food + amount).min(100);
        if self.food > 0 {
            self.hunger_zero_since = None;
        }
    }
}

struct App {
    mode: Mode,
    pomo_state: PomodoroState,
    pomo_remaining: Duration,
    pomo_total: Duration,
    pomo_sessions: u32,
    last_tick: Instant,
    game: GameData,
    frame: usize,
    message: Option<(String, Instant)>,
    paused_from_state: Option<PomodoroState>, // Track what state we paused from
    test_mode: bool, // Whether --test flag was passed
    pet_speech: Option<(String, Instant)>, // Current speech and when it started
    next_speech_time: Instant, // When the next speech should trigger
}

impl App {
    fn new(test_mode: bool) -> Self {
        let work_duration = Duration::from_secs(25 * 60);
        let now = Instant::now();
        // First speech in 30-60 seconds
        let next_speech_delay = Duration::from_secs(rand::thread_rng().gen_range(30..60));

        App {
            mode: Mode::Timer,
            pomo_state: PomodoroState::Paused,
            pomo_remaining: work_duration,
            pomo_total: work_duration,
            pomo_sessions: 0,
            last_tick: now,
            game: GameData::load(),
            frame: 0,
            message: None,
            paused_from_state: None,
            test_mode,
            pet_speech: None,
            next_speech_time: now + next_speech_delay,
        }
    }

    fn tick(&mut self) {
        self.frame = self.frame.wrapping_add(1);

        // Update food (decreases during daytime)
        self.game.update_food();

        // Clear old messages
        if let Some((_, time)) = &self.message {
            if time.elapsed() > Duration::from_secs(3) {
                self.message = None;
            }
        }

        // Handle pet speech
        if let Some((_, time)) = &self.pet_speech {
            // Clear speech after 5 seconds
            if time.elapsed() > Duration::from_secs(5) {
                self.pet_speech = None;
            }
        } else {
            // Check if it's time for new speech
            if Instant::now() >= self.next_speech_time {
                let phrase = self.get_pet_phrase();
                self.pet_speech = Some((phrase.to_string(), Instant::now()));
                // Next speech in 45-120 seconds
                self.next_speech_time = Instant::now() + Duration::from_secs(rand::thread_rng().gen_range(45..120));
            }
        }

        if self.pomo_state == PomodoroState::Work || self.pomo_state == PomodoroState::Break {
            let elapsed = self.last_tick.elapsed();
            self.last_tick = Instant::now();

            if self.pomo_remaining > elapsed {
                self.pomo_remaining -= elapsed;
            } else {
                if self.pomo_state == PomodoroState::Work {
                    self.pomo_sessions += 1;
                    let old_level = self.game.level;
                    self.game.record_session();

                    if self.game.level > old_level {
                        self.message = Some((
                            format!("LEVEL UP! Lv.{}", self.game.level),
                            Instant::now(),
                        ));
                        if self.game.evolution_stage()
                            > (GameData {
                                level: old_level,
                                ..Default::default()
                            })
                            .evolution_stage()
                        {
                            self.message = Some((
                                format!("{} evolved!", self.game.pet_name),
                                Instant::now(),
                            ));
                        }
                    } else {
                        let messages = [
                            "Great work!",
                            "Amazing focus!",
                            "You're on fire!",
                            "Keep it up!",
                            "Fantastic!",
                        ];
                        let msg = messages[rand::thread_rng().gen_range(0..messages.len())];
                        self.message = Some((msg.to_string(), Instant::now()));
                    }

                    self.pomo_state = PomodoroState::Break;
                    self.pomo_total = Duration::from_secs(5 * 60);
                    self.pomo_remaining = self.pomo_total;
                    self.paused_from_state = None; // Clear saved state on natural transition
                    self.game.mood = PetMood::Resting;
                } else {
                    self.pomo_state = PomodoroState::Paused;
                    self.pomo_total = Duration::from_secs(25 * 60);
                    self.pomo_remaining = self.pomo_total;
                    self.paused_from_state = None; // Clear saved state on natural transition
                    self.game.mood = PetMood::Idle;
                    self.message = Some(("Break over! Ready?".to_string(), Instant::now()));
                }
            }
        }
    }

    fn toggle_pomo(&mut self) {
        match self.pomo_state {
            PomodoroState::Paused => {
                // Resume to the state we paused from, or default to Work
                let resume_state = self.paused_from_state.unwrap_or(PomodoroState::Work);
                self.pomo_state = resume_state;
                self.paused_from_state = None; // Clear the saved state
                self.last_tick = Instant::now();

                // Set mood based on what state we're resuming to
                self.game.mood = match resume_state {
                    PomodoroState::Work => PetMood::Working,
                    PomodoroState::Break => PetMood::Resting,
                    PomodoroState::Paused => PetMood::Idle,
                };
                self.game.save();
            }
            _ => {
                // Save the current state before pausing
                self.paused_from_state = Some(self.pomo_state);
                self.pomo_state = PomodoroState::Paused;
                self.game.mood = PetMood::Idle;
                self.game.save();
            }
        }
    }

    fn reset_pomo(&mut self) {
        self.pomo_state = PomodoroState::Paused;
        self.pomo_total = Duration::from_secs(25 * 60);
        self.pomo_remaining = self.pomo_total;
        self.paused_from_state = None; // Clear saved state on reset
        self.game.mood = PetMood::Idle;
        self.game.save();
    }

    fn get_pet_phrase(&self) -> &'static str {
        use PetMood::*;
        use PetType::*;

        // Special cases first
        if self.game.is_dead {
            return match self.game.pet_type {
                Blob => "...",
                Cat => "*silence*",
                Robot => "ERROR 404",
                Ghost => "......",
            };
        }

        if self.game.food < 20 {
            return match self.game.pet_type {
                Blob => "I'm hungry...",
                Cat => "Meow! Feed me!",
                Robot => "Low energy!",
                Ghost => "Need food...",
            };
        }

        match self.game.mood {
            Working => match self.game.pet_type {
                Blob => *["You got this!", "Keep going!", "Focus!", "Stay strong!"].choose(&mut rand::thread_rng()).unwrap(),
                Cat => *["Purr... working hard?", "Meow! Stay focused!", "You're doing great!", "Keep it up, human!"].choose(&mut rand::thread_rng()).unwrap(),
                Robot => *["PROCESSING...", "Productivity mode!", "Executing tasks!", "System optimal!"].choose(&mut rand::thread_rng()).unwrap(),
                Ghost => *["Boo! Keep working!", "You can do it!", "Floating by to cheer!", "Focus time!"].choose(&mut rand::thread_rng()).unwrap(),
            },
            Happy => match self.game.pet_type {
                Blob => *["Yay! We did it!", "So happy!", "Woohoo!", "Great work!"].choose(&mut rand::thread_rng()).unwrap(),
                Cat => *["Purrrr~", "Meow! ^w^", "Head pats please!", "You're the best!"].choose(&mut rand::thread_rng()).unwrap(),
                Robot => *["SUCCESS!", "Task completed!", "Happiness.exe", "Achievement unlocked!"].choose(&mut rand::thread_rng()).unwrap(),
                Ghost => *["Boo! Yay!", "Spooky happy!", "We did it!", "Ghostly cheer!"].choose(&mut rand::thread_rng()).unwrap(),
            },
            Resting => match self.game.pet_type {
                Blob => *["zzz...", "Nice break...", "Relaxing~", "So comfy..."].choose(&mut rand::thread_rng()).unwrap(),
                Cat => *["*yawn* meow", "Nap time!", "zzz purr zzz", "Sleep well~"].choose(&mut rand::thread_rng()).unwrap(),
                Robot => *["Sleep mode...", "Recharging...", "Systems idle...", "zzz beep zzz"].choose(&mut rand::thread_rng()).unwrap(),
                Ghost => *["Floating dreams...", "zzz boo zzz", "Resting~", "Peaceful..."].choose(&mut rand::thread_rng()).unwrap(),
            },
            Idle => match self.game.pet_type {
                Blob => *["Hello!", "Ready when you are!", "Let's work!", "How are you?"].choose(&mut rand::thread_rng()).unwrap(),
                Cat => *["Meow!", "Pet me!", "*stares*", "Let's go!"].choose(&mut rand::thread_rng()).unwrap(),
                Robot => *["READY.", "Standing by!", "Awaiting input!", "Hello, human!"].choose(&mut rand::thread_rng()).unwrap(),
                Ghost => *["Boo!", "Floating here~", "Ready to help!", "Let's start!"].choose(&mut rand::thread_rng()).unwrap(),
            },
        }
    }
}

fn main() -> io::Result<()> {
    // Check for --test flag
    let test_mode = env::args().any(|arg| arg == "--test");

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app = App::new(test_mode);
    let tick_rate = Duration::from_millis(250);

    loop {
        terminal.draw(|f| ui(f, &app))?;

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
                            app.message = Some((format!("Level -> {}", app.game.level), Instant::now()));
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
                            app.message = Some((format!("Evolved to {}!", app.game.stage_name()), Instant::now()));
                        }
                        KeyCode::Char('5') if app.mode == Mode::Debug => {
                            app.game.streak_days = (app.game.streak_days + 1) % 15;
                            app.game.save();
                            app.message = Some((format!("Streak -> {}", app.game.streak_days), Instant::now()));
                        }
                        KeyCode::Char('6') if app.mode == Mode::Debug => {
                            app.game.pet_type = match app.game.pet_type {
                                PetType::Blob => PetType::Cat,
                                PetType::Cat => PetType::Robot,
                                PetType::Robot => PetType::Ghost,
                                PetType::Ghost => PetType::Blob,
                            };
                            app.game.save();
                            app.message = Some((format!("Pet -> {}", app.game.pet_type.name()), Instant::now()));
                        }
                        KeyCode::Char('7') if app.mode == Mode::Debug => {
                            app.game.feed(25);
                            app.game.save();
                            app.message = Some((format!("Food -> {}", app.game.food), Instant::now()));
                        }
                        KeyCode::Char('8') if app.mode == Mode::Debug => {
                            app.game.food = app.game.food.saturating_sub(25);
                            app.game.save();
                            app.message = Some((format!("Food -> {}", app.game.food), Instant::now()));
                        }
                        KeyCode::Char('9') if app.mode == Mode::Debug => {
                            app.game.is_dead = !app.game.is_dead;
                            if !app.game.is_dead {
                                app.game.food = 50; // Revive with some food
                                app.game.hunger_zero_since = None;
                            }
                            app.game.save();
                            app.message = Some((
                                if app.game.is_dead { "Pet died!".to_string() } else { "Pet revived!".to_string() },
                                Instant::now()
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

fn render_large_clock(f: &mut Frame, area: Rect) {
    let now = Local::now();
    let time_str = format!("{}", now.format("%H%M")); // Remove colon

    // Build the 5 lines of the clock
    let mut lines: Vec<String> = vec![String::new(); 5];

    for (idx, ch) in time_str.chars().enumerate() {
        let digit_art = ascii_digits::get_digit(ch);
        for (i, line) in digit_art.iter().enumerate() {
            lines[i].push_str(line);
            // Add spacing: 1 space within groups, 2 spaces between hour and minute
            if idx == 0 {
                // After first hour digit
                lines[i].push(' ');
            } else if idx == 1 {
                // After second hour digit (between hours and minutes) - 2 spaces for separation
                lines[i].push_str("  ");
            } else if idx == 2 {
                // After first minute digit
                lines[i].push(' ');
            }
            // idx == 3 (last digit) gets no space
        }
    }

    // Create a layout to vertically center the clock
    // Calculate explicit padding (area is 7 lines, clock is 5 lines, so 2 lines to split)
    let clock_height = 5;
    let total_padding = area.height.saturating_sub(clock_height);
    let top_padding = total_padding / 2;
    let bottom_padding = total_padding - top_padding;

    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(top_padding),    // Top padding (explicit)
            Constraint::Length(clock_height),   // Clock (5 lines)
            Constraint::Length(bottom_padding), // Bottom padding (explicit)
        ])
        .split(area);

    let clock_text: Vec<Line> = lines
        .iter()
        .map(|line| Line::from(Span::styled(line, Style::default().fg(colors::FG))))
        .collect();

    f.render_widget(
        Paragraph::new(clock_text).alignment(Alignment::Center),
        v_chunks[1],
    );
}

fn ui(f: &mut Frame, app: &App) {
    let area = f.area();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(match app.mode {
            Mode::Timer => colors::MAGENTA,
            Mode::Pet => colors::CYAN,
            Mode::Stats => colors::GREEN,
            Mode::Debug => colors::RED,
        }))
        .style(Style::default().bg(colors::BG));

    f.render_widget(block, area);

    let inner = area.inner(Margin {
        horizontal: 1,
        vertical: 1,
    });

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Tabs
            Constraint::Length(1), // Spacer
            Constraint::Min(10),   // Content (minimum height)
            Constraint::Min(5),    // Clock area (takes remaining space, min 5 for clock)
            Constraint::Length(1), // Message/Help
        ])
        .split(inner);

    // Tabs
    let mut tab_spans = vec![
        Span::styled(
            "TIMER",
            if app.mode == Mode::Timer {
                Style::default().fg(colors::BG).bg(colors::MAGENTA).bold()
            } else {
                Style::default().fg(colors::COMMENT)
            },
        ),
        Span::raw(" "),
        Span::styled(
            "PET",
            if app.mode == Mode::Pet {
                Style::default().fg(colors::BG).bg(colors::CYAN).bold()
            } else {
                Style::default().fg(colors::COMMENT)
            },
        ),
        Span::raw(" "),
        Span::styled(
            "STATS",
            if app.mode == Mode::Stats {
                Style::default().fg(colors::BG).bg(colors::GREEN).bold()
            } else {
                Style::default().fg(colors::COMMENT)
            },
        ),
    ];

    // Only show DEBUG tab when in test mode
    if app.test_mode {
        tab_spans.push(Span::raw(" "));
        tab_spans.push(Span::styled(
            "DEBUG",
            if app.mode == Mode::Debug {
                Style::default().fg(colors::BG).bg(colors::RED).bold()
            } else {
                Style::default().fg(colors::COMMENT)
            },
        ));
    }

    let tabs = Line::from(tab_spans);
    f.render_widget(Paragraph::new(tabs).alignment(Alignment::Center), chunks[0]);

    match app.mode {
        Mode::Timer => render_timer(f, chunks[2], app),
        Mode::Pet => render_pet(f, chunks[2], app),
        Mode::Stats => render_stats(f, chunks[2], app),
        Mode::Debug => render_debug(f, chunks[2], app),
    }

    // Large ASCII Clock
    render_large_clock(f, chunks[3]);

    // Message or help
    let bottom_text = if let Some((msg, _)) = &app.message {
        Line::from(Span::styled(msg, Style::default().fg(colors::YELLOW).bold()))
    } else {
        Line::from(vec![
            Span::styled("SPC", Style::default().fg(colors::YELLOW)),
            Span::styled(" go ", Style::default().fg(colors::COMMENT)),
            Span::styled("TAB", Style::default().fg(colors::YELLOW)),
            Span::styled(" tab ", Style::default().fg(colors::COMMENT)),
            Span::styled("q", Style::default().fg(colors::YELLOW)),
            Span::styled(" quit", Style::default().fg(colors::COMMENT)),
        ])
    };
    f.render_widget(
        Paragraph::new(bottom_text).alignment(Alignment::Center),
        chunks[4],
    );
}

fn render_timer(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Pet preview
            Constraint::Length(1), // State
            Constraint::Length(2), // Timer
            Constraint::Length(1), // Progress
            Constraint::Length(1), // XP bar
            Constraint::Min(0),    // Spacer
        ])
        .split(area);

    // Mini pet with optional speech
    let pet_color = if app.pomo_state == PomodoroState::Work {
        colors::RED
    } else if app.pomo_state == PomodoroState::Break {
        colors::GREEN
    } else {
        colors::CYAN
    };

    if let Some((speech, _)) = &app.pet_speech {
        render_pet_with_speech(f, chunks[0], app, speech, pet_color);
    } else {
        let pet_art = app.game.get_pet_art(app.frame / 2);
        let pet_text: Vec<Line> = pet_art
            .iter()
            .map(|line| {
                Line::from(Span::styled(*line, Style::default().fg(pet_color)))
            })
            .collect();
        f.render_widget(
            Paragraph::new(pet_text).alignment(Alignment::Center),
            chunks[0],
        );
    }

    // State
    let (state_text, state_color) = match app.pomo_state {
        PomodoroState::Work => ("FOCUSING", colors::RED),
        PomodoroState::Break => ("RESTING", colors::GREEN),
        PomodoroState::Paused => ("READY", colors::COMMENT),
    };
    f.render_widget(
        Paragraph::new(state_text)
            .style(Style::default().fg(state_color).bold())
            .alignment(Alignment::Center),
        chunks[1],
    );

    // Timer
    let mins = app.pomo_remaining.as_secs() / 60;
    let secs = app.pomo_remaining.as_secs() % 60;
    f.render_widget(
        Paragraph::new(format!("{:02}:{:02}", mins, secs))
            .style(Style::default().fg(colors::FG).bold())
            .alignment(Alignment::Center),
        chunks[2],
    );

    // Progress
    let progress = if app.pomo_total.as_secs() > 0 {
        1.0 - (app.pomo_remaining.as_secs_f64() / app.pomo_total.as_secs_f64())
    } else {
        0.0
    };
    f.render_widget(
        Gauge::default()
            .gauge_style(Style::default().fg(match app.pomo_state {
                PomodoroState::Work => colors::RED,
                PomodoroState::Break => colors::GREEN,
                PomodoroState::Paused => colors::COMMENT,
            }))
            .ratio(progress)
            .label(""),
        chunks[3],
    );

    // XP bar
    let xp_progress = app.game.xp as f64 / app.game.xp_to_next_level() as f64;
    f.render_widget(
        Gauge::default()
            .gauge_style(Style::default().fg(colors::YELLOW))
            .ratio(xp_progress)
            .label(format!("Lv.{}", app.game.level)),
        chunks[4],
    );
}

fn render_pet_with_speech(f: &mut Frame, area: Rect, app: &App, speech: &str, pet_color: Color) {
    // Create horizontal layout: left padding, pet, speech bubble, right padding
    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // Left padding
            Constraint::Length(10),     // Pet
            Constraint::Length(20),     // Speech bubble
            Constraint::Min(0),         // Right padding
        ])
        .split(area);

    // Render pet
    let pet_art = app.game.get_pet_art(app.frame / 2);
    let pet_text: Vec<Line> = pet_art
        .iter()
        .map(|line| {
            Line::from(Span::styled(*line, Style::default().fg(pet_color)))
        })
        .collect();
    f.render_widget(
        Paragraph::new(pet_text).alignment(Alignment::Center),
        h_chunks[1],
    );

    // Render speech bubble
    let bubble_border = "─".repeat(speech.len().min(18));
    let speech_trimmed = if speech.len() > 18 {
        format!("{}...", &speech[..15])
    } else {
        speech.to_string()
    };

    let bubble_text = vec![
        Line::from(format!("┌{}┐", bubble_border)),
        Line::from(format!("│{}│", speech_trimmed)),
        Line::from(format!("└{}┘", bubble_border)),
    ];

    f.render_widget(
        Paragraph::new(bubble_text)
            .style(Style::default().fg(colors::FG))
            .alignment(Alignment::Left),
        h_chunks[2],
    );
}

fn render_pet(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Name
            Constraint::Length(5), // Pet art + speech bubble
            Constraint::Length(1), // Stage
            Constraint::Length(1), // Level
            Constraint::Length(1), // XP
            Constraint::Length(1), // XP bar
            Constraint::Length(1), // Food
            Constraint::Length(1), // Food bar
            Constraint::Min(0),    // Spacer
        ])
        .split(area);

    // Name
    f.render_widget(
        Paragraph::new(format!("~ {} ~", app.game.pet_name))
            .style(Style::default().fg(colors::CYAN).bold())
            .alignment(Alignment::Center),
        chunks[0],
    );

    // Pet art with speech bubble
    if let Some((speech, _)) = &app.pet_speech {
        render_pet_with_speech(f, chunks[1], app, speech, match app.game.mood {
            PetMood::Working => colors::RED,
            PetMood::Happy => colors::YELLOW,
            PetMood::Resting => colors::GREEN,
            PetMood::Idle => colors::CYAN,
        });
    } else {
        let pet_art = app.game.get_pet_art(app.frame / 2);
        let pet_text: Vec<Line> = pet_art
            .iter()
            .map(|line| {
                Line::from(Span::styled(
                    *line,
                    Style::default().fg(match app.game.mood {
                        PetMood::Working => colors::RED,
                        PetMood::Happy => colors::YELLOW,
                        PetMood::Resting => colors::GREEN,
                        PetMood::Idle => colors::CYAN,
                    }),
                ))
            })
            .collect();
        f.render_widget(
            Paragraph::new(pet_text).alignment(Alignment::Center),
            chunks[1],
        );
    }

    // Stage + Type
    f.render_widget(
        Paragraph::new(format!("{} {}", app.game.pet_type.name(), app.game.stage_name()))
            .style(Style::default().fg(colors::MAGENTA))
            .alignment(Alignment::Center),
        chunks[2],
    );

    // Level
    f.render_widget(
        Paragraph::new(format!("Level {}", app.game.level))
            .style(Style::default().fg(colors::FG).bold())
            .alignment(Alignment::Center),
        chunks[3],
    );

    // XP
    f.render_widget(
        Paragraph::new(format!(
            "XP: {}/{}",
            app.game.xp,
            app.game.xp_to_next_level()
        ))
        .style(Style::default().fg(colors::YELLOW))
        .alignment(Alignment::Center),
        chunks[4],
    );

    // XP progress bar
    let xp_progress = app.game.xp as f64 / app.game.xp_to_next_level() as f64;
    f.render_widget(
        Gauge::default()
            .gauge_style(Style::default().fg(colors::YELLOW))
            .ratio(xp_progress)
            .label(""),
        chunks[5],
    );

    // Food
    f.render_widget(
        Paragraph::new(format!("Food: {}/100", app.game.food))
            .style(Style::default().fg(colors::GREEN))
            .alignment(Alignment::Center),
        chunks[6],
    );

    // Food bar
    let food_progress = app.game.food as f64 / 100.0;
    f.render_widget(
        Gauge::default()
            .gauge_style(Style::default().fg(if app.game.food > 30 {
                colors::GREEN
            } else if app.game.food > 10 {
                colors::YELLOW
            } else {
                colors::RED
            }))
            .ratio(food_progress)
            .label(""),
        chunks[7],
    );
}

fn render_stats(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Header
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // Today
            Constraint::Length(1), // Total sessions
            Constraint::Length(1), // Total time
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // Streak
            Constraint::Length(1), // Bonus
            Constraint::Min(0),    // Spacer
        ])
        .split(area);

    f.render_widget(
        Paragraph::new("Progress")
            .style(Style::default().fg(colors::GREEN).bold())
            .alignment(Alignment::Center),
        chunks[0],
    );

    f.render_widget(
        Paragraph::new(format!("Today: {} sessions", app.game.today_sessions))
            .style(Style::default().fg(colors::FG))
            .alignment(Alignment::Center),
        chunks[2],
    );

    f.render_widget(
        Paragraph::new(format!("Total: {}", app.game.total_sessions))
            .style(Style::default().fg(colors::FG))
            .alignment(Alignment::Center),
        chunks[3],
    );

    let hours = app.game.total_focus_mins / 60;
    let mins = app.game.total_focus_mins % 60;
    f.render_widget(
        Paragraph::new(format!("Time: {}h {}m", hours, mins))
            .style(Style::default().fg(colors::COMMENT))
            .alignment(Alignment::Center),
        chunks[4],
    );

    f.render_widget(
        Paragraph::new(format!("Streak: {} days", app.game.streak_days))
            .style(Style::default().fg(colors::YELLOW).bold())
            .alignment(Alignment::Center),
        chunks[6],
    );

    let bonus = (app.game.streak_days.min(7) * 5) as u32;
    f.render_widget(
        Paragraph::new(format!("+{} XP bonus", bonus))
            .style(Style::default().fg(colors::MAGENTA))
            .alignment(Alignment::Center),
        chunks[7],
    );
}

fn render_debug(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Header
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // Level/XP
            Constraint::Length(1), // Stage
            Constraint::Length(1), // Food
            Constraint::Length(1), // Streak
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // Controls header
            Constraint::Length(1), // Control 1
            Constraint::Length(1), // Control 2
            Constraint::Length(1), // Control 3
            Constraint::Length(1), // Control 4
            Constraint::Length(1), // Control 5
            Constraint::Length(1), // Control 6
            Constraint::Length(1), // Control 7
            Constraint::Min(0),    // Spacer
        ])
        .split(area);

    f.render_widget(
        Paragraph::new("Debug Panel")
            .style(Style::default().fg(colors::RED).bold())
            .alignment(Alignment::Center),
        chunks[0],
    );

    f.render_widget(
        Paragraph::new(format!(
            "Lv.{} | XP: {}/{}",
            app.game.level,
            app.game.xp,
            app.game.xp_to_next_level()
        ))
        .style(Style::default().fg(colors::FG))
        .alignment(Alignment::Center),
        chunks[2],
    );

    f.render_widget(
        Paragraph::new(format!("{} {} (Stg {})", app.game.pet_type.name(), app.game.stage_name(), app.game.evolution_stage()))
            .style(Style::default().fg(colors::CYAN))
            .alignment(Alignment::Center),
        chunks[3],
    );

    let food_text = if app.game.is_dead {
        "DEAD 💀".to_string()
    } else {
        format!("Food: {}/100", app.game.food)
    };
    f.render_widget(
        Paragraph::new(food_text)
            .style(Style::default().fg(if app.game.is_dead {
                colors::RED
            } else if app.game.food > 30 {
                colors::GREEN
            } else if app.game.food > 10 {
                colors::YELLOW
            } else {
                colors::RED
            }))
            .alignment(Alignment::Center),
        chunks[4],
    );

    f.render_widget(
        Paragraph::new(format!("Streak: {} | Sessions: {}", app.game.streak_days, app.game.total_sessions))
            .style(Style::default().fg(colors::YELLOW))
            .alignment(Alignment::Center),
        chunks[5],
    );

    f.render_widget(
        Paragraph::new("─ Controls ─")
            .style(Style::default().fg(colors::COMMENT))
            .alignment(Alignment::Center),
        chunks[7],
    );

    f.render_widget(
        Paragraph::new("1: +50 XP  2: +500 XP")
            .style(Style::default().fg(colors::GREEN))
            .alignment(Alignment::Center),
        chunks[8],
    );

    f.render_widget(
        Paragraph::new("3: +1 Lv   4: Evolve")
            .style(Style::default().fg(colors::MAGENTA))
            .alignment(Alignment::Center),
        chunks[9],
    );

    f.render_widget(
        Paragraph::new("5: +Streak 6: Pet")
            .style(Style::default().fg(colors::CYAN))
            .alignment(Alignment::Center),
        chunks[10],
    );

    f.render_widget(
        Paragraph::new("7: +Food 8: -Food")
            .style(Style::default().fg(colors::GREEN))
            .alignment(Alignment::Center),
        chunks[11],
    );

    f.render_widget(
        Paragraph::new("9: Kill/Revive")
            .style(Style::default().fg(if app.game.is_dead { colors::GREEN } else { colors::RED }))
            .alignment(Alignment::Center),
        chunks[12],
    );

    f.render_widget(
        Paragraph::new("0: RESET ALL")
            .style(Style::default().fg(colors::RED))
            .alignment(Alignment::Center),
        chunks[13],
    );
}
