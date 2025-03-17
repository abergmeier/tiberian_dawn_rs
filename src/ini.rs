pub struct Ini<'a> {
    /// root filename for scenario file to read
    root: &'a str,
}

pub trait IniName {
    fn INI_Name() -> &'static str;
}
