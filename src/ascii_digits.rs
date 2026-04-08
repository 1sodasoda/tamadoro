// Large ASCII digits for clock display (tty-clock style, all 7 chars wide)

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
