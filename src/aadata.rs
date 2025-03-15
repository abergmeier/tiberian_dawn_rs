#![allow(non_snake_case, non_upper_case_globals, unused_variables)]

use crate::aircraft::AircraftType::*;
use crate::aircraft::AircraftTypeClass;
use crate::armor::ArmorType::*;
use crate::building::Flags as STRUCTF;
use crate::house::Flags as HOUSEF;
use crate::mission::MissionType::*;
use crate::speed::MPHType::*;
use crate::text::IDs::*;
use crate::weapon::WeaponType::*;

// A-10 attack plane
const AttackPlane: AircraftTypeClass = AircraftTypeClass::new(
    AIRCRAFT_A10,         // What kind of aircraft is this.
    TXT_A10,              // Translated text number for aircraft.
    "A10",                // INI name of aircraft.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Is a leader type?
    false,                // Does it fire a pair of shots in quick succession?
    false,                //	Is this a typical transport vehicle?
    true,                 // Fixed wing aircraft?
    false,                // Equipped with a rotor?
    false,                // Custom rotor sets for each facing?
    false,                // Can this aircraft land on clear terrain?
    false,                // Can the aircraft be crushed by a tracked vehicle?
    true,                 // Is it invisible on radar?
    false,                // Can the player select it so as to give it orders?
    true,                 // Can it be assigned as a target for attack.
    false,                // Is it insignificant (won't be announced)?
    false,                // Is it immune to normal combat damage?
    false,                // Theater specific graphic image?
    false,                // Can it be repaired in a repair facility?
    false,                // Can the player construct or order this unit?
    true,                 // Is there a crew inside?
    3,                    // Number of shots it has (default).
    60,                   // The strength of this unit.
    0,                    // The range that it reveals terrain around itself.
    800,                  // Credit cost to construct.
    0,                    // The scenario this becomes available.
    10,
    1, // Risk, reward when calculating AI.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::BAD.bits(), // Who can own this aircraft type.
    Some(WEAPON_NAPALM),
    None,
    ARMOR_ALUMINUM, // Armor type of this aircraft.
    MPH_FAST,       // Maximum speed of aircraft.
    5,              // Rate of turn.
    MISSION_HUNT,   // Default mission for aircraft.
);

// Transport helicopter.
const TransportHeli: AircraftTypeClass = AircraftTypeClass::new(
    AIRCRAFT_TRANSPORT,      // What kind of aircraft is this.
    TXT_TRANS,               // Translated text number for aircraft.
    "TRAN",                  // INI name of aircraft.
    6,                       // Build level.
    STRUCTF::HELIPAD.bits(), // Building prerequisite.
    false,                   // Is a leader type?
    false,                   // Does it fire a pair of shots in quick succession?
    true,                    //	Is this a typical transport vehicle?
    false,                   // Fixed wing aircraft?
    true,                    // Equipped with a rotor?
    true,                    // Custom rotor sets for each facing?
    true,                    // Can this aircraft land on clear terrain?
    false,                   // Can the aircraft be crushed by a tracked vehicle?
    true,                    // Is it invisible on radar?
    true,                    // Can the player select it so as to give it orders?
    true,                    // Can it be assigned as a target for attack.
    false,                   // Is it insignificant (won't be announced)?
    false,                   // Theater specific graphic image?
    false,                   // Is it equipped with a combat turret?
    false,                   // Can it be repaired in a repair facility?
    true,                    // Can the player construct or order this unit?
    true,                    // Is there a crew inside?
    0,                       // Number of shots it has (default).
    90,                      // The strength of this unit.
    0,                       // The range that it reveals terrain around itself.
    1500,                    // Credit cost to construct.
    98,                      // The scenario this becomes available.
    10,
    80, // Risk, reward when calculating AI.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::GOOD.bits(), // Who can own this aircraft type.
    None,
    None,
    ARMOR_ALUMINUM,  // Armor type of this aircraft.
    MPH_MEDIUM_FAST, // Maximum speed of aircraft.
    5,               // Rate of turn.
    MISSION_HUNT,    // Default mission for aircraft.
);

// Apache attach helicopter.
const AttackHeli: AircraftTypeClass = AircraftTypeClass::new(
    AIRCRAFT_HELICOPTER,     // What kind of aircraft is this.
    TXT_HELI,                // Translated text number for aircraft.
    "HELI",                  // INI name of aircraft.
    6,                       // Build level.
    STRUCTF::HELIPAD.bits(), // Building prerequisite.
    true,                    // Is a leader type?
    true,                    // Does it fire a pair of shots in quick succession?
    false,                   //	Is this a typical transport vehicle?
    false,                   // Fixed wing aircraft?
    true,                    // Equipped with a rotor?
    false,                   // Custom rotor sets for each facing?
    false,                   // Can this aircraft land on clear terrain?
    false,                   // Can the aircraft be crushed by a tracked vehicle?
    true,                    // Is it invisible on radar?
    true,                    // Can the player select it so as to give it orders?
    true,                    // Can it be assigned as a target for attack.
    false,                   // Is it insignificant (won't be announced)?
    false,                   // Is it immune to normal combat damage?
    false,                   // Theater specific graphic image?
    false,                   // Can it be repaired in a repair facility?
    true,                    // Can the player construct or order this unit?
    true,                    // Is there a crew inside?
    15,                      // Number of shots it has (default).
    125,                     // The strength of this unit.
    0,                       // The range that it reveals terrain around itself.
    1200,                    // Credit cost to construct.
    10,                      // The scenario this becomes available.
    10,
    80, // Risk, reward when calculating AI.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits(), // Who can own this aircraft type.
    Some(WEAPON_CHAIN_GUN),
    None,
    ARMOR_STEEL,  // Armor type of this aircraft.
    MPH_FAST,     // Maximum speed of aircraft.
    4,            // Rate of turn.
    MISSION_HUNT, // Default mission for aircraft.
);

// Orca attack helicopter.
const OrcaHeli: AircraftTypeClass = AircraftTypeClass::new(
    AIRCRAFT_ORCA,           // What kind of aircraft is this.
    TXT_ORCA,                // Translated text number for aircraft.
    "ORCA",                  // INI name of aircraft.
    6,                       // Build level.
    STRUCTF::HELIPAD.bits(), // Building prerequisite.
    true,                    // Is a leader type?
    true,                    // Does it fire a pair of shots in quick succession?
    false,                   //	Is this a typical transport vehicle?
    false,                   // Fixed wing aircraft?
    false,                   // Equipped with a rotor?
    false,                   // Custom rotor sets for each facing?
    false,                   // Can this aircraft land on clear terrain?
    false,                   // Can the aircraft be crushed by a tracked vehicle?
    true,                    // Is it invisible on radar?
    true,                    // Can the player select it so as to give it orders?
    true,                    // Can it be assigned as a target for attack.
    false,                   // Is it insignificant (won't be announced)?
    false,                   // Is it immune to normal combat damage?
    false,                   // Theater specific graphic image?
    false,                   // Can it be repaired in a repair facility?
    true,                    // Can the player construct or order this unit?
    true,                    // Is there a crew inside?
    6,                       // Number of shots it has (default).
    125,                     // The strength of this unit.
    0,                       // The range that it reveals terrain around itself.
    1200,                    // Credit cost to construct.
    10,                      // The scenario this becomes available.
    10,
    80, // Risk, reward when calculating AI.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits(), // Who can own this aircraft type.
    Some(WEAPON_DRAGON),
    None,
    ARMOR_STEEL,  // Armor type of this aircraft.
    MPH_FAST,     // Maximum speed of aircraft.
    4,            // Rate of turn.
    MISSION_HUNT, // Default mission for aircraft.
);

// C-17 transport plane.
const CargoPlane: AircraftTypeClass = AircraftTypeClass::new(
    AIRCRAFT_CARGO,       // What kind of aircraft is this.
    TXT_C17,              // Translated text number for aircraft.
    "C17",                // INI name of aircraft.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Is a leader type?
    false,                // Does it fire a pair of shots in quick succession?
    true,                 //	Is this a typical transport vehicle?
    true,                 // Fixed wing aircraft?
    false,                // Equipped with a rotor?
    false,                // Custom rotor sets for each facing?
    false,                // Can this aircraft land on clear terrain?
    false,                // Can the aircraft be crushed by a tracked vehicle?
    true,                 // Is it invisible on radar?
    false,                // Can the player select it so as to give it orders?
    false,                // Can it be assigned as a target for attack.
    false,                // Is it insignificant (won't be announced)?
    false,                // Is it immune to normal combat damage?
    false,                // Theater specific graphic image?
    false,                // Can it be repaired in a repair facility?
    false,                // Can the player construct or order this unit?
    true,                 // Is there a crew inside?
    0,                    // Number of shots it has (default).
    25,                   // The strength of this unit.
    0,                    // The range that it reveals terrain around itself.
    800,                  // Credit cost to construct.
    0,                    // The scenario this becomes available.
    10,
    1, // Risk, reward when calculating AI.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::BAD.bits(), // Who can own this aircraft type.
    None,
    None,
    ARMOR_ALUMINUM, // Armor type of this aircraft.
    MPH_FAST,       // Maximum speed of aircraft.
    5,              // Rate of turn.
    MISSION_HUNT,   // Default mission for aircraft.
);
