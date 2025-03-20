#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_variables)]

use crate::color::*;
use crate::house::HouseTypeClass;
use crate::house::HousesType::*;
use crate::player::PlayerColorType::*;
use crate::text::IDs::*;

/// These are the colors used to identify the various owners.

const COLOR_GOOD: u8 = 180; // GOLD
const COLOR_BRIGHT_GOOD: u8 = 176; // GOLD
const COLOR_BAD: u8 = 123; //RED;
const COLOR_BRIGHT_BAD: u8 = 127; //RED;
const COLOR_NEUTRAL: u8 = 205; //WHITE;
const COLOR_BRIGHT_NEUTRAL: u8 = 202; //WHITE;

const HouseGood: HouseTypeClass = HouseTypeClass::new(
    HOUSE_GOOD,
    "GoodGuy",         //	NAME:			House name.
    TXT_GDI,           // FULLNAME:	Translated house name.
    ['G', 'D', 'I'],   // SUFFIX:		House file suffix.
    0,                 // LEMON:		Lemon vehicle frequency.
    COLOR_GOOD,        // COLOR:		Dark Radar map color.
    COLOR_BRIGHT_GOOD, // COLOR:		Bright Radar map color.
    REMAP_YELLOW,      // Remap color ID number.
    RemapYellow,       // Default remap table.
    'G',               // VOICE:		Voice prefix character.
);

const HouseBad: HouseTypeClass = HouseTypeClass::new(
    HOUSE_BAD,
    "BadGuy",         //	NAME:			House name.
    TXT_NOD,          // FULLNAME:	Translated house name.
    ['N', 'O', 'D'],  // SUFFIX:		House file suffix.
    0,                // LEMON:		Lemon vehicle frequency.
    COLOR_BAD,        // COLOR:		Dark Radar map color.
    COLOR_BRIGHT_BAD, // COLOR:		Bright Radar map color.
    REMAP_BLUE,       // Remap color ID number.
    RemapBlue,        // Default remap table.
    'B',              // VOICE:		Voice prefix character.
);

const HouseCivilian: HouseTypeClass = HouseTypeClass::new(
    HOUSE_NEUTRAL,
    "Neutral",            //	NAME:			House name.
    TXT_CIVILIAN,         // FULLNAME:	Translated house name.
    ['C', 'I', 'V'],      // SUFFIX:		House file suffix.
    0,                    // LEMON:		Lemon vehicle frequency.
    COLOR_NEUTRAL,        // COLOR:		Dark Radar map color.
    COLOR_BRIGHT_NEUTRAL, // COLOR:		Bright Radar map color.
    REMAP_YELLOW,         // Remap color ID number.
    RemapNone,            // Default remap table.
    'C',                  // VOICE:		Voice prefix character.
);

const HouseJP: HouseTypeClass = HouseTypeClass::new(
    HOUSE_JP,
    "Special",            //	NAME:			House name.
    TXT_JP,               // FULLNAME:	Translated house name.
    ['J', 'P', '\0'],     // SUFFIX:		House file suffix.
    0,                    // LEMON:		Lemon vehicle frequency.
    COLOR_NEUTRAL,        // COLOR:		Dark Radar map color.
    COLOR_BRIGHT_NEUTRAL, // COLOR:		Bright Radar map color.
    REMAP_YELLOW,         // Remap color ID number.
    RemapNone,            // Default remap table.
    'J',                  // VOICE:		Voice prefix character.
);

const HouseMulti1: HouseTypeClass = HouseTypeClass::new(
    HOUSE_MULTI1,
    "Multi1",             //	NAME:			House name.
    TXT_CIVILIAN,         // FULLNAME:	Translated house name.
    ['M', 'P', '1'],      // SUFFIX:		House file suffix.
    0,                    // LEMON:		Lemon vehicle frequency.
    COLOR_NEUTRAL,        // COLOR:		Radar map color.
    COLOR_BRIGHT_NEUTRAL, // COLOR:		Bright Radar map color.
    REMAP_AQUA,           // Remap color ID number.
    RemapBlueGreen,       // Default remap table.
    'M',                  // VOICE:		Voice prefix character.
);

const HouseMulti2: HouseTypeClass = HouseTypeClass::new(
    HOUSE_MULTI2,
    "Multi2",             //	NAME:			House name.
    TXT_CIVILIAN,         // FULLNAME:	Translated house name.
    ['M', 'P', '2'],      // SUFFIX:		House file suffix.
    0,                    // LEMON:		Lemon vehicle frequency.
    COLOR_NEUTRAL,        // COLOR:		Radar map color.
    COLOR_BRIGHT_NEUTRAL, // COLOR:		Bright Radar map color.
    REMAP_ORANGE,         // Remap color ID number.
    RemapOrange,          // Default remap table.
    'M',                  // VOICE:		Voice prefix character.
);

const HouseMulti3: HouseTypeClass = HouseTypeClass::new(
    HOUSE_MULTI3,
    "Multi3",             //	NAME:			House name.
    TXT_CIVILIAN,         // FULLNAME:	Translated house name.
    ['M', 'P', '3'],      // SUFFIX:		House file suffix.
    0,                    // LEMON:		Lemon vehicle frequency.
    COLOR_NEUTRAL,        // COLOR:		Radar map color.
    COLOR_BRIGHT_NEUTRAL, // COLOR:		Bright Radar map color.
    REMAP_GREEN,          // Remap color ID number.
    RemapGreen,           // Default remap table.
    'M',                  // VOICE:		Voice prefix character.
);

const HouseMulti4: HouseTypeClass = HouseTypeClass::new(
    HOUSE_MULTI4,
    "Multi4",             //	NAME:			House name.
    TXT_CIVILIAN,         // FULLNAME:	Translated house name.
    ['M', 'P', '4'],      // SUFFIX:		House file suffix.
    0,                    // LEMON:		Lemon vehicle frequency.
    COLOR_NEUTRAL,        // COLOR:		Radar map color.
    COLOR_BRIGHT_NEUTRAL, // COLOR:		Bright Radar map color.
    REMAP_BLUE,           // Remap color ID number.
    RemapBlue,            // Default remap table.
    'M',                  // VOICE:		Voice prefix character.
);

const HouseMulti5: HouseTypeClass = HouseTypeClass::new(
    HOUSE_MULTI5,
    "Multi5",             //	NAME:			House name.
    TXT_CIVILIAN,         // FULLNAME:	Translated house name.
    ['M', 'P', '5'],      // SUFFIX:		House file suffix.
    0,                    // LEMON:		Lemon vehicle frequency.
    COLOR_NEUTRAL,        // COLOR:		Radar map color.
    COLOR_BRIGHT_NEUTRAL, // COLOR:		Bright Radar map color.
    REMAP_YELLOW,         // Remap color ID number.
    RemapYellow,          // Default remap table.
    'M',                  // VOICE:		Voice prefix character.
);

const HouseMulti6: HouseTypeClass = HouseTypeClass::new(
    HOUSE_MULTI6,
    "Multi6",             //	NAME:			House name.
    TXT_CIVILIAN,         // FULLNAME:	Translated house name.
    ['M', 'P', '6'],      // SUFFIX:		House file suffix.
    0,                    // LEMON:		Lemon vehicle frequency.
    COLOR_NEUTRAL,        // COLOR:		Radar map color.
    COLOR_BRIGHT_NEUTRAL, // COLOR:		Bright Radar map color.
    REMAP_RED,            // Remap color ID number.
    RemapRed,             // Default remap table.
    'M',                  // VOICE:		Voice prefix character.
);
