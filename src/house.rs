#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables
)]

use bitflags::bitflags;

use crate::{player::PlayerColorType, text::IDs};

///	The houses that can be played are listed here. Each has their own
///	personality and strengths.
#[repr(u8)]
pub enum HousesType {
    //HOUSE_NONE=-1,
    HOUSE_GOOD,    // Global Defense Initiative
    HOUSE_BAD,     // Brotherhood of Nod
    HOUSE_NEUTRAL, // Civilians
    HOUSE_JP,      // Disaster Containment Team
    HOUSE_MULTI1,  // Multi-Player house #1
    HOUSE_MULTI2,  // Multi-Player house #2
    HOUSE_MULTI3,  // Multi-Player house #3
    HOUSE_MULTI4,  // Multi-Player house #4
    HOUSE_MULTI5,  // Multi-Player house #5
    HOUSE_MULTI6,  // Multi-Player house #6

                   //HOUSE_COUNT,
                   //HOUSE_FIRST=HOUSE_GOOD
}

bitflags! {
    pub struct Flags: u16 {
        const GOOD    = (1<<HousesType::HOUSE_GOOD as u16);
        const BAD     = (1<<HousesType::HOUSE_BAD as u16);
        const NEUTRAL = (1<<HousesType::HOUSE_NEUTRAL as u16);
        const JP      = (1<<HousesType::HOUSE_JP as u16);
        const MULTI1  = (1<<HousesType::HOUSE_MULTI1 as u16);
        const MULTI2  = (1<<HousesType::HOUSE_MULTI2 as u16);
        const MULTI3  = (1<<HousesType::HOUSE_MULTI3 as u16);
        const MULTI4  = (1<<HousesType::HOUSE_MULTI4 as u16);
        const MULTI5  = (1<<HousesType::HOUSE_MULTI5 as u16);
        const MULTI6  = (1<<HousesType::HOUSE_MULTI6 as u16);
    }
}

pub type HOUSEF = Flags;

/// Each house has certain unalienable characteristics. This structure
/// elaborates these.
pub struct HouseTypeClass {
    /// This is the house number (enum). This is a unique identification
    /// number for the house.
    House: HousesType,

    /// The INI name of the house is pointed to by this element. This is the
    /// identification name used in the scenario INI file.
    IniName: &'static str,

    /// The full name (translated) of the house is identified by this number.
    /// The actual text of the name is located in a text file loaded at run
    /// time.
    FullName: IDs,

    ///
    /// This is the filename suffix to use when creating a house specific
    /// file name. It is three characters long.
    Suffix: [char; 4],

    /// This is the "lemon percentage" to use when determining if a particular
    /// object owned by this house is to be flagged as a "lemon". Objects so
    /// flagged have a greater break-down chance. The percentage is expressed
    /// as a fixed point number with 0x000 meaning 0% and 0x100 meaning 100%.
    Lemon: u8,

    /// Each house is assigned a unique identification color to be used on the
    /// radar map and other color significant areas.
    Color: u8,

    BrightColor: u8,

    /// This points to the default remap table for this house.
    RemapTable: [u8; 256],
    RemapColor: PlayerColorType,

    /// This is a unique ASCII character used when constructing filenames. It
    /// serves a similar purpose as the "Suffix" element, but is only one
    /// character long.
    Prefix: char,
}

impl HouseTypeClass {
    /// This is the constructor for house type objects. This object holds the constant data
    /// for the house type.
    /// INPUT:   house    -- The ID number for this house type.
    ///          ini      -- The INI name of this house.
    ///          fullname -- The text number representing the complete name of the house.
    ///          ext      -- The filename extension used when loading data files.
    ///          lemon    -- The percentage for objects of this ownership to be lemon.
    ///          remapc   -- The remap color number to use.
    ///          color    -- The radar color to use for this "house".
    ///          prefix   -- A unique prefix letter used when building custom filenames.
    pub const fn new(
        house: HousesType,
        ini: &'static str,
        fullname: IDs,
        ext: [char; 3],
        lemon: u8,
        color: u8,
        bright_color: u8,
        remapcolor: PlayerColorType,
        remap: [u8; 256],
        prefix: char,
    ) -> Self {
        Self {
            RemapTable: remap,
            RemapColor: remapcolor,
            House: house,
            IniName: ini,
            FullName: fullname,
            Suffix: [ext[0], ext[1], ext[2], '\0'],
            Lemon: lemon,
            Color: color,
            BrightColor: bright_color,
            Prefix: prefix,
        }
    }
}
