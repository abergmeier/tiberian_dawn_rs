use crate::ini::IniName;

pub struct BaseClass {
}

impl IniName for BaseClass {
    fn INI_Name() -> &'static str {
        "base"
    }
}
