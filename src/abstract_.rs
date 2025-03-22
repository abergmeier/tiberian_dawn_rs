#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_variables)]

use std::iter::zip;

use crate::text::IDs;

pub trait MatchesInternalControlName {
    fn matches_internal_control_name(&self, other_internal_control_name: &str) -> bool;
}

pub trait LookupByInternalControlName: MatchesInternalControlName {
    type TypeEnum: Copy;

    fn lookup_type_enum_variant_by_internal_control_name(
        internal_control_name: &str,
    ) -> Option<Self::TypeEnum>;
}

pub trait LookupByName {
    type TypeEnum: Copy;

    fn lookup_type_enum_variant_by_name(name: &str) -> Option<Self::TypeEnum>;
}

/// This is the abstract type class. It holds information common to all
/// objects that might exist. This contains the name of
/// the object type.
#[derive(Default)]
pub struct AbstractTypeClass {
    /// This is the internal control name of the object. This name does
    /// not change regardless of language specified. This is the name
    /// used in scenario control files and for other text based unique
    /// identification purposes.
    pub IniName: [char; 9],

    /// The translated (language specific) text name number of this object.
    /// This number is used to fetch the object's name from the language
    /// text file. Whenever the name of the object needs to be displayed,
    /// this is used to determine the text string.
    Name: Option<IDs>,
}

impl MatchesInternalControlName for AbstractTypeClass {
    fn matches_internal_control_name(&self, other_internal_control_name: &str) -> bool {
        for (lhs, rhs) in zip(self.IniName.iter(), other_internal_control_name.bytes()) {
            if *lhs != (rhs as char) {
                return false;
            }
        }
        true
    }
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
