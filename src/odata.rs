#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables
)]

use crate::overlay::OverlayType::*;
use crate::overlay::OverlayTypeClass;
use crate::terrain::LandType::*;
use crate::text::IDs::*;

const Road: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_ROAD, // Overlay type number.
    "ROAD",       // INI name of overlay.
    TXT_CONCRETE, // Full name of overlay.
    LAND_ROAD,    // What kind of ground is it?
    0,            // If this is a wall, how many damage levels?
    0,            // If this is a wall, how many damage points can it take per level?
    true,         // Visible on the radar map?
    false,        // Is it a wooden overlay (affected by fire)?
    false,        // Targetable as a destroyable overlay?
    false,        // Crushable by tracked vehicle?
    false,        // Is this harvestable Tiberium?
    false,        // Stops low level bullets in flight?
    false,        // Theater specific art?
    false,        // Is this a wall type?
    false,        // Is this a crate?
);
const Concrete: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_CONCRETE, // Overlay type number.
    "CONC",           // INI name of overlay.
    TXT_CONCRETE,     // Full name of overlay.
    LAND_ROAD,        // What kind of ground is it?
    0,                // If this is a wall, how many damage levels?
    0,                // If this is a wall, how many damage points can it take per level?
    true,             // Visible on the radar map?
    false,            // Is it a wooden overlay (affected by fire)?
    false,            // Targetable as a destroyable overlay?
    false,            // Crushable by tracked vehicle?
    false,            // Is this harvestable Tiberium?
    false,            // Stops low level bullets in flight?
    false,            // Theater specific art?
    false,            // Is this a wall type?
    false,            // Is this a crate?
);
const Sandbag: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_SANDBAG_WALL, // Overlay type number.
    "SBAG",               // INI name of overlay.
    TXT_SANDBAG_WALL,     // Full name of overlay.
    LAND_WALL,            // What kind of ground is it?
    1,                    // If this is a wall, how many damage levels?
    20,                   // If this is a wall, how many damage points can it take per level?
    true,                 // Visible on the radar map?
    false,                // Is it a wooden overlay (affected by fire)?
    true,                 // Targetable as a destroyable overlay?
    false,                // Crushable by tracked vehicle?
    false,                // Is this harvestable Tiberium?
    false,                // Stops low level bullets in flight?
    false,                // Theater specific art?
    true,                 // Is this a wall type?
    false,                // Is this a crate?
);
const Cyclone: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_CYCLONE_WALL, // Overlay type number.
    "CYCL",               // INI name of overlay.
    TXT_CYCLONE_WALL,     // Full name of overlay.
    LAND_WALL,            // What kind of ground is it?
    2,                    // If this is a wall, how many damage levels?
    10,                   // If this is a wall, how many damage points can it take per level?
    true,                 // Visible on the radar map?
    false,                // Is it a wooden overlay (affected by fire)?
    true,                 // Targetable as a destroyable overlay?
    true,                 // Crushable by tracked vehicle?
    false,                // Is this harvestable Tiberium?
    false,                // Stops low level bullets in flight?
    false,                // Theater specific art?
    true,                 // Is this a wall type?
    false,                // Is this a crate?
);
const Brick: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_BRICK_WALL, // Overlay type number.
    "BRIK",             // INI name of overlay.
    TXT_BRICK_WALL,     // Full name of overlay.
    LAND_WALL,          // What kind of ground is it?
    3,                  // If this is a wall, how many damage levels?
    70,                 // If this is a wall, how many damage points can it take per level?
    true,               // Visible on the radar map?
    false,              // Is it a wooden overlay (affected by fire)?
    true,               // Targetable as a destroyable overlay?
    false,              // Crushable by tracked vehicle?
    false,              // Is this harvestable Tiberium?
    true,               // Stops low level bullets in flight?
    false,              // Theater specific art?
    true,               // Is this a wall type?
    false,              // Is this a crate?
);
const Barbwire: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_BARBWIRE_WALL, // Overlay type number.
    "BARB",                // INI name of overlay.
    TXT_BARBWIRE_WALL,     // Full name of overlay.
    LAND_WALL,             // What kind of ground is it?
    1,                     // If this is a wall, how many damage levels?
    2,                     // If this is a wall, how many damage points can it take per level?
    true,                  // Visible on the radar map?
    false,                 // Is it a wooden overlay (affected by fire)?
    true,                  // Targetable as a destroyable overlay?
    true,                  // Crushable by tracked vehicle?
    false,                 // Is this harvestable Tiberium?
    false,                 // Stops low level bullets in flight?
    false,                 // Theater specific art?
    true,                  // Is this a wall type?
    false,                 // Is this a crate?
);
const Wood: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_WOOD_WALL, // Overlay type number.
    "WOOD",            // INI name of overlay.
    TXT_WOOD_WALL,     // Full name of overlay.
    LAND_WALL,         // What kind of ground is it?
    1,                 // If this is a wall, how many damage levels?
    2,                 // If this is a wall, how many damage points can it take per level?
    true,              // Visible on the radar map?
    true,              // Is it a wooden overlay (affected by fire)?
    true,              // Targetable as a destroyable overlay?
    true,              // Crushable by tracked vehicle?
    false,             // Is this harvestable Tiberium?
    false,             // Stops low level bullets in flight?
    false,             // Theater specific art?
    true,              // Is this a wall type?
    false,             // Is this a crate?
);
const Tiberium1: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_TIBERIUM1, // Overlay type number.
    "TI1",             // INI name of overlay.
    TXT_TIBERIUM,      // Full name of overlay.
    LAND_TIBERIUM,     // What kind of ground is it?
    0,                 // If this is a wall, how many damage levels?
    0,                 // If this is a wall, how many damage points can it take per level?
    true,              // Visible on the radar map?
    false,             // Is it a wooden overlay (affected by fire)?
    false,             // Targetable as a destroyable overlay?
    false,             // Crushable by tracked vehicle?
    true,              // Is this harvestable Tiberium?
    false,             // Stops low level bullets in flight?
    true,              // Theater specific art?
    false,             // Is this a wall type?
    false,             // Is this a crate?
);
const Tiberium2: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_TIBERIUM2, // Overlay type number.
    "TI2",             // INI name of overlay.
    TXT_TIBERIUM,      // Full name of overlay.
    LAND_TIBERIUM,     // What kind of ground is it?
    0,                 // If this is a wall, how many damage levels?
    0,                 // If this is a wall, how many damage points can it take per level?
    true,              // Visible on the radar map?
    false,             // Is it a wooden overlay (affected by fire)?
    false,             // Targetable as a destroyable overlay?
    false,             // Crushable by tracked vehicle?
    true,              // Is this harvestable Tiberium?
    false,             // Stops low level bullets in flight?
    true,              // Theater specific art?
    false,             // Is this a wall type?
    false,             // Is this a crate?
);
const Tiberium3: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_TIBERIUM3, // Overlay type number.
    "TI3",             // INI name of overlay.
    TXT_TIBERIUM,      // Full name of overlay.
    LAND_TIBERIUM,     // What kind of ground is it?
    0,                 // If this is a wall, how many damage levels?
    0,                 // If this is a wall, how many damage points can it take per level?
    true,              // Visible on the radar map?
    false,             // Is it a wooden overlay (affected by fire)?
    false,             // Targetable as a destroyable overlay?
    false,             // Crushable by tracked vehicle?
    true,              // Is this harvestable Tiberium?
    false,             // Stops low level bullets in flight?
    true,              // Theater specific art?
    false,             // Is this a wall type?
    false,             // Is this a crate?
);
const Tiberium4: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_TIBERIUM4, // Overlay type number.
    "TI4",             // INI name of overlay.
    TXT_TIBERIUM,      // Full name of overlay.
    LAND_TIBERIUM,     // What kind of ground is it?
    0,                 // If this is a wall, how many damage levels?
    0,                 // If this is a wall, how many damage points can it take per level?
    true,              // Visible on the radar map?
    false,             // Is it a wooden overlay (affected by fire)?
    false,             // Targetable as a destroyable overlay?
    false,             // Crushable by tracked vehicle?
    true,              // Is this harvestable Tiberium?
    false,             // Stops low level bullets in flight?
    true,              // Theater specific art?
    false,             // Is this a wall type?
    false,             // Is this a crate?
);
const Tiberium5: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_TIBERIUM5, // Overlay type number.
    "TI5",             // INI name of overlay.
    TXT_TIBERIUM,      // Full name of overlay.
    LAND_TIBERIUM,     // What kind of ground is it?
    0,                 // If this is a wall, how many damage levels?
    0,                 // If this is a wall, how many damage points can it take per level?
    true,              // Visible on the radar map?
    false,             // Is it a wooden overlay (affected by fire)?
    false,             // Targetable as a destroyable overlay?
    false,             // Crushable by tracked vehicle?
    true,              // Is this harvestable Tiberium?
    false,             // Stops low level bullets in flight?
    true,              // Theater specific art?
    false,             // Is this a wall type?
    false,             // Is this a crate?
);
const Tiberium6: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_TIBERIUM6, // Overlay type number.
    "TI6",             // INI name of overlay.
    TXT_TIBERIUM,      // Full name of overlay.
    LAND_TIBERIUM,     // What kind of ground is it?
    0,                 // If this is a wall, how many damage levels?
    0,                 // If this is a wall, how many damage points can it take per level?
    true,              // Visible on the radar map?
    false,             // Is it a wooden overlay (affected by fire)?
    false,             // Targetable as a destroyable overlay?
    false,             // Crushable by tracked vehicle?
    true,              // Is this harvestable Tiberium?
    false,             // Stops low level bullets in flight?
    true,              // Theater specific art?
    false,             // Is this a wall type?
    false,             // Is this a crate?
);
const Tiberium7: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_TIBERIUM7, // Overlay type number.
    "TI7",             // INI name of overlay.
    TXT_TIBERIUM,      // Full name of overlay.
    LAND_TIBERIUM,     // What kind of ground is it?
    0,                 // If this is a wall, how many damage levels?
    0,                 // If this is a wall, how many damage points can it take per level?
    true,              // Visible on the radar map?
    false,             // Is it a wooden overlay (affected by fire)?
    false,             // Targetable as a destroyable overlay?
    false,             // Crushable by tracked vehicle?
    true,              // Is this harvestable Tiberium?
    false,             // Stops low level bullets in flight?
    true,              // Theater specific art?
    false,             // Is this a wall type?
    false,             // Is this a crate?
);
const Tiberium8: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_TIBERIUM8, // Overlay type number.
    "TI8",             // INI name of overlay.
    TXT_TIBERIUM,      // Full name of overlay.
    LAND_TIBERIUM,     // What kind of ground is it?
    0,                 // If this is a wall, how many damage levels?
    0,                 // If this is a wall, how many damage points can it take per level?
    true,              // Visible on the radar map?
    false,             // Is it a wooden overlay (affected by fire)?
    false,             // Targetable as a destroyable overlay?
    false,             // Crushable by tracked vehicle?
    true,              // Is this harvestable Tiberium?
    false,             // Stops low level bullets in flight?
    true,              // Theater specific art?
    false,             // Is this a wall type?
    false,             // Is this a crate?
);
const Tiberium9: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_TIBERIUM9, // Overlay type number.
    "TI9",             // INI name of overlay.
    TXT_TIBERIUM,      // Full name of overlay.
    LAND_TIBERIUM,     // What kind of ground is it?
    0,                 // If this is a wall, how many damage levels?
    0,                 // If this is a wall, how many damage points can it take per level?
    true,              // Visible on the radar map?
    false,             // Is it a wooden overlay (affected by fire)?
    false,             // Targetable as a destroyable overlay?
    false,             // Crushable by tracked vehicle?
    true,              // Is this harvestable Tiberium?
    false,             // Stops low level bullets in flight?
    true,              // Theater specific art?
    false,             // Is this a wall type?
    false,             // Is this a crate?
);
const Tiberium10: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_TIBERIUM10, // Overlay type number.
    "TI10",             // INI name of overlay.
    TXT_TIBERIUM,       // Full name of overlay.
    LAND_TIBERIUM,      // What kind of ground is it?
    0,                  // If this is a wall, how many damage levels?
    0,                  // If this is a wall, how many damage points can it take per level?
    true,               // Visible on the radar map?
    false,              // Is it a wooden overlay (affected by fire)?
    false,              // Targetable as a destroyable overlay?
    false,              // Crushable by tracked vehicle?
    true,               // Is this harvestable Tiberium?
    false,              // Stops low level bullets in flight?
    true,               // Theater specific art?
    false,              // Is this a wall type?
    false,              // Is this a crate?
);
const Tiberium11: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_TIBERIUM11, // Overlay type number.
    "TI11",             // INI name of overlay.
    TXT_TIBERIUM,       // Full name of overlay.
    LAND_TIBERIUM,      // What kind of ground is it?
    0,                  // If this is a wall, how many damage levels?
    0,                  // If this is a wall, how many damage points can it take per level?
    true,               // Visible on the radar map?
    false,              // Is it a wooden overlay (affected by fire)?
    false,              // Targetable as a destroyable overlay?
    false,              // Crushable by tracked vehicle?
    true,               // Is this harvestable Tiberium?
    false,              // Stops low level bullets in flight?
    true,               // Theater specific art?
    false,              // Is this a wall type?
    false,              // Is this a crate?
);
const Tiberium12: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_TIBERIUM12, // Overlay type number.
    "TI12",             // INI name of overlay.
    TXT_TIBERIUM,       // Full name of overlay.
    LAND_TIBERIUM,      // What kind of ground is it?
    0,                  // If this is a wall, how many damage levels?
    0,                  // If this is a wall, how many damage points can it take per level?
    true,               // Visible on the radar map?
    false,              // Is it a wooden overlay (affected by fire)?
    false,              // Targetable as a destroyable overlay?
    false,              // Crushable by tracked vehicle?
    true,               // Is this harvestable Tiberium?
    false,              // Stops low level bullets in flight?
    true,               // Theater specific art?
    false,              // Is this a wall type?
    false,              // Is this a crate?
);
const Squish: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_SQUISH, // Overlay type number.
    "SQUISH",       // INI name of overlay.
    TXT_SQUISH,     // Full name of overlay.
    LAND_CLEAR,     // What kind of ground is it?
    0,              // If this is a wall, how many damage levels?
    0,              // If this is a wall, how many damage points can it take per level?
    false,          // Visible on the radar map?
    false,          // Is it a wooden overlay (affected by fire)?
    false,          // Targetable as a destroyable overlay?
    false,          // Crushable by tracked vehicle?
    false,          // Is this harvestable Tiberium?
    false,          // Stops low level bullets in flight?
    false,          // Theater specific art?
    false,          // Is this a wall type?
    false,          // Is this a crate?
);

const V12: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_V12, // Overlay type number.
    "V12",       // INI name of overlay.
    TXT_CIV12,   // Full name of overlay.
    LAND_ROCK,   // What kind of ground is it?
    0,           // If this is a wall, how many damage levels?
    0,           // If this is a wall, how many damage points can it take per level?
    false,       // Visible on the radar map?
    false,       // Is it a wooden overlay (affected by fire)?
    false,       // Targetable as a destroyable overlay?
    true,        // Crushable by tracked vehicle?
    false,       // Is this harvestable Tiberium?
    false,       // Stops low level bullets in flight?
    true,        // Theater specific art?
    false,       // Is this a wall type?
    false,       // Is this a crate?
);
const V13: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_V13, // Overlay type number.
    "V13",       // INI name of overlay.
    TXT_CIV13,   // Full name of overlay.
    LAND_ROCK,   // What kind of ground is it?
    0,           // If this is a wall, how many damage levels?
    0,           // If this is a wall, how many damage points can it take per level?
    false,       // Visible on the radar map?
    false,       // Is it a wooden overlay (affected by fire)?
    false,       // Targetable as a destroyable overlay?
    true,        // Crushable by tracked vehicle?
    false,       // Is this harvestable Tiberium?
    false,       // Stops low level bullets in flight?
    true,        // Theater specific art?
    false,       // Is this a wall type?
    false,       // Is this a crate?
);
const V14: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_V14, // Overlay type number.
    "V14",       // INI name of overlay.
    TXT_CIV14,   // Full name of overlay.
    LAND_ROCK,   // What kind of ground is it?
    0,           // If this is a wall, how many damage levels?
    0,           // If this is a wall, how many damage points can it take per level?
    false,       // Visible on the radar map?
    false,       // Is it a wooden overlay (affected by fire)?
    false,       // Targetable as a destroyable overlay?
    true,        // Crushable by tracked vehicle?
    false,       // Is this harvestable Tiberium?
    false,       // Stops low level bullets in flight?
    true,        // Theater specific art?
    false,       // Is this a wall type?
    false,       // Is this a crate?
);
const V15: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_V15, // Overlay type number.
    "V15",       // INI name of overlay.
    TXT_CIV15,   // Full name of overlay.
    LAND_ROCK,   // What kind of ground is it?
    0,           // If this is a wall, how many damage levels?
    0,           // If this is a wall, how many damage points can it take per level?
    false,       // Visible on the radar map?
    false,       // Is it a wooden overlay (affected by fire)?
    false,       // Targetable as a destroyable overlay?
    true,        // Crushable by tracked vehicle?
    false,       // Is this harvestable Tiberium?
    false,       // Stops low level bullets in flight?
    true,        // Theater specific art?
    false,       // Is this a wall type?
    false,       // Is this a crate?
);
const V16: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_V16, // Overlay type number.
    "V16",       // INI name of overlay.
    TXT_CIV16,   // Full name of overlay.
    LAND_ROCK,   // What kind of ground is it?
    0,           // If this is a wall, how many damage levels?
    0,           // If this is a wall, how many damage points can it take per level?
    false,       // Visible on the radar map?
    false,       // Is it a wooden overlay (affected by fire)?
    false,       // Targetable as a destroyable overlay?
    true,        // Crushable by tracked vehicle?
    false,       // Is this harvestable Tiberium?
    false,       // Stops low level bullets in flight?
    true,        // Theater specific art?
    false,       // Is this a wall type?
    false,       // Is this a crate?
);
const V17: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_V17, // Overlay type number.
    "V17",       // INI name of overlay.
    TXT_CIV17,   // Full name of overlay.
    LAND_ROCK,   // What kind of ground is it?
    0,           // If this is a wall, how many damage levels?
    0,           // If this is a wall, how many damage points can it take per level?
    false,       // Visible on the radar map?
    false,       // Is it a wooden overlay (affected by fire)?
    false,       // Targetable as a destroyable overlay?
    true,        // Crushable by tracked vehicle?
    false,       // Is this harvestable Tiberium?
    false,       // Stops low level bullets in flight?
    true,        // Theater specific art?
    false,       // Is this a wall type?
    false,       // Is this a crate?
);
const V18: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_V18, // Overlay type number.
    "V18",       // INI name of overlay.
    TXT_CIV18,   // Full name of overlay.
    LAND_ROCK,   // What kind of ground is it?
    0,           // If this is a wall, how many damage levels?
    0,           // If this is a wall, how many damage points can it take per level?
    false,       // Visible on the radar map?
    false,       // Is it a wooden overlay (affected by fire)?
    false,       // Targetable as a destroyable overlay?
    true,        // Crushable by tracked vehicle?
    false,       // Is this harvestable Tiberium?
    false,       // Stops low level bullets in flight?
    true,        // Theater specific art?
    false,       // Is this a wall type?
    false,       // Is this a crate?
);
const FlagSpot: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_FLAG_SPOT, // Overlay type number.
    "FPLS",            // INI name of overlay.
    TXT_FLAG_SPOT,     // Full name of overlay.
    LAND_CLEAR,        // What kind of ground is it?
    0,                 // If this is a wall, how many damage levels?
    0,                 // If this is a wall, how many damage points can it take per level?
    true,              // Visible on the radar map?
    false,             // Is it a wooden overlay (affected by fire)?
    false,             // Targetable as a destroyable overlay?
    false,             // Crushable by tracked vehicle?
    false,             // Is this harvestable Tiberium?
    false,             // Stops low level bullets in flight?
    false,             // Theater specific art?
    false,             // Is this a wall type?
    false,             // Is this a crate?
);
const WoodCrate: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_WOOD_CRATE, // Overlay type number.
    "WCRATE",           // INI name of overlay.
    TXT_WOOD_CRATE,     // Full name of overlay.
    LAND_CLEAR,         // What kind of ground is it?
    0,                  // If this is a wall, how many damage levels?
    0,                  // If this is a wall, how many damage points can it take per level?
    false,              // Visible on the radar map?
    false,              // Is it a wooden overlay (affected by fire)?
    false,              // Targetable as a destroyable overlay?
    false,              // Crushable by tracked vehicle?
    false,              // Is this harvestable Tiberium?
    false,              // Stops low level bullets in flight?
    false,              // Theater specific art?
    false,              // Is this a wall type?
    true,               // Is this a crate?
);
const SteelCrate: OverlayTypeClass = OverlayTypeClass::new(
    OVERLAY_STEEL_CRATE, // Overlay type number.
    "SCRATE",            // INI name of overlay.
    TXT_STEEL_CRATE,     // Full name of overlay.
    LAND_CLEAR,          // What kind of ground is it?
    0,                   // If this is a wall, how many damage levels?
    0,                   // If this is a wall, how many damage points can it take per level?
    false,               // Visible on the radar map?
    false,               // Is it a wooden overlay (affected by fire)?
    false,               // Targetable as a destroyable overlay?
    false,               // Crushable by tracked vehicle?
    false,               // Is this harvestable Tiberium?
    false,               // Stops low level bullets in flight?
    false,               // Theater specific art?
    false,               // Is this a wall type?
    true,                // Is this a crate?
);
