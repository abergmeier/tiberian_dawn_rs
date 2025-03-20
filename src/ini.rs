#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_variables)]

pub struct Ini<'a> {
    /// root filename for scenario file to read
    root: &'a str,
}

pub trait IniName {
    fn INI_Name() -> &'static str;
}

pub struct IniProfile {
    pub ini: ini::Ini,
    pub root: String,
}
