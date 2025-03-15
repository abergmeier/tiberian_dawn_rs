#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables
)]
use crate::animation::AnimType::*;
use crate::armor::ArmorType::*;
use crate::building::STRUCTF;
use crate::house::HOUSEF;
use crate::mission::MissionType::*;
use crate::speed::MPHType::*;
use crate::speed::SpeedType::*;
use crate::text::IDs::*;
use crate::unit::UnitType::*;
use crate::unit::UnitTypeClass;
use crate::weapon::WeaponType::*;
use crate::ADVANCED;

// Visceroid
const UnitVisceroid: UnitTypeClass = UnitTypeClass::new(
    UNIT_VICE,
    TXT_VISCEROID,        // NAME:			Text name of this unit type.
    "VICE",               // NAME:			Text name of this unit type.
    ANIM_NAPALM2,         // EXPLOSION:	Type of explosion when destroyed.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Can this be a goodie surprise from a crate?
    true,                 // Is a leader type?
    false,                // Only has eight facings?
    true,                 // Always use the given name for the vehicle?
    false,                //	Is this a typical transport vehicle?
    false,                // Can it be crushed by a heavy vehicle?
    true,                 // Can this unit squash infantry?
    false,                // Does this unit harvest Tiberium?
    true,                 // Is invisible to radar?
    true,                 // Is selectable by player?
    true,                 // Can it be a target for attack or move?
    true,                 // Is it insignificant (won't be announced)?
    false,                // Is it immune to normal combat damage?
    false,                // Is it equipped with a combat turret?
    false,                // Fires multiple shots in quick succession?
    false,                // Can it be repaired in a repair facility?
    false,                // Can the player construct or order this unit?
    false,                // Is there a crew inside?
    false,                // Does it have a rotating radar dish?
    false,                // Is there an associated firing animation?
    false,                // Must the turret be in a locked down position while moving?
    false,                // Does it lay tracks while moving?
    false,                // Is this a gigundo-rotund-enormous unit?
    false,                // Is the unit's art as "chunky" cardinal facing only?
    false,                // Is the unit capable of cloaking?
    true,                 // Does the unit have a constant animation?
    -1,                   // AMMO:			Number of shots it has (default).
    150,                  // STRENGTH:	Strength (in damage points).
    4,                    // SIGHTRANGE:	Range of sighting.
    800,                  // COST:			Cost to build (Credits).
    1,                    // SCENARIO:	Starting availability scenario.
    80,
    20, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::JP.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_CHEMSPRAY),
    None,
    ARMOR_WOOD,   // ARMOR:		Armor type
    SPEED_TRACK,  // MOVE:			Locomotion type.
    MPH_MEDIUM,   // SPEED:		Miles per hour.
    5,            // ROT:			Rate of turn (degrees per tick).
    0,            // Turret center offset along body centerline.
    MISSION_HUNT, // ORDERS:		Default order to give new unit.
);

// Flame tank
const UnitFTank: UnitTypeClass = UnitTypeClass::new(
    UNIT_FTANK,
    TXT_FTANK,             // NAME:			Text name of this unit type.
    "FTNK",                // NAME:			Text name of this unit type.
    ANIM_NAPALM3,          // EXPLOSION:	Type of explosion when destroyed.
    4,                     // Build level.
    STRUCTF::RADAR.bits(), // Building prerequisite.
    true,                  // Can this be a goodie surprise from a crate?
    true,                  // Is a leader type?
    false,                 // Only has eight facings?
    false,                 // Always use the given name for the vehicle?
    false,                 //	Is this a typical transport vehicle?
    false,                 // Can it be crushed by a heavy vehicle?
    true,                  // Can this unit squash infantry?
    false,                 // Does this unit harvest Tiberium?
    false,                 // Is invisible to radar?
    true,                  // Is selectable by player?
    true,                  // Can it be a target for attack or move?
    false,                 // Is it insignificant (won't be announced)?
    false,                 // Is it immune to normal combat damage?
    false,                 // Is it equipped with a combat turret?
    true,                  // Fires multiple shots in quick succession?
    true,                  // Can it be repaired in a repair facility?
    true,                  // Can the player construct or order this unit?
    true,                  // Is there a crew inside?
    false,                 // Does it have a rotating radar dish?
    false,                 // Is there an associated firing animation?
    false,                 // Must the turret be in a locked down position while moving?
    true,                  // Does it lay tracks while moving?
    false,                 // Is this a gigundo-rotund-enormous unit?
    false,                 // Is the unit's art as "chunky" cardinal facing only?
    false,                 // Is the unit capable of cloaking?
    false,                 // Does the unit have a constant animation?
    -1,                    // AMMO:			Number of shots it has (default).
    300,                   // STRENGTH:	Strength (in damage points).
    4,                     // SIGHTRANGE:	Range of sighting.
    800,                   // COST:			Cost to build (Credits).
    9,                     // SCENARIO:	Starting availability scenario.
    80,
    66, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_FLAME_TONGUE),
    None,
    ARMOR_STEEL,  // ARMOR:		Armor type
    SPEED_TRACK,  // MOVE:			Locomotion type.
    MPH_MEDIUM,   // SPEED:		Miles per hour.
    5,            // ROT:			Rate of turn (degrees per tick).
    0,            // Turret center offset along body centerline.
    MISSION_HUNT, // ORDERS:		Default order to give new unit.
);

// Stealth tank
const UnitSTank: UnitTypeClass = UnitTypeClass::new(
    UNIT_STANK,
    TXT_STANK,             // NAME:			Text name of this unit type.
    "STNK",                // NAME:			Text name of this unit type.
    ANIM_FRAG2,            // EXPLOSION:	Type of explosion when destroyed.
    5,                     // Build level.
    STRUCTF::RADAR.bits(), // Building prerequisite.
    true,                  // Can this be a goodie surprise from a crate?
    true,                  // Is a leader type?
    false,                 // Only has eight facings?
    false,                 // Always use the given name for the vehicle?
    false,                 //	Is this a typical transport vehicle?
    false,                 // Can it be crushed by a heavy vehicle?
    true,                  // Can this unit squash infantry?
    false,                 // Does this unit harvest Tiberium?
    true,                  // Is invisible to radar?
    true,                  // Is selectable by player?
    true,                  // Can it be a target for attack or move?
    false,                 // Is it insignificant (won't be announced)?
    false,                 // Is it immune to normal combat damage?
    false,                 // Is it equipped with a combat turret?
    true,                  // Fires multiple shots in quick succession?
    true,                  // Can it be repaired in a repair facility?
    true,                  // Can the player construct or order this unit?
    true,                  // Is there a crew inside?
    false,                 // Does it have a rotating radar dish?
    false,                 // Is there an associated firing animation?
    false,                 // Must the turret be in a locked down position while moving?
    true,                  // Does it lay tracks while moving?
    false,                 // Is this a gigundo-rotund-enormous unit?
    false,                 // Is the unit's art as "chunky" cardinal facing only?
    true,                  // Is the unit capable of cloaking?
    false,                 // Does the unit have a constant animation?
    -1,                    // AMMO:			Number of shots it has (default).
    110,                   // STRENGTH:	Strength (in damage points).
    4,                     // SIGHTRANGE:	Range of sighting.
    900,                   // COST:			Cost to build (Credits).
    12,                    // SCENARIO:	Starting availability scenario.
    80,
    81, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_DRAGON),
    None,
    ARMOR_ALUMINUM,  // ARMOR:		Armor type
    SPEED_TRACK,     // MOVE:			Locomotion type.
    MPH_MEDIUM_FAST, // SPEED:		Miles per hour.
    5,               // ROT:			Rate of turn (degrees per tick).
    0,               // Turret center offset along body centerline.
    MISSION_HUNT,    // ORDERS:		Default order to give new unit.
);

// Light tank
const UnitLTank: UnitTypeClass = UnitTypeClass::new(
    UNIT_LTANK,
    TXT_LTANK,            // NAME:			Text name of this unit type.
    "LTNK",               // NAME:			Text name of this unit type.
    ANIM_FRAG1,           // EXPLOSION:	Type of explosion when destroyed.
    3,                    // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    true,                 // Can this be a goodie surprise from a crate?
    true,                 // Is a leader type?
    false,                // Only has eight facings?
    false,                // Always use the given name for the vehicle?
    false,                //	Is this a typical transport vehicle?
    false,                // Can it be crushed by a heavy vehicle?
    true,                 // Can this unit squash infantry?
    false,                // Does this unit harvest Tiberium?
    false,                // Is invisible to radar?
    true,                 // Is selectable by player?
    true,                 // Can it be a target for attack or move?
    false,                // Is it insignificant (won't be announced)?
    false,                // Is it immune to normal combat damage?
    true,                 // Is it equipped with a combat turret?
    false,                // Fires multiple shots in quick succession?
    true,                 // Can it be repaired in a repair facility?
    true,                 // Can the player construct or order this unit?
    true,                 // Is there a crew inside?
    false,                // Does it have a rotating radar dish?
    false,                // Is there an associated firing animation?
    false,                // Must the turret be in a locked down position while moving?
    true,                 // Does it lay tracks while moving?
    false,                // Is this a gigundo-rotund-enormous unit?
    false,                // Is the unit's art as "chunky" cardinal facing only?
    false,                // Is the unit capable of cloaking?
    false,                // Does the unit have a constant animation?
    -1,                   // AMMO:			Number of shots it has (default).
    300,                  // STRENGTH:	Strength (in damage points).
    3,                    // SIGHTRANGE:	Range of sighting.
    600,                  // COST:			Cost to build (Credits).
    5,                    // SCENARIO:	Starting availability scenario.
    80,
    56, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_75MM),
    None,
    ARMOR_STEEL,  // ARMOR:		Armor type
    SPEED_TRACK,  // MOVE:			Locomotion type.
    MPH_MEDIUM,   // SPEED:		Miles per hour.
    5,            // ROT:			Rate of turn (degrees per tick).
    0,            // Turret center offset along body centerline.
    MISSION_HUNT, // ORDERS:		Default order to give new unit.
);

// Medium tank
const UnitMTank: UnitTypeClass = UnitTypeClass::new(
    UNIT_MTANK,
    TXT_MTANK,            // NAME:			Text name of this unit type.
    "MTNK",               // NAME:			Text name of this unit type.
    ANIM_FRAG2,           // EXPLOSION:	Type of explosion when destroyed.
    3,                    // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    true,                 // Can this be a goodie surprise from a crate?
    true,                 // Is a leader type?
    false,                // Only has eight facings?
    false,                // Always use the given name for the vehicle?
    false,                //	Is this a typical transport vehicle?
    false,                // Can it be crushed by a heavy vehicle?
    true,                 // Can this unit squash infantry?
    false,                // Does this unit harvest Tiberium?
    false,                // Is invisible to radar?
    true,                 // Is selectable by player?
    true,                 // Can it be a target for attack or move?
    false,                // Is it insignificant (won't be announced)?
    false,                // Is it immune to normal combat damage?
    true,                 // Is it equipped with a combat turret?
    false,                // Fires multiple shots in quick succession?
    true,                 // Can it be repaired in a repair facility?
    true,                 // Can the player construct or order this unit?
    true,                 // Is there a crew inside?
    false,                // Does it have a rotating radar dish?
    false,                // Is there an associated firing animation?
    false,                // Must the turret be in a locked down position while moving?
    true,                 // Does it lay tracks while moving?
    true,                 // Is this a gigundo-rotund-enormous unit?
    false,                // Is the unit's art as "chunky" cardinal facing only?
    false,                // Is the unit capable of cloaking?
    false,                // Does the unit have a constant animation?
    -1,                   // AMMO:			Number of shots it has (default).
    400,                  // STRENGTH:	Strength (in damage points).
    3,                    // SIGHTRANGE:	Range of sighting.
    800,                  // COST:			Cost to build (Credits).
    7,                    // SCENARIO:	Starting availability scenario.
    80,
    62, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_105MM),
    None,
    ARMOR_STEEL,  // ARMOR:		Armor type
    SPEED_TRACK,  // MOVE:			Locomotion type.
    MPH_MEDIUM,   // SPEED:		Miles per hour.
    5,            // ROT:			Rate of turn (degrees per tick).
    0,            // Turret center offset along body centerline.
    MISSION_HUNT, // ORDERS:		Default order to give new unit.
);

// Mastadon tank
const UnitHTank: UnitTypeClass = UnitTypeClass::new(
    UNIT_HTANK,
    TXT_HTANK,              // NAME:			Text name of this unit type.
    "HTNK",                 // NAME:			Text name of this unit type.
    ANIM_ART_EXP1,          // EXPLOSION:	Type of explosion when destroyed.
    5,                      // Build level.
    STRUCTF::REPAIR.bits(), // Building prerequisite.
    true,                   // Can this be a goodie surprise from a crate?
    true,                   // Is a leader type?
    false,                  // Only has eight facings?
    false,                  // Always use the given name for the vehicle?
    false,                  //	Is this a typical transport vehicle?
    false,                  // Can it be crushed by a heavy vehicle?
    true,                   // Can this unit squash infantry?
    false,                  // Does this unit harvest Tiberium?
    false,                  // Is invisible to radar?
    true,                   // Is selectable by player?
    true,                   // Can it be a target for attack or move?
    false,                  // Is it insignificant (won't be announced)?
    false,                  // Is it immune to normal combat damage?
    true,                   // Is it equipped with a combat turret?
    true,                   // Fires multiple shots in quick succession?
    true,                   // Can it be repaired in a repair facility?
    true,                   // Can the player construct or order this unit?
    true,                   // Is there a crew inside?
    false,                  // Does it have a rotating radar dish?
    false,                  // Is there an associated firing animation?
    false,                  // Must the turret be in a locked down position while moving?
    true,                   // Does it lay tracks while moving?
    true,                   // Is this a gigundo-rotund-enormous unit?
    false,                  // Is the unit's art as "chunky" cardinal facing only?
    false,                  // Is the unit capable of cloaking?
    false,                  // Does the unit have a constant animation?
    -1,                     // AMMO:			Number of shots it has (default).
    600,                    // STRENGTH:	Strength (in damage points).
    4,                      // SIGHTRANGE:	Range of sighting.
    1500,                   // COST:			Cost to build (Credits).
    13,                     // SCENARIO:	Starting availability scenario.
    80,
    80, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_120MM),
    Some(WEAPON_MAMMOTH_TUSK),
    ARMOR_STEEL,     // ARMOR:		Armor type
    SPEED_TRACK,     // MOVE:			Locomotion type.
    MPH_MEDIUM_SLOW, // SPEED:		Miles per hour.
    5,               // ROT:			Rate of turn (degrees per tick).
    0,               // Turret center offset along body centerline.
    MISSION_HUNT,    // ORDERS:		Default order to give new unit.
);

// Mobile HQ
const UnitMHQ: UnitTypeClass = UnitTypeClass::new(
    UNIT_MHQ,
    TXT_MHQ,              // NAME:			Text name of this unit type.
    "MHQ",                // NAME:			Text name of this unit type.
    ANIM_FRAG2,           // EXPLOSION:	Type of explosion when destroyed.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Can this be a goodie surprise from a crate?
    false,                // Is a leader type?
    false,                // Only has eight facings?
    false,                // Always use the given name for the vehicle?
    false,                //	Is this a typical transport vehicle?
    false,                // Can it be crushed by a heavy vehicle?
    true,                 // Can this unit squash infantry?
    false,                // Does this unit harvest Tiberium?
    false,                // Is invisible to radar?
    true,                 // Is selectable by player?
    true,                 // Can it be a target for attack or move?
    false,                // Is it insignificant (won't be announced)?
    false,                // Is it immune to normal combat damage?
    false,                // Is it equipped with a combat turret?
    false,                // Fires multiple shots in quick succession?
    true,                 // Can it be repaired in a repair facility?
    true,                 // Can the player construct or order this unit?
    true,                 // Is there a crew inside?
    true,                 // Does it have a rotating radar dish?
    false,                // Is there an associated firing animation?
    false,                // Must the turret be in a locked down position while moving?
    true,                 // Does it lay tracks while moving?
    false,                // Is this a gigundo-rotund-enormous unit?
    false,                // Is the unit's art as "chunky" cardinal facing only?
    false,                // Is the unit capable of cloaking?
    false,                // Does the unit have a constant animation?
    -1,                   // AMMO:			Number of shots it has (default).
    110,                  // STRENGTH:	Strength (in damage points).
    5,                    // SIGHTRANGE:	Range of sighting.
    600,                  // COST:			Cost to build (Credits).
    99,                   // SCENARIO:	Starting availability scenario.
    80,
    100, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::BAD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    ARMOR_ALUMINUM, // ARMOR:		Armor type
    SPEED_TRACK,    // MOVE:			Locomotion type.
    MPH_MEDIUM,     // SPEED:		Miles per hour.
    5,              // ROT:			Rate of turn (degrees per tick).
    0,              // Turret center offset along body centerline.
    MISSION_HUNT,   // ORDERS:		Default order to give new unit.
);

// Landing craft
const UnitHover: UnitTypeClass = UnitTypeClass::new(
    UNIT_HOVER,
    TXT_HOVER,            // NAME:			Text name of this unit type.
    "LST",                // NAME:			Text name of this unit type.
    ANIM_FBALL1,          // EXPLOSION:	Type of explosion when destroyed.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Can this be a goodie surprise from a crate?
    false,                // Is a leader type?
    false,                // Only has eight facings?
    true,                 // Always use the given name for the vehicle?
    true,                 //	Is this a typical transport vehicle?
    false,                // Can it be crushed by a heavy vehicle?
    false,                // Can this unit squash infantry?
    false,                // Does this unit harvest Tiberium?
    false,                // Is invisible to radar?
    //		true,			// Is selectable by player?
    false, // Is selectable by player?
    false, // Can it be a target for attack or move?
    true,  // Is it insignificant (won't be announced)?
    true,  // Is it immune to normal combat damage?
    false, // Is it equipped with a combat turret?
    false, // Fires multiple shots in quick succession?
    false, // Can it be repaired in a repair facility?
    false, // Can the player construct or order this unit?
    false, // Is there a crew inside?
    false, // Does it have a rotating radar dish?
    false, // Is there an associated firing animation?
    false, // Must the turret be in a locked down position while moving?
    false, // Does it lay tracks while moving?
    true,  // Is this a gigundo-rotund-enormous unit?
    true,  // Is the unit's art as "chunky" cardinal facing only?
    false, // Is the unit capable of cloaking?
    false, // Does the unit have a constant animation?
    -1,    // AMMO:			Number of shots it has (default).
    400,   // STRENGTH:	Strength (in damage points).
    3,     // SIGHTRANGE:	Range of sighting.
    300,   // COST:			Cost to build (Credits).
    99,    // SCENARIO:	Starting availability scenario.
    80,
    40, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::BAD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    ARMOR_ALUMINUM,  // ARMOR:		Armor type
    SPEED_HOVER,     // MOVE:			Locomotion type.
    MPH_MEDIUM_FAST, // SPEED:		Miles per hour.
    127,             // ROT:			Rate of turn (degrees per tick).
    0,               // Turret center offset along body centerline.
    MISSION_HUNT,    // ORDERS:		Default order to give new unit.
);

// Mobile sam launcher
const UnitSAM: UnitTypeClass = UnitTypeClass::new(
    UNIT_MSAM,
    TXT_MSAM,               // NAME:			Text name of this unit type.
    "MLRS",                 // NAME:			Text name of this unit type.
    ANIM_FRAG2,             // EXPLOSION:	Type of explosion when destroyed.
    7,                      // Build level.
    STRUCTF::ATOWER.bits(), // Building prerequisite.
    true,                   // Can this be a goodie surprise from a crate?
    true,                   // Is a leader type?
    false,                  // Only has eight facings?
    false,                  // Always use the given name for the vehicle?
    false,                  //	Is this a typical transport vehicle?
    false,                  // Can it be crushed by a heavy vehicle?
    false,                  // Can this unit squash infantry?
    false,                  // Does this unit harvest Tiberium?
    false,                  // Is invisible to radar?
    true,                   // Is selectable by player?
    true,                   // Can it be a target for attack or move?
    false,                  // Is it insignificant (won't be announced)?
    false,                  // Is it immune to normal combat damage?
    true,                   // Is it equipped with a combat turret?
    false,                  // Fires multiple shots in quick succession?
    true,                   // Can it be repaired in a repair facility?
    true,                   // Can the player construct or order this unit?
    true,                   // Is there a crew inside?
    false,                  // Does it have a rotating radar dish?
    false,                  // Is there an associated firing animation?
    true,                   // Must the turret be in a locked down position while moving?
    true,                   // Does it lay tracks while moving?
    true,                   // Is this a gigundo-rotund-enormous unit?
    false,                  // Is the unit's art as "chunky" cardinal facing only?
    false,                  // Is the unit capable of cloaking?
    false,                  // Does the unit have a constant animation?
    2,                      // AMMO:			Number of shots it has (default).
    120,                    // STRENGTH:	Strength (in damage points).
    4,                      // SIGHTRANGE:	Range of sighting.
    750,                    // COST:			Cost to build (Credits).
    98,                     // SCENARIO:	Starting availability scenario.
    80,
    30, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()|
     HOUSEF::MULTI2.bits()|
     HOUSEF::MULTI3.bits()|
     HOUSEF::MULTI4.bits()|
     HOUSEF::MULTI5.bits()|
     HOUSEF::MULTI6.bits()|
     HOUSEF::JP.bits()|
 //	HOUSEF::GOOD.bits()|
     HOUSEF::BAD.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_HONEST_JOHN),
    None,
    ARMOR_ALUMINUM, // ARMOR:		Armor type
    SPEED_TRACK,    // MOVE:			Locomotion type.
    MPH_MEDIUM,     // SPEED:		Miles per hour.
    5,              // ROT:			Rate of turn (degrees per tick).
    0,              // Turret center offset along body centerline.
    MISSION_HUNT,   // ORDERS:		Default order to give new unit.
);

// Artillery
const UnitArty: UnitTypeClass = UnitTypeClass::new(
    UNIT_ARTY,
    TXT_ARTY,             // NAME:			Text name of this unit type.
    "ARTY",               // NAME:			Text name of this unit type.
    ANIM_ART_EXP1,        // EXPLOSION:	Type of explosion when destroyed.
    6,                    // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    true,                 // Can this be a goodie surprise from a crate?
    true,                 // Is a leader type?
    false,                // Only has eight facings?
    false,                // Always use the given name for the vehicle?
    false,                //	Is this a typical transport vehicle?
    false,                // Can it be crushed by a heavy vehicle?
    false,                // Can this unit squash infantry?
    false,                // Does this unit harvest Tiberium?
    false,                // Is invisible to radar?
    true,                 // Is selectable by player?
    true,                 // Can it be a target for attack or move?
    false,                // Is it insignificant (won't be announced)?
    false,                // Is it immune to normal combat damage?
    false,                // Is it equipped with a combat turret?
    false,                // Fires multiple shots in quick succession?
    true,                 // Can it be repaired in a repair facility?
    true,                 // Can the player construct or order this unit?
    true,                 // Is there a crew inside?
    false,                // Does it have a rotating radar dish?
    false,                // Is there an associated firing animation?
    false,                // Must the turret be in a locked down position while moving?
    true,                 // Does it lay tracks while moving?
    false,                // Is this a gigundo-rotund-enormous unit?
    false,                // Is the unit's art as "chunky" cardinal facing only?
    false,                // Is the unit capable of cloaking?
    false,                // Does the unit have a constant animation?
    -1,                   // AMMO:			Number of shots it has (default).
    75,                   // STRENGTH:	Strength (in damage points).
    4,                    // SIGHTRANGE:	Range of sighting.
    450,                  // COST:			Cost to build (Credits).
    9,                    // SCENARIO:	Starting availability scenario.
    80,
    73, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_155MM),
    None,
    ARMOR_ALUMINUM,  // ARMOR:		Armor type
    SPEED_TRACK,     // MOVE:			Locomotion type.
    MPH_MEDIUM_SLOW, // SPEED:		Miles per hour.
    2,               // ROT:			Rate of turn (degrees per tick).
    0,               // Turret center offset along body centerline.
    MISSION_HUNT,    // ORDERS:		Default order to give new unit.
);

// Harvester
const UnitHarvester: UnitTypeClass = UnitTypeClass::new(
    UNIT_HARVESTER,
    TXT_HARVESTER,            // NAME:			Text name of this unit type.
    "HARV",                   // NAME:			Text name of this unit type.
    ANIM_FBALL1,              // EXPLOSION:	Type of explosion when destroyed.
    2,                        // Build level.
    STRUCTF::REFINERY.bits(), // Building prerequisite.
    true,                     // Can this be a goodie surprise from a crate?
    false,                    // Is a leader type?
    false,                    // Only has eight facings?
    true,                     // Always use the given name for the vehicle?
    false,                    //	Is this a typical transport vehicle?
    false,                    // Can it be crushed by a heavy vehicle?
    true,                     // Can this unit squash infantry?
    true,                     // Does this unit harvest Tiberium?
    false,                    // Is invisible to radar?
    true,                     // Is selectable by player?
    true,                     // Can it be a target for attack or move?
    false,                    // Is it insignificant (won't be announced)?
    false,                    // Is it immune to normal combat damage?
    false,                    // Is it equipped with a combat turret?
    false,                    // Fires multiple shots in quick succession?
    true,                     // Can it be repaired in a repair facility?
    true,                     // Can the player construct or order this unit?
    true,                     // Is there a crew inside?
    false,                    // Does it have a rotating radar dish?
    false,                    // Is there an associated firing animation?
    false,                    // Must the turret be in a locked down position while moving?
    true,                     // Does it lay tracks while moving?
    true,                     // Is this a gigundo-rotund-enormous unit?
    false,                    // Is the unit's art as "chunky" cardinal facing only?
    false,                    // Is the unit capable of cloaking?
    false,                    // Does the unit have a constant animation?
    -1,                       // AMMO:			Number of shots it has (default).
    600,                      // STRENGTH:	Strength (in damage points).
    //	300,										// STRENGTH:	Strength (in damage points).
    2,    // SIGHTRANGE:	Range of sighting.
    1400, // COST:			Cost to build (Credits).
    7,    // SCENARIO:	Starting availability scenario.
    80,
    85, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::BAD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    ARMOR_ALUMINUM,  // ARMOR:		Armor type
    SPEED_WHEEL,     // MOVE:			Locomotion type.
    MPH_MEDIUM_SLOW, // SPEED:		Miles per hour.
    5,               // ROT:			Rate of turn (degrees per tick).
    0,               // Turret center offset along body centerline.
    MISSION_HARVEST, // ORDERS:		Default order to give new unit.
);

// Mobile construction vehicle
const UnitMCV: UnitTypeClass = UnitTypeClass::new(
    UNIT_MCV,
    TXT_MCV,             // NAME:			Text name of this unit type.
    "MCV",               // NAME:			Text name of this unit type.
    ANIM_FBALL1,         // EXPLOSION:	Type of explosion when destroyed.
    7,                   // Build level.
    STRUCTF::EYE.bits(), // Building prerequisite.
    true,                // Can this be a goodie surprise from a crate?
    false,               // Is a leader type?
    false,               // Only has eight facings?
    false,               // Always use the given name for the vehicle?
    false,               //	Is this a typical transport vehicle?
    false,               // Can it be crushed by a heavy vehicle?
    true,                // Can this unit squash infantry?
    false,               // Does this unit harvest Tiberium?
    false,               // Is invisible to radar?
    true,                // Is selectable by player?
    true,                // Can it be a target for attack or move?
    false,               // Is it insignificant (won't be announced)?
    false,               // Is it immune to normal combat damage?
    false,               // Is it equipped with a combat turret?
    false,               // Fires multiple shots in quick succession?
    false,               // Can it be repaired in a repair facility?
    true,                // Can the player construct or order this unit?
    true,                // Is there a crew inside?
    false,               // Does it have a rotating radar dish?
    false,               // Is there an associated firing animation?
    false,               // Must the turret be in a locked down position while moving?
    true,                // Does it lay tracks while moving?
    true,                // Is this a gigundo-rotund-enormous unit?
    false,               // Is the unit's art as "chunky" cardinal facing only?
    false,               // Is the unit capable of cloaking?
    false,               // Does the unit have a constant animation?
    -1,                  // AMMO:			Number of shots it has (default).
    600,                 // STRENGTH:	Strength (in damage points).
    2,                   // SIGHTRANGE:	Range of sighting.
    5000,                // COST:			Cost to build (Credits).
    15,                  // SCENARIO:	Starting availability scenario.
    80,
    86, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::BAD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    ARMOR_ALUMINUM,  // ARMOR:		Armor type
    SPEED_WHEEL,     // MOVE:			Locomotion type.
    MPH_MEDIUM_SLOW, // SPEED:		Miles per hour.
    5,               // ROT:			Rate of turn (degrees per tick).
    0,               // Turret center offset along body centerline.
    MISSION_HUNT,    // ORDERS:		Default order to give new unit.
);

// Jeep (hummer)
const UnitJeep: UnitTypeClass = UnitTypeClass::new(
    UNIT_JEEP,
    TXT_JEEP,             // NAME:			Text name of this unit type.
    "JEEP",               // NAME:			Text name of this unit type.
    ANIM_FRAG1,           // EXPLOSION:	Type of explosion when destroyed.
    2,                    // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    true,                 // Can this be a goodie surprise from a crate?
    true,                 // Is a leader type?
    false,                // Only has eight facings?
    false,                // Always use the given name for the vehicle?
    false,                //	Is this a typical transport vehicle?
    false,                // Can it be crushed by a heavy vehicle?
    false,                // Can this unit squash infantry?
    false,                // Does this unit harvest Tiberium?
    false,                // Is invisible to radar?
    true,                 // Is selectable by player?
    true,                 // Can it be a target for attack or move?
    false,                // Is it insignificant (won't be announced)?
    false,                // Is it immune to normal combat damage?
    true,                 // Is it equipped with a combat turret?
    false,                // Fires multiple shots in quick succession?
    true,                 // Can it be repaired in a repair facility?
    true,                 // Can the player construct or order this unit?
    true,                 // Is there a crew inside?
    false,                // Does it have a rotating radar dish?
    false,                // Is there an associated firing animation?
    false,                // Must the turret be in a locked down position while moving?
    true,                 // Does it lay tracks while moving?
    false,                // Is this a gigundo-rotund-enormous unit?
    false,                // Is the unit's art as "chunky" cardinal facing only?
    false,                // Is the unit capable of cloaking?
    false,                // Does the unit have a constant animation?
    -1,                   // AMMO:			Number of shots it has (default).
    150,                  // STRENGTH:	Strength (in damage points).
    2,                    // SIGHTRANGE:	Range of sighting.
    400,                  // COST:			Cost to build (Credits).
    5,                    // SCENARIO:	Starting availability scenario.
    80,
    41, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_M60MG),
    None,
    ARMOR_ALUMINUM,  // ARMOR:		Armor type
    SPEED_WHEEL,     // MOVE:			Locomotion type.
    MPH_MEDIUM_FAST, // SPEED:		Miles per hour.
    10,              // ROT:			Rate of turn (degrees per tick).
    0,               // Turret center offset along body centerline.
    MISSION_HUNT,    // ORDERS:		Default order to give new unit.
);

// Buggy
const UnitBuggy: UnitTypeClass = UnitTypeClass::new(
    UNIT_BUGGY,
    TXT_DUNE_BUGGY,       // NAME:			Text name of this unit type.
    "BGGY",               // NAME:			Text name of this unit type.
    ANIM_FRAG1,           // EXPLOSION:	Type of explosion when destroyed.
    4,                    // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    true,                 // Can this be a goodie surprise from a crate?
    true,                 // Is a leader type?
    false,                // Only has eight facings?
    false,                // Always use the given name for the vehicle?
    false,                //	Is this a typical transport vehicle?
    false,                // Can it be crushed by a heavy vehicle?
    false,                // Can this unit squash infantry?
    false,                // Does this unit harvest Tiberium?
    false,                // Is invisible to radar?
    true,                 // Is selectable by player?
    true,                 // Can it be a target for attack or move?
    false,                // Is it insignificant (won't be announced)?
    false,                // Is it immune to normal combat damage?
    true,                 // Is it equipped with a combat turret?
    false,                // Fires multiple shots in quick succession?
    true,                 // Can it be repaired in a repair facility?
    true,                 // Can the player construct or order this unit?
    true,                 // Is there a crew inside?
    false,                // Does it have a rotating radar dish?
    false,                // Is there an associated firing animation?
    false,                // Must the turret be in a locked down position while moving?
    false,                // Does it lay tracks while moving?
    false,                // Is this a gigundo-rotund-enormous unit?
    false,                // Is the unit's art as "chunky" cardinal facing only?
    false,                // Is the unit capable of cloaking?
    false,                // Does the unit have a constant animation?
    -1,                   // AMMO:			Number of shots it has (default).
    140,                  // STRENGTH:	Strength (in damage points).
    2,                    // SIGHTRANGE:	Range of sighting.
    300,                  // COST:			Cost to build (Credits).
    5,                    // SCENARIO:	Starting availability scenario.
    80,
    42, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_M60MG),
    None,
    ARMOR_ALUMINUM,  // ARMOR:		Armor type
    SPEED_WHEEL,     // MOVE:			Locomotion type.
    MPH_MEDIUM_FAST, // SPEED:		Miles per hour.
    10,              // ROT:			Rate of turn (degrees per tick).
    0,               // Turret center offset along body centerline.
    MISSION_HUNT,    // ORDERS:		Default order to give new unit.
);

// Attack cycle
const UnitBike: UnitTypeClass = UnitTypeClass::new(
    UNIT_BIKE,
    TXT_BIKE,                        // NAME:			Text name of this unit type.
    "BIKE",                          // NAME:			Text name of this unit type.
    ANIM_FRAG1,                      // EXPLOSION:	Type of explosion when destroyed.
    2,                               // Build level.
    STRUCTF::NONE.bits(),            // Building prerequisite.
    true,                            // Can this be a goodie surprise from a crate?
    true,                            // Is a leader type?
    false,                           // Only has eight facings?
    false,                           // Always use the given name for the vehicle?
    false,                           //	Is this a typical transport vehicle?
    true,                            // Can it be crushed by a heavy vehicle?
    false,                           // Can this unit squash infantry?
    false,                           // Does this unit harvest Tiberium?
    false,                           // Is invisible to radar?
    true,                            // Is selectable by player?
    true,                            // Can it be a target for attack or move?
    false,                           // Is it insignificant (won't be announced)?
    false,                           // Is it immune to normal combat damage?
    false,                           // Is it equipped with a combat turret?
    false,                           // Fires multiple shots in quick succession?
    true,                            // Can it be repaired in a repair facility?
    true,                            // Can the player construct or order this unit?
    true,                            // Is there a crew inside?
    false,                           // Does it have a rotating radar dish?
    false,                           // Is there an associated firing animation?
    false,                           // Must the turret be in a locked down position while moving?
    false,                           // Does it lay tracks while moving?
    false,                           // Is this a gigundo-rotund-enormous unit?
    false,                           // Is the unit's art as "chunky" cardinal facing only?
    false,                           // Is the unit capable of cloaking?
    false,                           // Does the unit have a constant animation?
    -1,                              // AMMO:			Number of shots it has (default).
    if ADVANCED { 90 } else { 160 }, // STRENGTH:	Strength (in damage points).
    2,                               // SIGHTRANGE:	Range of sighting.
    500,                             // COST:			Cost to build (Credits).
    5,                               // SCENARIO:	Starting availability scenario.
    80,
    45, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_DRAGON),
    None,
    ARMOR_WOOD,   // ARMOR:		Armor type
    SPEED_WHEEL,  // MOVE:			Locomotion type.
    MPH_FAST,     // SPEED:		Miles per hour.
    10,           // ROT:			Rate of turn (degrees per tick).
    0,            // Turret center offset along body centerline.
    MISSION_HUNT, // ORDERS:		Default order to give new unit.
);

// Rocket launcher
const UnitMLRS: UnitTypeClass = UnitTypeClass::new(
    UNIT_MLRS,
    TXT_MLRS,            // NAME:			Text name of this unit type.
    "MSAM",              // NAME:			Text name of this unit type.
    ANIM_ART_EXP1,       // EXPLOSION:	Type of explosion when destroyed.
    7,                   // Build level.
    STRUCTF::EYE.bits(), // Building prerequisite.
    true,                // Can this be a goodie surprise from a crate?
    true,                // Is a leader type?
    false,               // Only has eight facings?
    false,               // Always use the given name for the vehicle?
    false,               //	Is this a typical transport vehicle?
    false,               // Can it be crushed by a heavy vehicle?
    false,               // Can this unit squash infantry?
    false,               // Does this unit harvest Tiberium?
    false,               // Is invisible to radar?
    true,                // Is selectable by player?
    true,                // Can it be a target for attack or move?
    false,               // Is it insignificant (won't be announced)?
    false,               // Is it immune to normal combat damage?
    true,                // Is it equipped with a combat turret?
    true,                // Fires multiple shots in quick succession?
    true,                // Can it be repaired in a repair facility?
    true,                // Can the player construct or order this unit?
    true,                // Is there a crew inside?
    false,               // Does it have a rotating radar dish?
    false,               // Is there an associated firing animation?
    true,                // Must the turret be in a locked down position while moving?
    true,                // Does it lay tracks while moving?
    false,               // Is this a gigundo-rotund-enormous unit?
    false,               // Is the unit's art as "chunky" cardinal facing only?
    false,               // Is the unit capable of cloaking?
    false,               // Does the unit have a constant animation?
    -1,                  // AMMO:			Number of shots it has (default).
    100,                 // STRENGTH:	Strength (in damage points).
    4,                   // SIGHTRANGE:	Range of sighting.
    800,                 // COST:			Cost to build (Credits).
    11,                  // SCENARIO:	Starting availability scenario.
    80,
    72, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_MLRS),
    None,
    ARMOR_ALUMINUM, // ARMOR:		Armor type
    SPEED_TRACK,    // MOVE:			Locomotion type.
    MPH_MEDIUM,     // SPEED:		Miles per hour.
    5,              // ROT:			Rate of turn (degrees per tick).
    0,              // Turret center offset along body centerline.
    MISSION_GUARD,  // ORDERS:		Default order to give new unit.
);

// Armored personnel carrier
const UnitAPC: UnitTypeClass = UnitTypeClass::new(
    UNIT_APC,
    TXT_APC,                  // NAME:			Text name of this unit type.
    "APC",                    // NAME:			Text name of this unit type.
    ANIM_FRAG2,               // EXPLOSION:	Type of explosion when destroyed.
    4,                        // Build level.
    STRUCTF::BARRACKS.bits(), // Building prerequisite.
    true,                     // Can this be a goodie surprise from a crate?
    true,                     // Is a leader type?
    false,                    // Only has eight facings?
    false,                    // Always use the given name for the vehicle?
    true,                     //	Is this a typical transport vehicle?
    false,                    // Can it be crushed by a heavy vehicle?
    true,                     // Can this unit squash infantry?
    false,                    // Does this unit harvest Tiberium?
    false,                    // Is invisible to radar?
    true,                     // Is selectable by player?
    true,                     // Can it be a target for attack or move?
    false,                    // Is it insignificant (won't be announced)?
    false,                    // Is it immune to normal combat damage?
    false,                    // Is it equipped with a combat turret?
    false,                    // Fires multiple shots in quick succession?
    true,                     // Can it be repaired in a repair facility?
    true,                     // Can the player construct or order this unit?
    false,                    // Is there a crew inside?
    false,                    // Does it have a rotating radar dish?
    false,                    // Is there an associated firing animation?
    false,                    // Must the turret be in a locked down position while moving?
    true,                     // Does it lay tracks while moving?
    false,                    // Is this a gigundo-rotund-enormous unit?
    false,                    // Is the unit's art as "chunky" cardinal facing only?
    false,                    // Is the unit capable of cloaking?
    false,                    // Does the unit have a constant animation?
    -1,                       // AMMO:			Number of shots it has (default).
    200,                      // STRENGTH:	Strength (in damage points).
    4,                        // SIGHTRANGE:	Range of sighting.
    700,                      // COST:			Cost to build (Credits).
    5,                        // SCENARIO:	Starting availability scenario.
    80,
    15, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_M60MG),
    None,
    ARMOR_STEEL,       // ARMOR:		Armor type
    SPEED_TRACK,       // MOVE:			Locomotion type.
    MPH_MEDIUM_FASTER, // SPEED:		Miles per hour.
    5,                 // ROT:			Rate of turn (degrees per tick).
    0,                 // Turret center offset along body centerline.
    MISSION_HUNT,      // ORDERS:		Default order to give new unit.
);

// Gunboat
const UnitGunBoat: UnitTypeClass = UnitTypeClass::new(
    UNIT_GUNBOAT,
    TXT_GUNBOAT,          // NAME:			Text name of this unit type.
    "BOAT",               // NAME:			Text name of this unit type.
    ANIM_FBALL1,          // EXPLOSION:	Type of explosion when destroyed.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Can this be a goodie surprise from a crate?
    true,                 // Is a leader type?
    false,                // Only has eight facings?
    true,                 // Always use the given name for the vehicle?
    false,                //	Is this a typical transport vehicle?
    false,                // Can it be crushed by a heavy vehicle?
    false,                // Can this unit squash infantry?
    false,                // Does this unit harvest Tiberium?
    false,                // Is invisible to radar?
    true,                 // Is selectable by player?
    true,                 // Can it be a target for attack or move?
    true,                 // Is it insignificant (won't be announced)?
    false,                // Is it immune to normal combat damage?
    true,                 // Is it equipped with a combat turret?
    true,                 // Fires multiple shots in quick succession?
    false,                // Can it be repaired in a repair facility?
    false,                // Can the player construct or order this unit?
    true,                 // Is there a crew inside?
    false,                // Does it have a rotating radar dish?
    false,                // Is there an associated firing animation?
    false,                // Must the turret be in a locked down position while moving?
    false,                // Does it lay tracks while moving?
    true,                 // Is this a gigundo-rotund-enormous unit?
    true,                 // Is the unit's art as "chunky" cardinal facing only?
    false,                // Is the unit capable of cloaking?
    false,                // Does the unit have a constant animation?
    -1,                   // AMMO:			Number of shots it has (default).
    700,                  // STRENGTH:	Strength (in damage points).
    5,                    // SIGHTRANGE:	Range of sighting.
    300,                  // COST:			Cost to build (Credits).
    99,                   // SCENARIO:	Starting availability scenario.
    80,
    40, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::BAD.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_TOMAHAWK),
    None,
    ARMOR_STEEL,   // ARMOR:		Armor type
    SPEED_FLOAT,   // MOVE:			Locomotion type.
    MPH_SLOW,      // SPEED:		Miles per hour.
    1,             // ROT:			Rate of turn (degrees per tick).
    14,            // Turret center offset along body centerline.
    MISSION_GUARD, // ORDERS:		Default order to give new unit.
);

// Triceratops
const UnitTric: UnitTypeClass = UnitTypeClass::new(
    UNIT_TRIC,
    TXT_TRIC,             // NAME:			Text name of this unit type.
    "TRIC",               // NAME:			Text name of this unit type.
    ANIM_TRIC_DIE,        // EXPLOSION:	Type of explosion when destroyed.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Can this be a goodie surprise from a crate?
    true,                 // Is a leader type?
    true,                 // Only has eight facings?
    true,                 // Always use the given name for the vehicle?
    false,                //	Is this a typical transport vehicle?
    false,                // Can it be crushed by a heavy vehicle?
    true,                 // Can this unit squash infantry?
    false,                // Does this unit harvest Tiberium?
    true,                 // Is invisible to radar?
    true,                 // Is selectable by player?
    true,                 // Can it be a target for attack or move?
    false,                // Is it insignificant (won't be announced)?
    false,                // Is it immune to normal combat damage?
    false,                // Is it equipped with a combat turret?
    false,                // Fires multiple shots in quick succession?
    false,                // Can it be repaired in a repair facility?
    false,                // Can the player construct or order this unit?
    false,                // Is there a crew inside?
    false,                // Does it have a rotating radar dish?
    true,                 // Is there an associated firing animation?
    false,                // Must the turret be in a locked down position while moving?
    false,                // Does it lay tracks while moving?
    true,                 // Is this a gigundo-rotund-enormous unit?
    false,                // Is the unit's art as "chunky" cardinal facing only?
    false,                // Is the unit capable of cloaking?
    false,                // Does the unit have a constant animation?
    -1,                   // AMMO:			Number of shots it has (default).
    700,                  // STRENGTH:	Strength (in damage points).
    5,                    // SIGHTRANGE:	Range of sighting.
    0,                    // COST:			Cost to build (Credits).
    99,                   // SCENARIO:	Starting availability scenario.
    50,
    50,                // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::JP.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_STEG),
    None,
    ARMOR_STEEL,   // ARMOR:		Armor type
    SPEED_TRACK,   // MOVE:			Locomotion type.
    MPH_SLOW,      // SPEED:		Miles per hour.
    5,             // ROT:			Rate of turn (degrees per tick).
    0,             // Turret center offset along body centerline.
    MISSION_GUARD, // ORDERS:		Default order to give new unit.
);

// Tyrannosaurus Rex
const UnitTrex: UnitTypeClass = UnitTypeClass::new(
    UNIT_TREX,
    TXT_TREX,             // NAME:			Text name of this unit type.
    "TREX",               // NAME:			Text name of this unit type.
    ANIM_TREX_DIE,        // EXPLOSION:	Type of explosion when destroyed.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Can this be a goodie surprise from a crate?
    true,                 // Is a leader type?
    true,                 // Only has eight facings?
    true,                 // Always use the given name for the vehicle?
    false,                //	Is this a typical transport vehicle?
    false,                // Can it be crushed by a heavy vehicle?
    true,                 // Can this unit squash infantry?
    false,                // Does this unit harvest Tiberium?
    true,                 // Is invisible to radar?
    true,                 // Is selectable by player?
    true,                 // Can it be a target for attack or move?
    false,                // Is it insignificant (won't be announced)?
    false,                // Is it immune to normal combat damage?
    false,                // Is it equipped with a combat turret?
    false,                // Fires multiple shots in quick succession?
    false,                // Can it be repaired in a repair facility?
    false,                // Can the player construct or order this unit?
    false,                // Is there a crew inside?
    false,                // Does it have a rotating radar dish?
    true,                 // Is there an associated firing animation?
    false,                // Must the turret be in a locked down position while moving?
    false,                // Does it lay tracks while moving?
    true,                 // Is this a gigundo-rotund-enormous unit?
    false,                // Is the unit's art as "chunky" cardinal facing only?
    false,                // Is the unit capable of cloaking?
    false,                // Does the unit have a constant animation?
    -1,                   // AMMO:			Number of shots it has (default).
    750,                  // STRENGTH:	Strength (in damage points).
    5,                    // SIGHTRANGE:	Range of sighting.
    0,                    // COST:			Cost to build (Credits).
    99,                   // SCENARIO:	Starting availability scenario.
    50,
    50,                // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::JP.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_TREX),
    None,
    ARMOR_STEEL,   // ARMOR:		Armor type
    SPEED_TRACK,   // MOVE:			Locomotion type.
    MPH_MEDIUM,    // SPEED:		Miles per hour.
    5,             // ROT:			Rate of turn (degrees per tick).
    0,             // Turret center offset along body centerline.
    MISSION_GUARD, // ORDERS:		Default order to give new unit.
);

// Velociraptor
const UnitRapt: UnitTypeClass = UnitTypeClass::new(
    UNIT_RAPT,
    TXT_RAPT,             // NAME:			Text name of this unit type.
    "RAPT",               // NAME:			Text name of this unit type.
    ANIM_RAPT_DIE,        // EXPLOSION:	Type of explosion when destroyed.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Can this be a goodie surprise from a crate?
    true,                 // Is a leader type?
    true,                 // Only has eight facings?
    true,                 // Always use the given name for the vehicle?
    false,                //	Is this a typical transport vehicle?
    true,                 // Can it be crushed by a heavy vehicle?
    false,                // Can this unit squash infantry?
    false,                // Does this unit harvest Tiberium?
    true,                 // Is invisible to radar?
    true,                 // Is selectable by player?
    true,                 // Can it be a target for attack or move?
    false,                // Is it insignificant (won't be announced)?
    false,                // Is it immune to normal combat damage?
    false,                // Is it equipped with a combat turret?
    false,                // Fires multiple shots in quick succession?
    false,                // Can it be repaired in a repair facility?
    false,                // Can the player construct or order this unit?
    false,                // Is there a crew inside?
    false,                // Does it have a rotating radar dish?
    true,                 // Is there an associated firing animation?
    false,                // Must the turret be in a locked down position while moving?
    false,                // Does it lay tracks while moving?
    true,                 // Is this a gigundo-rotund-enormous unit?
    false,                // Is the unit's art as "chunky" cardinal facing only?
    false,                // Is the unit capable of cloaking?
    false,                // Does the unit have a constant animation?
    -1,                   // AMMO:			Number of shots it has (default).
    180,                  // STRENGTH:	Strength (in damage points).
    5,                    // SIGHTRANGE:	Range of sighting.
    0,                    // COST:			Cost to build (Credits).
    99,                   // SCENARIO:	Starting availability scenario.
    50,
    50,                // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::JP.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_TREX),
    None,
    ARMOR_STEEL,   // ARMOR:		Armor type
    SPEED_TRACK,   // MOVE:			Locomotion type.
    MPH_FAST,      // SPEED:		Miles per hour.
    5,             // ROT:			Rate of turn (degrees per tick).
    0,             // Turret center offset along body centerline.
    MISSION_GUARD, // ORDERS:		Default order to give new unit.
);

// Stegosaurus
const UnitSteg: UnitTypeClass = UnitTypeClass::new(
    UNIT_STEG,
    TXT_STEG,             // NAME:			Text name of this unit type.
    "STEG",               // NAME:			Text name of this unit type.
    ANIM_STEG_DIE,        // EXPLOSION:	Type of explosion when destroyed.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Can this be a goodie surprise from a crate?
    true,                 // Is a leader type?
    true,                 // Only has eight facings?
    true,                 // Always use the given name for the vehicle?
    false,                //	Is this a typical transport vehicle?
    false,                // Can it be crushed by a heavy vehicle?
    true,                 // Can this unit squash infantry?
    false,                // Does this unit harvest Tiberium?
    true,                 // Is invisible to radar?
    true,                 // Is selectable by player?
    true,                 // Can it be a target for attack or move?
    false,                // Is it insignificant (won't be announced)?
    false,                // Is it immune to normal combat damage?
    false,                // Is it equipped with a combat turret?
    false,                // Fires multiple shots in quick succession?
    false,                // Can it be repaired in a repair facility?
    false,                // Can the player construct or order this unit?
    false,                // Is there a crew inside?
    false,                // Does it have a rotating radar dish?
    true,                 // Is there an associated firing animation?
    false,                // Must the turret be in a locked down position while moving?
    false,                // Does it lay tracks while moving?
    true,                 // Is this a gigundo-rotund-enormous unit?
    false,                // Is the unit's art as "chunky" cardinal facing only?
    false,                // Is the unit capable of cloaking?
    false,                // Does the unit have a constant animation?
    -1,                   // AMMO:			Number of shots it has (default).
    600,                  // STRENGTH:	Strength (in damage points).
    5,                    // SIGHTRANGE:	Range of sighting.
    0,                    // COST:			Cost to build (Credits).
    99,                   // SCENARIO:	Starting availability scenario.
    50,
    50,                // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::JP.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_STEG),
    None,
    ARMOR_STEEL,   // ARMOR:		Armor type
    SPEED_TRACK,   // MOVE:			Locomotion type.
    MPH_SLOW,      // SPEED:		Miles per hour.
    5,             // ROT:			Rate of turn (degrees per tick).
    0,             // Turret center offset along body centerline.
    MISSION_GUARD, // ORDERS:		Default order to give new unit.
);
