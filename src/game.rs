use std::fs;
use std::path::PathBuf;
use std::process::Command;

use chrono::{Local, NaiveDate, Timelike};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::pets;

// Pet types
#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum PetType {
    Blob,
    Cat,
    Robot,
    Ghost,
}

impl PetType {
    pub fn random() -> Self {
        match rand::thread_rng().gen_range(0..4) {
            0 => PetType::Blob,
            1 => PetType::Cat,
            2 => PetType::Robot,
            _ => PetType::Ghost,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            PetType::Blob => "Blob",
            PetType::Cat => "Cat",
            PetType::Robot => "Robot",
            PetType::Ghost => "Ghost",
        }
    }
}

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum PetMood {
    Idle,
    Working,
    Happy,
    Resting,
}

#[derive(Serialize, Deserialize)]
pub struct GameData {
    // XP and leveling
    pub xp: u32,
    pub level: u32,
    // Stats
    pub total_sessions: u32,
    pub total_focus_mins: u32,
    pub last_session_date: Option<NaiveDate>,
    pub streak_days: u32,
    pub today_sessions: u32,
    pub today_date: Option<NaiveDate>,
    // Pet
    pub pet_name: String,
    pub pet_type: PetType,
    pub mood: PetMood,
    // Food system (0-100)
    pub food: u32,
    #[serde(default)]
    pub last_food_check: Option<i64>, // Unix timestamp
    #[serde(default)]
    pub hunger_zero_since: Option<i64>, // When food hit 0
    #[serde(default)]
    pub is_dead: bool,
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

    pub fn load() -> Self {
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

    pub fn save(&self) {
        let path = Self::data_path();
        if let Ok(data) = serde_json::to_string_pretty(self) {
            fs::write(path, data).ok();
        }
    }

    pub fn xp_for_level(level: u32) -> u32 {
        // XP needed: 100, 150, 225, 337, ...
        (100.0 * 1.5_f64.powi((level - 1) as i32)) as u32
    }

    pub fn xp_to_next_level(&self) -> u32 {
        Self::xp_for_level(self.level)
    }

    pub fn add_xp(&mut self, amount: u32) {
        self.xp += amount;
        while self.xp >= self.xp_to_next_level() {
            self.xp -= self.xp_to_next_level();
            self.level += 1;
        }
    }

    pub fn record_session(&mut self) {
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

    pub fn evolution_stage(&self) -> u32 {
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

    pub fn get_pet_art(&self, frame: usize) -> &'static [&'static str] {
        if self.is_dead {
            pets::get_dead_art(self.pet_type)
        } else {
            pets::get_art(self.pet_type, self.evolution_stage(), self.mood, frame)
        }
    }

    pub fn stage_name(&self) -> &'static str {
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

    pub fn hunger_cry(&self) -> &'static str {
        match self.pet_type {
            PetType::Blob => "I'm so hungry... please help me! I need food! 😢",
            PetType::Cat => "Meow... *weak* ...please... food... I'm starving... 🐱",
            PetType::Robot => "CRITICAL: Energy depleted... shutting down... need fuel... 🤖",
            PetType::Ghost => "I'm fading away... so hungry... please don't let me disappear... 👻",
        }
    }

    pub fn death_cry(&self) -> &'static str {
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

    pub fn update_food(&mut self) {
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

    pub fn feed(&mut self, amount: u32) {
        self.food = (self.food + amount).min(100);
        if self.food > 0 {
            self.hunger_zero_since = None;
        }
    }
}
