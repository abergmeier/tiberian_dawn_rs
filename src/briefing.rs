#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_variables)]

use ini::Ini;

use crate::ini::IniProfile;

pub struct Briefing {
    Text: String,
}

pub enum TryFromError {
    IniError(ini::Error),
}

impl TryFrom<IniProfile> for Briefing {
    type Error = TryFromError;

    fn try_from(ini: IniProfile) -> Result<Self, Self::Error> {
        Ok(Self {
            Text: {
                match ini.ini.section(Some("Briefing")) {
                    None => String::new(),
                    Some(properties) => {
                        let value = Self::read_text_from_properties(properties);
                        if value.is_empty() {
                            // If the briefing text could not be found in the INI file, then search
                            // the mission.ini file.
                            Self::read_text_from_mission_properties(&ini)
                                .map_err(|err| TryFromError::IniError(err))?
                        } else {
                            value
                        }
                    }
                }
            },
        })
    }
}

impl Briefing {
    fn read_text_from_properties(props: &ini::Properties) -> String {
        // Read in any briefing text.
        let mut briefing_text = String::new();

        // Build the full text of the mission objective.
        for i in 1.. {
            match props.get(i.to_string()) {
                None => break,
                Some(s) => {
                    if s.is_empty() {
                        break;
                    }
                    briefing_text += " ";
                    briefing_text += s;
                }
            }
        }

        briefing_text
    }

    fn read_text_from_mission_properties(ini: &IniProfile) -> Result<String, ini::Error> {
        let mission_ini = Ini::load_from_file("MISSION.INI")?;

        match mission_ini.section(Some(&ini.root)) {
            None => Ok(String::new()),
            Some(props) => {
                let mut briefing_text = String::new();
                // Build the full text of the mission objective.
                for i in 1.. {
                    match props.get(i.to_string()) {
                        None => break,
                        Some(s) => {
                            if s.is_empty() {
                                break;
                            }

                            briefing_text += " ";
                            briefing_text += s;
                        }
                    }
                }
                Ok(briefing_text)
            }
        }
    }
}
