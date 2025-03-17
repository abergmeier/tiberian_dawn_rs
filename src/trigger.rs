use crate::ini::IniName;

pub struct TriggerClass {
}

impl IniName for TriggerClass {
    fn INI_Name() -> &'static str{
        "Triggers"
    }
}
