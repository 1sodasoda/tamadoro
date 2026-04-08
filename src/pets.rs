// Pet evolution stages with ASCII art
use crate::game::{PetMood, PetType};

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

pub fn get_art(pet_type: PetType, stage: u32, mood: PetMood, frame: usize) -> &'static [&'static str] {
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
