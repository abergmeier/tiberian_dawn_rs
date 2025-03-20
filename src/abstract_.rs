#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_variables)]

use crate::text::IDs;

/// This is the abstract type class. It holds information common to all
/// objects that might exist. This contains the name of
/// the object type.
#[derive(Default)]
pub struct AbstractTypeClass {
    /// This is the internal control name of the object. This name does
    /// not change regardless of language specified. This is the name
    /// used in scenario control files and for other text based unique
    /// identification purposes.
    IniName: [char; 9],

    /// The translated (language specific) text name number of this object.
    /// This number is used to fetch the object's name from the language
    /// text file. Whenever the name of the object needs to be displayed,
    /// this is used to determine the text string.
    Name: Option<IDs>,
}

impl AbstractTypeClass {
    pub const fn new(name: Option<IDs>, ini: &str) -> Self {
        let mut ini_name = ['\0'; 9];
        let ini_bytes = ini.as_bytes();
        let len = if ini_bytes.len() < 9 {
            ini_bytes.len()
        } else {
            8
        };

        let mut i = 0;
        while i < len {
            ini_name[i] = ini_bytes[i] as char;
            i += 1;
        }

        Self {
            Name: name,
            IniName: ini_name,
        }
    }
}
