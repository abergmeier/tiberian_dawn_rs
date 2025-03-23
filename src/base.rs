#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables
)]

use crate::ini::IniName;

#[derive(Default)]
pub struct BaseClass {}

impl IniName for BaseClass {
    fn INI_Name() -> &'static str {
        "base"
    }
}
