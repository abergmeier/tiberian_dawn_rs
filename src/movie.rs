#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_variables)]

use ini::Ini;

/// This contains the movie names to use for mission briefing, winning, and losing
/// sequences. They are read from the INI file.
pub struct Movies {
    /// ActionMovie in C++ code
    action: String,
    /// BriefMovie in C++ code
    brief: String,
    /// IntroMovie in C++ code
    intro: String,
    /// LoseMovie in C++ code
    lose: String,
    /// WinMovie in C++ code
    win: String,
}

impl Default for Movies {
    fn default() -> Self {
        Self {
            action: String::from("x"),
            brief: String::from("x"),
            intro: String::from("x"),
            lose: String::from("x"),
            win: String::from("x"),
        }
    }
}

impl TryFrom<Ini> for Movies {
    type Error = std::fmt::Error;

    fn try_from(ini: Ini) -> Result<Self, Self::Error> {
        match ini.section(Some("Basic")) {
            None => Ok(Default::default()),
            Some(section) => Ok(Self {
                action: section
                    .get("Action")
                    .map_or(String::from("x"), str_to_string),
                brief: section
                    .get("Brief")
                    .map_or(String::from("x"), str_to_string),
                intro: section
                    .get("Intro")
                    .map_or(String::from("x"), str_to_string),
                lose: section.get("Lose").map_or(String::from("x"), str_to_string),
                win: section.get("Win").map_or(String::from("x"), str_to_string),
            }),
        }
    }
}

fn str_to_string(s: &str) -> String {
    s.to_string()
}
