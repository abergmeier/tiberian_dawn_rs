#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_variables)]

use crate::{
    base::BaseClass, heap::TFixedIHeapClass, movie::Movies, team::TeamTypeClass, theme::ThemeClass,
};

const TEAMTYPE_MAX: usize = 40;

pub struct Scenario {
    movies: Movies,
    /// This is the list of BuildingTypes that define the AI's base.
    base: BaseClass,
    theme: ThemeClass,
    /// Used to be ScenarioCRC in C++ code
    //CRC: crc::Crc<u64>,
    pub TeamTypes: TFixedIHeapClass<TeamTypeClass>,
}

impl Scenario {
    pub fn new() -> Self {
        Self {
            TeamTypes: TFixedIHeapClass::with_capacity(TEAMTYPE_MAX),
            base: Default::default(),
            movies: Default::default(),
            theme: Default::default(),
        }
    }
}
