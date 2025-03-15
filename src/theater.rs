#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables
)]

use bitflags::bitflags;

/// The theaters of operation are as follows.
#[repr(u8)]
pub enum TheaterType {
    //THEATER_NONE=-1,
    THEATER_DESERT,
    THEATER_JUNGLE,
    THEATER_TEMPERATE,
    THEATER_WINTER,
    //THEATER_COUNT,
    //THEATER_FIRST=0
}

bitflags! {
    /// Theater flags
    pub struct THEATERF: u8 {
        const DESERT = 1 << TheaterType::THEATER_DESERT as u8;
        const JUNGLE = 1 << TheaterType::THEATER_JUNGLE as u8;
        const TEMPERATE = 1 << TheaterType::THEATER_TEMPERATE as u8;
        const WINTER = 1 << TheaterType::THEATER_WINTER as u8;
    }
}
