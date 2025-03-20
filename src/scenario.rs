#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_variables)]

use crate::{
    base::BaseClass, heap::TFixedIHeapClass, movie::Movies, team::TeamTypeClass,
    theme::ThemeClass,
};

pub struct Scenario {
    movies: Movies,
    /// This is the list of BuildingTypes that define the AI's base.
    base: BaseClass,
    theme: ThemeClass,
    /// Used to be ScenarioCRC in C++ code
    //CRC: crc::Crc<u64>,
    pub TeamTypes: TFixedIHeapClass<TeamTypeClass>,
}
