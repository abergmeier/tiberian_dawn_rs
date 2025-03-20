#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_variables)]

use crate::armor::ArmorType::*;
use crate::building::BSizeType::*;
use crate::building::BuildingTypeClass;
use crate::building::StructType::*;
use crate::building::STRUCTF;
use crate::coords::XYP_COORD;
use crate::direction::*;
use crate::display::*;
use crate::house::HOUSEF;
use crate::map::MAP_CELL_W;
use crate::map::MCW;
use crate::rtti::RTTIType::*;
use crate::text::IDs::*;
use crate::weapon::WeaponType::*;
use crate::ADVANCED;
use crate::PATCH;
use crate::REFRESH_EOL;

macro_rules! XYCELL {
    ($x:expr, $y:expr) => {
        $y * MAP_CELL_W as i16 + $x
    };
}
const ExitPyle: [i16; 14] = [
    XYCELL!(1, 2),
    XYCELL!(2, 2),
    XYCELL!(0, 2),
    XYCELL!(-1, 2),
    XYCELL!(-1, -1),
    XYCELL!(0, -1),
    XYCELL!(1, -1),
    XYCELL!(2, -1),
    XYCELL!(2, -1),
    XYCELL!(-1, 0),
    XYCELL!(2, 0),
    XYCELL!(2, 1),
    XYCELL!(-1, 1),
    REFRESH_EOL as i16,
];
const ExitHand: [i16; 13] = [
    XYCELL!(2, 3),
    XYCELL!(1, 3),
    XYCELL!(0, 3),
    XYCELL!(2, 2),
    XYCELL!(-1, 3),
    XYCELL!(-1, 2),
    XYCELL!(0, 0),
    XYCELL!(1, 0),
    XYCELL!(-1, 0),
    XYCELL!(2, 0),
    XYCELL!(2, 1),
    XYCELL!(-1, 1),
    REFRESH_EOL as i16,
];
const ExitWeap: [i16; 10] = [
    XYCELL!(-1, 3),
    XYCELL!(0, 3),
    XYCELL!(-1, 2),
    XYCELL!(1, 3),
    //	XYCELL!(0,0),
    //	XYCELL!(1,0),
    //	XYCELL!(2,0),
    //	XYCELL!(-1,0),
    //	XYCELL!(3,0),
    XYCELL!(-1, 1),
    XYCELL!(3, 1),
    XYCELL!(3, 2),
    XYCELL!(3, 3),
    XYCELL!(2, 3),
    REFRESH_EOL as i16,
];
const ExitAirstrip: [i16; 17] = [
    XYCELL!(-1, -1),
    XYCELL!(-1, 0),
    XYCELL!(-1, 1),
    XYCELL!(-1, 2),
    XYCELL!(0, -1),
    XYCELL!(0, 2),
    XYCELL!(1, -1),
    XYCELL!(1, 2),
    XYCELL!(2, -1),
    XYCELL!(2, 2),
    XYCELL!(3, -1),
    XYCELL!(3, 2),
    XYCELL!(4, -1),
    XYCELL!(4, 0),
    XYCELL!(4, 1),
    XYCELL!(4, 2),
    REFRESH_EOL as i16,
];

/***************************************************************************
 */
const OListSAM: [i16; 3] = [-(MCW as i16), -((MCW - 1) as i16), REFRESH_EOL as i16];
const List32: [i16; 7] = [
    0,
    1,
    2,
    MCW as i16,
    MCW as i16 + 1,
    MCW as i16 + 2,
    REFRESH_EOL as i16,
];
const List22_0011: [i16; 3] = [MCW as i16, MCW as i16 + 1, REFRESH_EOL as i16];
const List22_1100: [i16; 3] = [0, 1, REFRESH_EOL as i16];
const ListFix: [i16; 6] = [
    1,
    MCW as i16,
    MCW as i16 + 1,
    MCW as i16 + 2,
    (MCW + MCW + 1) as i16,
    REFRESH_EOL as i16,
];
const StoreList: [i16; 3] = [0, 1, REFRESH_EOL as i16];
const List2: [i16; 5] = [0, 1, MCW as i16 + 1, MCW as i16, REFRESH_EOL as i16];
const List42: [i16; 9] = [
    0,
    1,
    2,
    3,
    MCW as i16,
    MCW as i16 + 1,
    MCW as i16 + 2,
    MCW as i16 + 3,
    REFRESH_EOL as i16,
];
const ListWestwood: [i16; 7] = [
    1,
    2,
    3,
    MCW as i16 + 1,
    MCW as i16 + 2,
    MCW as i16 + 3,
    REFRESH_EOL as i16,
];
const OListWestwood: [i16; 3] = [0, MCW as i16, REFRESH_EOL as i16];
const ComList: [i16; 4] = [0, MCW as i16, (MCW + 1) as i16, REFRESH_EOL as i16];
const List21: [i16; 3] = [0, 1, REFRESH_EOL as i16];
const ListWeap: [i16; 7] = [
    (MCW * 1) as i16,
    (MCW * 1 + 1) as i16,
    (MCW * 1 + 2) as i16,
    (MCW * 2) as i16,
    (MCW * 2 + 1) as i16,
    (MCW * 2 + 2) as i16,
    REFRESH_EOL as i16,
];
const List12: [i16; 2] = [MCW as i16, REFRESH_EOL as i16];
const ListHand: [i16; 4] = [
    MCW as i16,
    (MCW + 1) as i16,
    (MCW * 2 + 1) as i16,
    REFRESH_EOL as i16,
];
const ListTmpl: [i16; 7] = [
    MCW as i16,
    (MCW + 1) as i16,
    (MCW + 2) as i16,
    (MCW * 2) as i16,
    (MCW * 2 + 1) as i16,
    (MCW * 2 + 2) as i16,
    REFRESH_EOL as i16,
];
const List0011: [i16; 3] = [(MCW * 1) as i16, (MCW * 1 + 1) as i16, REFRESH_EOL as i16];
const List1101: [i16; 4] = [0, 1, (MCW * 1 + 1) as i16, REFRESH_EOL as i16];
const List11: [i16; 3] = [0, 1, REFRESH_EOL as i16];
const List1: [i16; 2] = [0, REFRESH_EOL as i16];
const List1100: [i16; 3] = [0, 1, REFRESH_EOL as i16];
const List0010: [i16; 2] = [MCW as i16, REFRESH_EOL as i16];
const List1000: [i16; 2] = [0, REFRESH_EOL as i16];
const List0100: [i16; 2] = [1, REFRESH_EOL as i16];
const List0111: [i16; 4] = [
    1,
    (MCW * 1) as i16,
    (MCW * 1 + 1) as i16,
    REFRESH_EOL as i16,
];
//const List1111: [i16; 5] = [0, 1, (MCW*1), (MCW*1)+1, REFRESH_EOL];
const List1011: [i16; 4] = [
    0,
    (MCW * 1) as i16,
    (MCW * 1 + 1) as i16,
    REFRESH_EOL as i16,
];
const List010111000: [i16; 5] = [
    1,
    (MCW * 1) as i16,
    (MCW * 1 + 1) as i16,
    (MCW * 1 + 2) as i16,
    REFRESH_EOL as i16,
];
const List101000111: [i16; 6] = [
    0,
    2,
    (MCW * 2) as i16,
    (MCW * 2 + 1) as i16,
    (MCW * 2 + 2) as i16,
    REFRESH_EOL as i16,
];

const OListFix: [i16; 5] = [
    0,
    2,
    (MCW + MCW) as i16,
    (MCW + MCW + 2) as i16,
    REFRESH_EOL as i16,
];
const OListWeap: [i16; 4] = [0, 1, 2, REFRESH_EOL as i16];
const OComList: [i16; 2] = [1, REFRESH_EOL as i16];
const OList12: [i16; 2] = [0, REFRESH_EOL as i16];
const OListHand: [i16; 5] = [0, 1, (MCW * 2) as i16, (MCW * 1) as i16, REFRESH_EOL as i16];
const OListTmpl: [i16; 4] = [0, 1, 2, REFRESH_EOL as i16];

const ClassTemple: BuildingTypeClass<0, 7, 4> = BuildingTypeClass::new(
    STRUCT_TEMPLE,
    TXT_TEMPLE,              // NAME:			Short name of the structure.
    "TMPL",                  // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    7,                       // Build level.
    STRUCTF::RADAR.bits(),   // Building prerequisite.
    false,                   // Has ability to detect adjacent cloaked objects?
    false,                   // Animation rate is regulated for constant speed?
    true,                    // Requires a bib dirt patch?
    false,                   // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    false,                   // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    false,                   // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    true,                    // Can it be repaired?
    true,                    // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    1000,                    // STRNTH:		Full strength of building.
    4,                       // SIGHTRANGE:	Range of sighting.
    3000,                    // COST:			Cost to purchase.
    13,                      // SCENARIO:	Starting availability scenario.
    0,
    20, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_ALUMINUM), // ARMOR:		Armor type
    0,                    // CANENTER:	Units that can enter building.
    0,                    // CAPACITY:	Spice storage capacity.
    0,                    // POWER:		Power points generated.
    150,                  // DRAIN:		Power points required.
    BSIZE_33,             // SIZE:			Building size.
    [],                   // Preferred exit cell list.
    ListTmpl,             // OCCUPYLIST:	List of active foundation squares.
    OListTmpl,            // OVERLAPLIST:List of overlap cell offset.
);

const ClassEye: BuildingTypeClass<0, 4, 2> = BuildingTypeClass::new(
    STRUCT_EYE,
    TXT_EYE,                 // NAME:			Short name of the structure.
    "EYE",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    7,                       // Build level.
    STRUCTF::RADAR.bits(),   // Building prerequisite.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    true,                    // Requires a bib dirt patch?
    false,                   // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    true,                    // Can this building be captured?
    false,                   // Does it catch fire?
    false,                   // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    false,                   // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    true,                    // Can it be repaired?
    true,                    // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    160,                     // Starting idle frame to match construction.
    500,                     // STRNTH:		Full strength of building.
    10,                      // SIGHTRANGE:	Range of sighting.
    2800,                    // COST:			Cost to purchase.
    13,                      // SCENARIO:	Starting availability scenario.
    0,
    100, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    200,              // DRAIN:		Power points required.
    BSIZE_22,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    ComList,          // OCCUPYLIST:	List of active foundation squares.
    OComList,         // OVERLAPLIST:List of overlap cell offset.
);

const ClassWeapon: BuildingTypeClass<10, 7, 4> = BuildingTypeClass::new(
    STRUCT_WEAP,
    TXT_WEAPON_FACTORY, // NAME:			Short name of the structure.
    "WEAP",             // NAME:			Short name of the structure.
    XYP_COORD!(
        10 + (CELL_PIXEL_W / 2),
        ((CELL_PIXEL_H * 3) - (CELL_PIXEL_H / 2)) - 21
    ) as u64, // Exit point for produced units.
    2,                  // Build level.
    STRUCTF::REFINERY.bits(), // Building prerequisite.
    false,              // Has ability to detect adjacent cloaked objects?
    false,              // Animation rate is regulated for constant speed?
    true,               // Requires a bib dirt patch?
    false,              // Always use the given name for the building?
    false,              // Is this a wall type structure?
    true,               // Is it a factory type building?
    true,               // Can this building be captured?
    false,              // Does it catch fire?
    false,              // Simple (one frame) damage imagery?
    false,              // Is it invisible to radar?
    true,               // Can the player select this?
    true,               // Is this a legal target for attack or move?
    false,              // Is this an insignificant building?
    false,              // Is it immune to normal combat damage?
    false,              // Theater specific graphic image?
    false,              // Does it have a rotating turret?
    false,              // Fires multiple shots in quick succession?
    true,               // Can it be repaired?
    true,               // Can it be manufactured by the player?
    true,               // Does it contain a crew?
    false,              // Does building care less if placed on concrete?
    Some(RTTI_UNITTYPE), // The object type produced at this factory.
    DIR_N,              // Starting idle frame to match construction.
    if ADVANCED { 500 } else { 200 }, // STRNTH:		Full strength of building.
    3,                  // SIGHTRANGE:	Range of sighting.
    2000,               // COST:			Cost to purchase.
    5,                  // SCENARIO:	Starting availability scenario.
    0,
    86, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_ALUMINUM), // ARMOR:		Armor type
    0,                    // CANENTER:	Units that can enter building.
    0,                    // CAPACITY:	Spice storage capacity.
    0,                    // POWER:		Power points required.
    30,                   // DRAIN:		Power points required.
    BSIZE_33,             // SIZE:			Building size.
    ExitWeap,             // Preferred exit cell list.
    ListWeap,             // OCCUPYLIST:	List of active foundation squares.
    OListWeap,            // OVERLAPLIST:List of overlap cell offset.
);

const ClassGTower: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_GTOWER,
    TXT_GUARD_TOWER,          // NAME:			Short name of the structure.
    "GTWR",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64,  // Exit point for produced units.
    2,                        // Build level.
    STRUCTF::BARRACKS.bits(), // Building prerequisite.
    true,                     // Has ability to detect adjacent cloaked objects?
    false,                    // Animation rate is regulated for constant speed?
    false,                    // Requires a bib dirt patch?
    false,                    // Always use the given name for the building?
    false,                    // Is this a wall type structure?
    false,                    // Is it a factory type building?
    false,                    // Can this building be captured?
    false,                    // Does it catch fire?
    true,                     // Simple (one frame) damage imagery?
    false,                    // Is it invisible to radar?
    true,                     // Can the player select this?
    true,                     // Is this a legal target for attack or move?
    false,                    // Is this an insignificant building?
    false,                    // Is it immune to normal combat damage?
    false,                    // Theater specific graphic image?
    false,                    // Does it have a rotating turret?
    false,                    // Fires multiple shots in quick succession?
    true,                     // Can it be repaired?
    true,                     // Can it be manufactured by the player?
    true,                     // Does it contain a crew?
    false,                    // Does building care less if placed on concrete?
    None,                     // The object type produced at this factory.
    DIR_N,                    // Starting idle frame to match construction.
    200,                      // STRNTH:		Full strength of building.
    3,                        // SIGHTRANGE:	Range of sighting.
    500,                      // COST:			Cost to purchase.
    7,                        // SCENARIO:	Starting availability scenario.
    100,
    25, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_CHAIN_GUN),
    None,
    //	WEAPON_M60MG,None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    00,               // POWER:		Power points required.
    10,               // DRAIN:		Power points required.
    BSIZE_11,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassATower: BuildingTypeClass<0, 2, 2> = BuildingTypeClass::new(
    STRUCT_ATOWER,
    TXT_AGUARD_TOWER,        // NAME:			Short name of the structure.
    "ATWR",                  // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    4,                       // Build level.
    STRUCTF::RADAR.bits(),   // Building prerequisite.
    true,                    // Has ability to detect adjacent cloaked objects?
    false,                   // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    false,                   // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    false,                   // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    false,                   // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    true,                    // Fires multiple shots in quick succession?
    true,                    // Can it be repaired?
    true,                    // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    300,                     // STRNTH:		Full strength of building.
    4,                       // SIGHTRANGE:	Range of sighting.
    1000,                    // COST:			Cost to purchase.
    13,                      // SCENARIO:	Starting availability scenario.
    100,
    30, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_TOW_TWO),
    None,
    //	WEAPON_TOMAHAWK,None,
    Some(ARMOR_ALUMINUM), // ARMOR:		Armor type
    0,                    // CANENTER:	Units that can enter building.
    0,                    // CAPACITY:	Spice storage capacity.
    0,                    // POWER:		Power points required.
    20,                   // DRAIN:		Power points required.
    BSIZE_12,             // SIZE:			Building size.
    [],                   // Preferred exit cell list.
    List12,               // OCCUPYLIST:	List of active foundation squares.
    OList12,              // OVERLAPLIST:List of overlap cell offset.
);

const ClassObelisk: BuildingTypeClass<0, 2, 2> = BuildingTypeClass::new(
    STRUCT_OBELISK,
    TXT_OBELISK,             // NAME:			Short name of the structure.
    "OBLI",                  // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    4,                       // Build level.
    STRUCTF::RADAR.bits(),   // Building prerequisite.
    true,                    // Has ability to detect adjacent cloaked objects?
    false,                   // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    false,                   // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    false,                   // Does it catch fire?
    false,                   // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    false,                   // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    true,                    // Can it be repaired?
    true,                    // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    5,                       // SIGHTRANGE:	Range of sighting.
    1500,                    // COST:			Cost to purchase.
    11,                      // SCENARIO:	Starting availability scenario.
    100,
    35, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_OBELISK_LASER),
    None,
    Some(ARMOR_ALUMINUM), // ARMOR:		Armor type
    0,                    // CANENTER:	Units that can enter building.
    0,                    // CAPACITY:	Spice storage capacity.
    0,                    // POWER:		Power points required.
    150,                  // DRAIN:		Power points required.
    BSIZE_12,             // SIZE:			Building size.
    [],                   // Preferred exit cell list.
    List12,               // OCCUPYLIST:	List of active foundation squares.
    OList12,              // OVERLAPLIST:List of overlap cell offset.
);

const ClassTurret: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_TURRET,
    TXT_TURRET,               // NAME:			Short name of the structure.
    "GUN",                    // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64,  // Exit point for produced units.
    2,                        // Build level.
    STRUCTF::BARRACKS.bits(), // Building prerequisite.
    true,                     // Has ability to detect adjacent cloaked objects?
    false,                    // Animation rate is regulated for constant speed?
    false,                    // Requires a bib dirt patch?
    false,                    // Always use the given name for the building?
    false,                    // Is this a wall type structure?
    false,                    // Is it a factory type building?
    false,                    // Can this building be captured?
    false,                    // Does it catch fire?
    false,                    // Simple (one frame) damage imagery?
    false,                    // Is it invisible to radar?
    true,                     // Can the player select this?
    true,                     // Is this a legal target for attack or move?
    false,                    // Is this an insignificant building?
    false,                    // Is it immune to normal combat damage?
    false,                    // Theater specific graphic image?
    true,                     // Does it have a rotating turret?
    false,                    // Fires multiple shots in quick succession?
    true,                     // Can it be repaired?
    true,                     // Can it be manufactured by the player?
    true,                     // Does it contain a crew?
    false,                    // Does building care less if placed on concrete?
    None,                     // The object type produced at this factory.
    208,                      // Starting idle frame to match construction.
    200,                      // STRNTH:		Full strength of building.
    5,                        // SIGHTRANGE:	Range of sighting.
    if ADVANCED {
        600 // COST:			Cost to purchase.
    } else if PATCH {
        600
    } else {
        250 // COST:			Cost to purchase.
    },
    8, // SCENARIO:	Starting availability scenario.
    300,
    26, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_TURRET_GUN),
    None,
    Some(ARMOR_STEEL), // ARMOR:		Armor type
    0,                 // CANENTER:	Units that can enter building.
    0,                 // CAPACITY:	Spice storage capacity.
    0,                 // POWER:		Power points required.
    20,                // DRAIN:		Power points required.
    BSIZE_11,          // SIZE:			Building size.
    [],                // Preferred exit cell list.
    List1,             // OCCUPYLIST:	List of active foundation squares.
    [],                // OVERLAPLIST:List of overlap cell offset.
);

const ClassConst: BuildingTypeClass<0, 7, 0> = BuildingTypeClass::new(
    STRUCT_CONST,
    TXT_CONST_YARD,          // NAME:			Short name of the structure.
    "FACT",                  // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // Building prerequisite.
    false,                   // Has ability to detect adjacent cloaked objects?
    false,                   // Animation rate is regulated for constant speed?
    true,                    // Requires a bib dirt patch?
    false,                   // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    true,                    // Is it a factory type building?
    true,                    // Can this building be captured?
    false,                   // Does it catch fire?
    false,                   // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    false,                   // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    true,                    // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    Some(RTTI_BUILDINGTYPE), // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    400,                     // STRNTH:		Full strength of building.
    3,                       // SIGHTRANGE:	Range of sighting.
    5000,                    // COST:			Cost to purchase.
    1,                       // SCENARIO:	Starting availability scenario.
    0,
    70, // RISK/RWRD:	Risk/reward rating values.
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
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    30,               // POWER:		Power points required.
    15,               // DRAIN:		Power points required.
    BSIZE_32,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List32,           // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassRefinery: BuildingTypeClass<0, 5, 6> = BuildingTypeClass::new(
    STRUCT_REFINERY,
    TXT_REFINERY,            // NAME:			Short name of the structure.
    "PROC",                  // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    1,                       // Build level.
    STRUCTF::POWER.bits(),   // Building prerequisite.
    false,                   // Has ability to detect adjacent cloaked objects?
    false,                   // Animation rate is regulated for constant speed?
    true,                    // Requires a bib dirt patch?
    false,                   // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    true,                    // Can this building be captured?
    false,                   // Does it catch fire?
    false,                   // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    false,                   // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    true,                    // Can it be repaired?
    true,                    // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    450,                     // STRNTH:		Full strength of building.
    4,                       // SIGHTRANGE:	Range of sighting.
    2000,                    // COST:			Cost to purchase.
    2,                       // SCENARIO:	Starting availability scenario.
    0,
    55, // RISK/RWRD:	Risk/reward rating values.
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
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    1000,             // CAPACITY:	Spice storage capacity.
    10,               // POWER:		Power points required.
    40,               // DRAIN:		Power points required.
    BSIZE_33,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List010111000,    // OCCUPYLIST:	List of active foundation squares.
    List101000111,    // OVERLAPLIST:List of overlap cell offset.
);

const ClassStorage: BuildingTypeClass<0, 3, 0> = BuildingTypeClass::new(
    STRUCT_STORAGE,
    TXT_STORAGE,              // NAME:			Short name of the structure.
    "SILO",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64,  // Exit point for produced units.
    1,                        // Build level.
    STRUCTF::REFINERY.bits(), // Building prerequisite.
    false,                    // Has ability to detect adjacent cloaked objects?
    false,                    // Animation rate is regulated for constant speed?
    true,                     // Requires a bib dirt patch?
    false,                    // Always use the given name for the building?
    false,                    // Is this a wall type structure?
    false,                    // Is it a factory type building?
    true,                     // Can this building be captured?
    false,                    // Does it catch fire?
    true,                     // Simple (one frame) damage imagery?
    false,                    // Is it invisible to radar?
    true,                     // Can the player select this?
    true,                     // Is this a legal target for attack or move?
    false,                    // Is this an insignificant building?
    false,                    // Is it immune to normal combat damage?
    false,                    // Theater specific graphic image?
    false,                    // Does it have a rotating turret?
    false,                    // Fires multiple shots in quick succession?
    true,                     // Can it be repaired?
    true,                     // Can it be manufactured by the player?
    false,                    // Does it contain a crew?
    false,                    // Does building care less if placed on concrete?
    None,                     // The object type produced at this factory.
    DIR_N,                    // Starting idle frame to match construction.
    150,                      // STRNTH:		Full strength of building.
    2,                        // SIGHTRANGE:	Range of sighting.
    150,                      // COST:			Cost to purchase.
    //	300,										// COST:			Cost to purchase.
    2, // SCENARIO:	Starting availability scenario.
    0,
    16, // RISK/RWRD:	Risk/reward rating values.
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
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    1500,             // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    10,               // DRAIN:		Power points required.
    BSIZE_21,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    StoreList,        // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassHelipad: BuildingTypeClass<0, 5, 0> = BuildingTypeClass::new(
    STRUCT_HELIPAD,
    TXT_HELIPAD,              // NAME:			Short name of the structure.
    "HPAD",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64,  // Exit point for produced units.
    6,                        // Build level.
    STRUCTF::BARRACKS.bits(), // Building prerequisite.
    false,                    // Has ability to detect adjacent cloaked objects?
    false,                    // Animation rate is regulated for constant speed?
    true,                     // Requires a bib dirt patch?
    false,                    // Always use the given name for the building?
    false,                    // Is this a wall type structure?
    false,                    // Is it a factory type building?
    true,                     // Can this building be captured?
    false,                    // Does it catch fire?
    false,                    // Simple (one frame) damage imagery?
    false,                    // Is it invisible to radar?
    true,                     // Can the player select this?
    true,                     // Is this a legal target for attack or move?
    false,                    // Is this an insignificant building?
    false,                    // Is it immune to normal combat damage?
    false,                    // Theater specific graphic image?
    false,                    // Does it have a rotating turret?
    false,                    // Fires multiple shots in quick succession?
    true,                     // Can it be repaired?
    true,                     // Can it be manufactured by the player?
    false,                    // Does it contain a crew?
    false,                    // Does building care less if placed on concrete?
    Some(RTTI_AIRCRAFTTYPE),  // The object type produced at this factory.
    DIR_N,                    // Starting idle frame to match construction.
    400,                      // STRNTH:		Full strength of building.
    3,                        // SIGHTRANGE:	Range of sighting.
    1500,                     // COST:			Cost to purchase.
    10,                       // SCENARIO:	Starting availability scenario.
    0,
    65, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    10,               // DRAIN:		Power points required.
    BSIZE_22,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List2,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassCommand: BuildingTypeClass<0, 4, 2> = BuildingTypeClass::new(
    STRUCT_RADAR,
    TXT_COMMAND,              // NAME:			Short name of the structure.
    "HQ",                     // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64,  // Exit point for produced units.
    2,                        // Build level.
    STRUCTF::REFINERY.bits(), // Building prerequisite.
    true,                     // Has ability to detect adjacent cloaked objects?
    true,                     // Animation rate is regulated for constant speed?
    true,                     // Requires a bib dirt patch?
    false,                    // Always use the given name for the building?
    false,                    // Is this a wall type structure?
    false,                    // Is it a factory type building?
    true,                     // Can this building be captured?
    false,                    // Does it catch fire?
    false,                    // Simple (one frame) damage imagery?
    false,                    // Is it invisible to radar?
    true,                     // Can the player select this?
    true,                     // Is this a legal target for attack or move?
    false,                    // Is this an insignificant building?
    false,                    // Is it immune to normal combat damage?
    false,                    // Theater specific graphic image?
    false,                    // Does it have a rotating turret?
    false,                    // Fires multiple shots in quick succession?
    true,                     // Can it be repaired?
    true,                     // Can it be manufactured by the player?
    true,                     // Does it contain a crew?
    false,                    // Does building care less if placed on concrete?
    None,                     // The object type produced at this factory.
    160,                      // Starting idle frame to match construction.
    500,                      // STRNTH:		Full strength of building.
    10,                       // SIGHTRANGE:	Range of sighting.
    1000,                     // COST:			Cost to purchase.
    3,                        // SCENARIO:	Starting availability scenario.
    0,
    20, // RISK/RWRD:	Risk/reward rating values.
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
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    40,               // DRAIN:		Power points required.
    BSIZE_22,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    ComList,          // OCCUPYLIST:	List of active foundation squares.
    OComList,         // OVERLAPLIST:List of overlap cell offset.
);

const ClassSAM: BuildingTypeClass<0, 3, 3> = BuildingTypeClass::new(
    STRUCT_SAM,
    TXT_SAM,                  // NAME:			Short name of the structure.
    "SAM",                    // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64,  // Exit point for produced units.
    6,                        // Build level.
    STRUCTF::BARRACKS.bits(), // Building prerequisite.
    false,                    // Has ability to detect adjacent cloaked objects?
    false,                    // Animation rate is regulated for constant speed?
    false,                    // Requires a bib dirt patch?
    false,                    // Always use the given name for the building?
    false,                    // Is this a wall type structure?
    false,                    // Is it a factory type building?
    false,                    // Can this building be captured?
    false,                    // Does it catch fire?
    false,                    // Simple (one frame) damage imagery?
    false,                    // Is it invisible to radar?
    true,                     // Can the player select this?
    true,                     // Is this a legal target for attack or move?
    false,                    // Is this an insignificant building?
    false,                    // Is it immune to normal combat damage?
    false,                    // Theater specific graphic image?
    true,                     // Does it have a rotating turret?
    true,                     // Fires multiple shots in quick succession?
    true,                     // Can it be repaired?
    true,                     // Can it be manufactured by the player?
    false,                    // Does it contain a crew?
    false,                    // Does building care less if placed on concrete?
    None,                     // The object type produced at this factory.
    DIR_N,                    // Starting idle frame to match construction.
    200,                      // STRNTH:		Full strength of building.
    3,                        // SIGHTRANGE:	Range of sighting.
    750,                      // COST:			Cost to purchase.
    5,                        // SCENARIO:	Starting availability scenario.
    300,
    40, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits(), // OWNABLE:		Ownable by house (bit field).
    Some(WEAPON_NIKE),
    None,
    Some(ARMOR_STEEL), // ARMOR:		Armor type
    0,                 // CANENTER:	Units that can enter building.
    0,                 // CAPACITY:	Spice storage capacity.
    0,                 // POWER:		Power points required.
    20,                // DRAIN:		Power points required.
    BSIZE_21,          // SIZE:			Building size.
    [],                // Preferred exit cell list.
    List21,            // OCCUPYLIST:	List of active foundation squares.
    OListSAM,          // OVERLAPLIST:List of overlap cell offset.
);

const ClassAirStrip: BuildingTypeClass<17, 9, 0> = BuildingTypeClass::new(
    STRUCT_AIRSTRIP,
    TXT_AIRSTRIP,             // NAME:			Short name of the structure.
    "AFLD",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64,  // Exit point for produced units.
    2,                        // Build level.
    STRUCTF::REFINERY.bits(), // Building prerequisite.
    false,                    // Has ability to detect adjacent cloaked objects?
    true,                     // Animation rate is regulated for constant speed?
    true,                     // Requires a bib dirt patch?
    false,                    // Always use the given name for the building?
    false,                    // Is this a wall type structure?
    true,                     // Is it a factory type building?
    true,                     // Can this building be captured?
    false,                    // Does it catch fire?
    false,                    // Simple (one frame) damage imagery?
    false,                    // Is it invisible to radar?
    true,                     // Can the player select this?
    true,                     // Is this a legal target for attack or move?
    false,                    // Is this an insignificant building?
    false,                    // Is it immune to normal combat damage?
    false,                    // Theater specific graphic image?
    false,                    // Does it have a rotating turret?
    false,                    // Fires multiple shots in quick succession?
    true,                     // Can it be repaired?
    true,                     // Can it be manufactured by the player?
    true,                     // Does it contain a crew?
    false,                    // Does building care less if placed on concrete?
    Some(RTTI_UNITTYPE),      // The object type produced at this factory.
    DIR_N,                    // Starting idle frame to match construction.
    500,                      // STRNTH:		Full strength of building.
    5,                        // SIGHTRANGE:	Range of sighting.
    2000,                     // COST:			Cost to purchase.
    5,                        // SCENARIO:	Starting availability scenario.
    300,
    86, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_STEEL), // ARMOR:		Armor type
    0,                 // CANENTER:	Units that can enter building.
    0,                 // CAPACITY:	Spice storage capacity.
    0,                 // POWER:		Power points required.
    30,                // DRAIN:		Power points required.
    BSIZE_42,          // SIZE:			Building size.
    ExitAirstrip,      // Preferred exit cell list.
    List42,            // OCCUPYLIST:	List of active foundation squares.
    [],                // OVERLAPLIST:List of overlap cell offset.
);

const ClassPower: BuildingTypeClass<0, 4, 2> = BuildingTypeClass::new(
    STRUCT_POWER,
    TXT_POWER,               // NAME:			Short name of the structure.
    "NUKE",                  // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    0,                       // Build level.
    STRUCTF::NONE.bits(),    // Building prerequisite.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    true,                    // Requires a bib dirt patch?
    false,                   // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    true,                    // Can this building be captured?
    false,                   // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    false,                   // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    true,                    // Can it be repaired?
    true,                    // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    300,                     // COST:			Cost to purchase.
    1,                       // SCENARIO:	Starting availability scenario.
    0,
    50, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    100,              // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_22,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1011,         // OCCUPYLIST:	List of active foundation squares.
    List0100,         // OVERLAPLIST:List of overlap cell offset.
);

const ClassAdvancedPower: BuildingTypeClass<0, 4, 2> = BuildingTypeClass::new(
    STRUCT_ADVANCED_POWER,
    TXT_ADVANCED_POWER,      // NAME:			Short name of the structure.
    "NUK2",                  // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    5,                       // Build level.
    STRUCTF::POWER.bits(),   // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    true,                    // Requires a bib dirt patch?
    false,                   // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    true,                    // Can this building be captured?
    false,                   // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    false,                   // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    true,                    // Can it be repaired?
    true,                    // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    300,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    700,                     // COST:			Cost to purchase.
    13,                      // SCENARIO:	Starting availability scenario.
    0,
    75, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    200,              // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_22,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1011,         // OCCUPYLIST:	List of active foundation squares.
    List0100,         // OVERLAPLIST:List of overlap cell offset.
);

const ClassHospital: BuildingTypeClass<0, 5, 0> = BuildingTypeClass::new(
    STRUCT_HOSPITAL,
    TXT_HOSPITAL,             // NAME:			Short name of the structure.
    "HOSP",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64,  // Exit point for produced units.
    99,                       // Build level.
    STRUCTF::BARRACKS.bits(), // PREREQ:		Buildings that must exist first.
    false,                    // Has ability to detect adjacent cloaked objects?
    true,                     // Animation rate is regulated for constant speed?
    true,                     // Requires a bib dirt patch?
    false,                    // Always use the given name for the building?
    false,                    // Is this a wall type structure?
    false,                    // Is it a factory type building?
    true,                     // Can this building be captured?
    false,                    // Does it catch fire?
    false,                    // Simple (one frame) damage imagery?
    false,                    // Is it invisible to radar?
    true,                     // Can the player select this?
    true,                     // Is this a legal target for attack or move?
    false,                    // Is this an insignificant building?
    false,                    // Is it immune to normal combat damage?
    false,                    // Theater specific graphic image?
    false,                    // Does it have a rotating turret?
    false,                    // Fires multiple shots in quick succession?
    true,                     // Can it be repaired?
    true,                     // Can it be manufactured by the player?
    true,                     // Does it contain a crew?
    false,                    // Does building care less if placed on concrete?
    None,                     // The object type produced at this factory.
    DIR_N,                    // Starting idle frame to match construction.
    200,                      // STRNTH:		Full strength of building.
    2,                        // SIGHTRANGE:	Range of sighting.
    500,                      // COST:			Cost to purchase.
    99,                       // SCENARIO:	Starting availability scenario.
    0,
    20, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    100,              // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    20,               // DRAIN:		Power points required.
    BSIZE_22,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List2,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassBioLab: BuildingTypeClass<0, 5, 0> = BuildingTypeClass::new(
    STRUCT_BIO_LAB,
    TXT_BIO_LAB,              // NAME:			Short name of the structure.
    "BIO",                    // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64,  // Exit point for produced units.
    99,                       // Build level.
    STRUCTF::HOSPITAL.bits(), // PREREQ:		Buildings that must exist first.
    false,                    // Has ability to detect adjacent cloaked objects?
    true,                     // Animation rate is regulated for constant speed?
    true,                     // Requires a bib dirt patch?
    true,                     // Always use the given name for the building?
    false,                    // Is this a wall type structure?
    false,                    // Is it a factory type building?
    false,                    // Can this building be captured?
    false,                    // Does it catch fire?
    false,                    // Simple (one frame) damage imagery?
    false,                    // Is it invisible to radar?
    true,                     // Can the player select this?
    true,                     // Is this a legal target for attack or move?
    false,                    // Is this an insignificant building?
    false,                    // Is it immune to normal combat damage?
    false,                    // Theater specific graphic image?
    false,                    // Does it have a rotating turret?
    false,                    // Fires multiple shots in quick succession?
    true,                     // Can it be repaired?
    true,                     // Can it be manufactured by the player?
    true,                     // Does it contain a crew?
    false,                    // Does building care less if placed on concrete?
    None,                     // The object type produced at this factory.
    DIR_N,                    // Starting idle frame to match construction.
    300,                      // STRNTH:		Full strength of building.
    2,                        // SIGHTRANGE:	Range of sighting.
    500,                      // COST:			Cost to purchase.
    99,                       // SCENARIO:	Starting availability scenario.
    0,
    1, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    100,              // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    40,               // DRAIN:		Power points required.
    BSIZE_22,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List2,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassBarracks: BuildingTypeClass<14, 3, 3> = BuildingTypeClass::new(
    STRUCT_BARRACKS,
    TXT_BARRACKS,              // NAME:			Short name of the structure.
    "PYLE",                    // NAME:			Short name of the structure.
    XYP_COORD!(30, 33) as u64, // Exit point for produced units.
    0,                         // Build level.
    STRUCTF::POWER.bits(),     // Building prerequisite.
    false,                     // Has ability to detect adjacent cloaked objects?
    true,                      // Animation rate is regulated for constant speed?
    true,                      // Requires a bib dirt patch?
    false,                     // Always use the given name for the building?
    false,                     // Is this a wall type structure?
    true,                      // Is it a factory type building?
    true,                      // Can this building be captured?
    false,                     // Does it catch fire?
    false,                     // Simple (one frame) damage imagery?
    false,                     // Is it invisible to radar?
    true,                      // Can the player select this?
    true,                      // Is this a legal target for attack or move?
    false,                     // Is this an insignificant building?
    false,                     // Is it immune to normal combat damage?
    false,                     // Theater specific graphic image?
    false,                     // Does it have a rotating turret?
    false,                     // Fires multiple shots in quick succession?
    true,                      // Can it be repaired?
    true,                      // Can it be manufactured by the player?
    true,                      // Does it contain a crew?
    false,                     // Does building care less if placed on concrete?
    Some(RTTI_INFANTRYTYPE),   // The object type produced at this factory.
    DIR_N,                     // Starting idle frame to match construction.
    400,                       // STRNTH:		Full strength of building.
    3,                         // SIGHTRANGE:	Range of sighting.
    300,                       // COST:			Cost to purchase.
    1,                         // SCENARIO:	Starting availability scenario.
    0,
    60, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    20,               // DRAIN:		Power points required.
    BSIZE_22,         // SIZE:			Building size.
    ExitPyle,         // Preferred exit cell list.
    List22_1100,      // OCCUPYLIST:	List of active foundation squares.
    List22_0011,      // OVERLAPLIST:List of overlap cell offset.
);

const ClassHand: BuildingTypeClass<13, 4, 5> = BuildingTypeClass::new(
    STRUCT_HAND,
    TXT_HAND,                  // NAME:			Short name of the structure.
    "HAND",                    // NAME:			Short name of the structure.
    XYP_COORD!(36, 63) as u64, // Exit point for produced units.
    0,                         // Build level.
    STRUCTF::POWER.bits(),     // Building prerequisite.
    false,                     // Has ability to detect adjacent cloaked objects?
    true,                      // Animation rate is regulated for constant speed?
    true,                      // Requires a bib dirt patch?
    false,                     // Always use the given name for the building?
    false,                     // Is this a wall type structure?
    true,                      // Is it a factory type building?
    true,                      // Can this building be captured?
    false,                     // Does it catch fire?
    true,                      // Simple (one frame) damage imagery?
    false,                     // Is it invisible to radar?
    true,                      // Can the player select this?
    true,                      // Is this a legal target for attack or move?
    false,                     // Is this an insignificant building?
    false,                     // Is it immune to normal combat damage?
    false,                     // Theater specific graphic image?
    false,                     // Does it have a rotating turret?
    false,                     // Fires multiple shots in quick succession?
    true,                      // Can it be repaired?
    true,                      // Can it be manufactured by the player?
    true,                      // Does it contain a crew?
    false,                     // Does building care less if placed on concrete?
    Some(RTTI_INFANTRYTYPE),   // The object type produced at this factory.
    DIR_N,                     // Starting idle frame to match construction.
    400,                       // STRNTH:		Full strength of building.
    3,                         // SIGHTRANGE:	Range of sighting.
    300,                       // COST:			Cost to purchase.
    2,                         // SCENARIO:	Starting availability scenario.
    0,
    61, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    20,               // DRAIN:		Power points required.
    BSIZE_23,         // SIZE:			Building size.
    ExitHand,         // Preferred exit cell list.
    ListHand,         // OCCUPYLIST:	List of active foundation squares.
    OListHand,        // OVERLAPLIST:List of overlap cell offset.
);

const ClassTanker: BuildingTypeClass<0, 3, 0> = BuildingTypeClass::new(
    STRUCT_TANKER,
    TXT_TANKER,              // NAME:			Short name of the structure.
    "ARCO",                  // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::POWER.bits(),   // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    false,                   // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    true,                    // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    100,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    1, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_21,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List21,           // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassRepair: BuildingTypeClass<0, 6, 5> = BuildingTypeClass::new(
    STRUCT_REPAIR,
    TXT_FIX_IT,              // NAME:			Short name of the structure.
    "FIX",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    5,                       // Build level.
    STRUCTF::POWER.bits(),   // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    true,                    // Requires a bib dirt patch?
    false,                   // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    true,                    // Can this building be captured?
    false,                   // Does it catch fire?
    false,                   // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    false,                   // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    true,                    // Can it be repaired?
    true,                    // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    400,                     // STRNTH:		Full strength of building.
    3,                       // SIGHTRANGE:	Range of sighting.
    1200,                    // COST:			Cost to purchase.
    8,                       // SCENARIO:	Starting availability scenario.
    0,
    46, // RISK/RWRD:	Risk/reward rating values.
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
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    30,               // DRAIN:		Power points required.
    BSIZE_33,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    ListFix,          // OCCUPYLIST:	List of active foundation squares.
    OListFix,         // OVERLAPLIST:List of overlap cell offset.
);

const ClassRoad: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_ROAD,
    TXT_ROAD,                // NAME:			Short name of the structure.
    "ROAD",                  // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // Building prerequisite.
    false,                   // Has ability to detect adjacent cloaked objects?
    false,                   // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    false,                   // Does it catch fire?
    false,                   // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    false,                   // Can the player select this?
    false,                   // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    false,                   // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    true,                    // Can it be manufactured by the player?
    false,                   // Does it contain a crew?
    true,                    // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    1,                       // STRNTH:		Full strength of building.
    0,                       // SIGHTRANGE:	Range of sighting.
    50,                      // COST:			Cost to purchase.
    99,                      // SCENARIO:	Starting availability scenario.
    0,
    0, // RISK/RWRD:	Risk/reward rating values.
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
    None,     // ARMOR:		Armor type
    0,        // CANENTER:	Units that can enter building.
    0,        // CAPACITY:	Spice storage capacity.
    0,        // POWER:		Power points required.
    0,        // DRAIN:		Power points required.
    BSIZE_11, // SIZE:			Building size.
    [],       // Preferred exit cell list.
    List1,    // OCCUPYLIST:	List of active foundation squares.
    [],       // OVERLAPLIST:List of overlap cell offset.
);

const ClassV01: BuildingTypeClass<0, 3, 3> = BuildingTypeClass::new(
    STRUCT_V01,
    TXT_CIV1,                // NAME:			Short name of the structure.
    "V01",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    true,                    // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    1,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_22,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List0011,         // OCCUPYLIST:	List of active foundation squares.
    List1100,         // OVERLAPLIST:List of overlap cell offset.
);

const ClassV02: BuildingTypeClass<0, 3, 3> = BuildingTypeClass::new(
    STRUCT_V02,
    TXT_CIV2,                // NAME:			Short name of the structure.
    "V02",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    true,                    // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    1,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_22,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List0011,         // OCCUPYLIST:	List of active foundation squares.
    List1100,         // OVERLAPLIST:List of overlap cell offset.
);

const ClassV03: BuildingTypeClass<0, 4, 2> = BuildingTypeClass::new(
    STRUCT_V03,
    TXT_CIV3,                // NAME:			Short name of the structure.
    "V03",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    true,                    // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    1,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_22,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List0111,         // OCCUPYLIST:	List of active foundation squares.
    List1000,         // OVERLAPLIST:List of overlap cell offset.
);

const ClassV04: BuildingTypeClass<0, 3, 3> = BuildingTypeClass::new(
    STRUCT_V04,
    TXT_CIV4,                // NAME:			Short name of the structure.
    "V04",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    true,                    // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    1,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_22,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List0011,         // OCCUPYLIST:	List of active foundation squares.
    List1100,         // OVERLAPLIST:List of overlap cell offset.
);

const ClassV05: BuildingTypeClass<0, 3, 0> = BuildingTypeClass::new(
    STRUCT_V05,
    TXT_CIV5,                // NAME:			Short name of the structure.
    "V05",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    true,                    // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    1,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_21,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List11,           // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV06: BuildingTypeClass<0, 3, 0> = BuildingTypeClass::new(
    STRUCT_V06,
    TXT_CIV6,                // NAME:			Short name of the structure.
    "V06",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    true,                    // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    1,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_21,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List11,           // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV07: BuildingTypeClass<0, 3, 0> = BuildingTypeClass::new(
    STRUCT_V07,
    TXT_CIV7,                // NAME:			Short name of the structure.
    "V07",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    true,                    // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    1,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_21,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List11,           // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV08: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_V08,
    TXT_CIV8,                // NAME:			Short name of the structure.
    "V08",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    true,                    // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    1,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_11,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV09: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_V09,
    TXT_CIV9,                // NAME:			Short name of the structure.
    "V09",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    true,                    // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    1,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_11,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV10: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_V10,
    TXT_CIV10,               // NAME:			Short name of the structure.
    "V10",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    true,                    // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    1,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_11,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV11: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_V11,
    TXT_CIV11,               // NAME:			Short name of the structure.
    "V11",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    true,                    // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    1,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_11,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV12: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_V12,
    TXT_CIV12,               // NAME:			Short name of the structure.
    "V12",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    true,                    // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    1,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_11,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV13: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_V13,
    TXT_CIV13,               // NAME:			Short name of the structure.
    "V13",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    true,                    // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    1,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_11,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV14: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_V14,
    TXT_CIV14,               // NAME:			Short name of the structure.
    "V14",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    true,                    // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    1,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_11,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV15: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_V15,
    TXT_CIV15,               // NAME:			Short name of the structure.
    "V15",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    true,                    // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    1,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_11,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV16: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_V16,
    TXT_CIV16,               // NAME:			Short name of the structure.
    "V16",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    true,                    // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    1,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_11,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV17: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_V17,
    TXT_CIV17,               // NAME:			Short name of the structure.
    "V17",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    true,                    // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    1,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_11,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV18: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_V18,
    TXT_CIV18,               // NAME:			Short name of the structure.
    "V18",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    true,                    // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    1,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_11,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV19: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_PUMP,
    TXT_PUMP,                // NAME:			Short name of the structure.
    "V19",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    false,                   // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    false,                   // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_11,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV20: BuildingTypeClass<0, 3, 3> = BuildingTypeClass::new(
    STRUCT_V20,
    TXT_CIV20,               // NAME:			Short name of the structure.
    "V20",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    false,                   // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_22,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List0011,         // OCCUPYLIST:	List of active foundation squares.
    List1100,         // OVERLAPLIST:List of overlap cell offset.
);

const ClassV21: BuildingTypeClass<0, 4, 2> = BuildingTypeClass::new(
    STRUCT_V21,
    TXT_CIV21,               // NAME:			Short name of the structure.
    "V21",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    false,                   // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_22,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1101,         // OCCUPYLIST:	List of active foundation squares.
    List0010,         // OVERLAPLIST:List of overlap cell offset.
);

const ClassV22: BuildingTypeClass<0, 3, 0> = BuildingTypeClass::new(
    STRUCT_V22,
    TXT_CIV22,               // NAME:			Short name of the structure.
    "V22",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    false,                   // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_21,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List11,           // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV23: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_V23,
    TXT_CIV23,               // NAME:			Short name of the structure.
    "V23",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    false,                   // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_11,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV24: BuildingTypeClass<0, 3, 3> = BuildingTypeClass::new(
    STRUCT_V24,
    TXT_CIV24,               // NAME:			Short name of the structure.
    "V24",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_22,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List0011,         // OCCUPYLIST:	List of active foundation squares.
    List1100,         // OVERLAPLIST:List of overlap cell offset.
);

const ClassV25: BuildingTypeClass<0, 4, 2> = BuildingTypeClass::new(
    STRUCT_V25,
    TXT_CIV25,               // NAME:			Short name of the structure.
    "V25",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_22,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List0111,         // OCCUPYLIST:	List of active foundation squares.
    List1000,         // OVERLAPLIST:List of overlap cell offset.
);

const ClassV26: BuildingTypeClass<0, 3, 0> = BuildingTypeClass::new(
    STRUCT_V26,
    TXT_CIV26,               // NAME:			Short name of the structure.
    "V26",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_21,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List11,           // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV27: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_V27,
    TXT_CIV27,               // NAME:			Short name of the structure.
    "V27",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_11,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV28: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_V28,
    TXT_CIV28,               // NAME:			Short name of the structure.
    "V28",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_11,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV29: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_V29,
    TXT_CIV29,               // NAME:			Short name of the structure.
    "V29",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_11,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV30: BuildingTypeClass<0, 3, 0> = BuildingTypeClass::new(
    STRUCT_V30,
    TXT_CIV30,               // NAME:			Short name of the structure.
    "V30",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_21,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List11,           // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV31: BuildingTypeClass<0, 3, 0> = BuildingTypeClass::new(
    STRUCT_V31,
    TXT_CIV31,               // NAME:			Short name of the structure.
    "V31",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_21,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List11,           // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV32: BuildingTypeClass<0, 3, 0> = BuildingTypeClass::new(
    STRUCT_V32,
    TXT_CIV32,               // NAME:			Short name of the structure.
    "V32",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_21,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List11,           // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV33: BuildingTypeClass<0, 3, 0> = BuildingTypeClass::new(
    STRUCT_V33,
    TXT_CIV33,               // NAME:			Short name of the structure.
    "V33",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_21,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List11,           // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV34: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_V34,
    TXT_CIV34,               // NAME:			Short name of the structure.
    "V34",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_11,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV35: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_V35,
    TXT_CIV35,               // NAME:			Short name of the structure.
    "V35",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_11,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

const ClassV36: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_V36,
    TXT_CIV36,               // NAME:			Short name of the structure.
    "V36",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_11,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List1,            // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);
const ClassV37: BuildingTypeClass<0, 7, 3> = BuildingTypeClass::new(
    STRUCT_V37,
    TXT_CIV37,               // NAME:			Short name of the structure.
    "V37",                   // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    true,                    // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    300,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    0,                       // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_42,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    ListWestwood,     // OCCUPYLIST:	List of active foundation squares.
    OListWestwood,    // OVERLAPLIST:List of overlap cell offset.
);
const ClassMission: BuildingTypeClass<0, 7, 0> = BuildingTypeClass::new(
    STRUCT_MISSION,
    TXT_CIVMISS,             // NAME:			Short name of the structure.
    "MISS",                  // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    true,                    // Has ability to detect adjacent cloaked objects?
    true,                    // Animation rate is regulated for constant speed?
    true,                    // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    false,                   // Is this a wall type structure?
    false,                   // Is it a factory type building?
    true,                    // Can this building be captured?
    true,                    // Does it catch fire?
    true,                    // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    true,                    // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    false,                   // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    false,                   // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    true,                    // Can it be repaired?
    false,                   // Can it be manufactured by the player?
    true,                    // Does it contain a crew?
    false,                   // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    200,                     // STRNTH:		Full strength of building.
    2,                       // SIGHTRANGE:	Range of sighting.
    1000,                    // COST:			Cost to purchase.
    0,                       // SCENARIO:	Starting availability scenario.
    0,
    2, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_WOOD), // ARMOR:		Armor type
    0,                // CANENTER:	Units that can enter building.
    0,                // CAPACITY:	Spice storage capacity.
    0,                // POWER:		Power points required.
    0,                // DRAIN:		Power points required.
    BSIZE_32,         // SIZE:			Building size.
    [],               // Preferred exit cell list.
    List32,           // OCCUPYLIST:	List of active foundation squares.
    [],               // OVERLAPLIST:List of overlap cell offset.
);

// Sandbag wall
const Sandbag: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_SANDBAG_WALL,
    TXT_SANDBAG_WALL,        // NAME:			Short name of the structure.
    "SBAG",                  // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    2,                       // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    false,                   // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    true,                    // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    false,                   // Does it catch fire?
    false,                   // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    false,                   // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    false,                   // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    true,                    // Can it be manufactured by the player?
    false,                   // Does it contain a crew?
    true,                    // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    1,                       // STRNTH:		Full strength of building.
    0,                       // SIGHTRANGE:	Range of sighting.
    50,                      // COST:			Cost to purchase.
    5,                       // SCENARIO:	Starting availability scenario.
    0,
    0, // RISK/RWRD:	Risk/reward rating values.
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
    Some(ARMOR_ALUMINUM), // ARMOR:		Armor type
    0,                    // CANENTER:	Units that can enter building.
    0,                    // CAPACITY:	Spice storage capacity.
    0,                    // POWER:		Power points required.
    0,                    // DRAIN:		Power points required.
    BSIZE_11,             // SIZE:			Building size.
    [],                   // Preferred exit cell list.
    List1,                // OCCUPYLIST:	List of active foundation squares.
    [],                   // OVERLAPLIST:List of overlap cell offset.
);
// Cyclone fence
const Cyclone: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_CYCLONE_WALL,
    TXT_CYCLONE_WALL,        // NAME:			Short name of the structure.
    "CYCL",                  // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    5,                       // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    false,                   // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    true,                    // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    false,                   // Does it catch fire?
    false,                   // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    false,                   // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    false,                   // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    true,                    // Can it be manufactured by the player?
    false,                   // Does it contain a crew?
    true,                    // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    1,                       // STRNTH:		Full strength of building.
    0,                       // SIGHTRANGE:	Range of sighting.
    75,                      // COST:			Cost to purchase.
    9,                       // SCENARIO:	Starting availability scenario.
    0,
    0, // RISK/RWRD:	Risk/reward rating values.
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
    Some(ARMOR_ALUMINUM), // ARMOR:		Armor type
    0,                    // CANENTER:	Units that can enter building.
    0,                    // CAPACITY:	Spice storage capacity.
    0,                    // POWER:		Power points required.
    0,                    // DRAIN:		Power points required.
    BSIZE_11,             // SIZE:			Building size.
    [],                   // Preferred exit cell list.
    List1,                // OCCUPYLIST:	List of active foundation squares.
    [],                   // OVERLAPLIST:List of overlap cell offset.
);
// Brick wall
const Brick: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_BRICK_WALL,
    TXT_BRICK_WALL,          // NAME:			Short name of the structure.
    "BRIK",                  // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    7,                       // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    false,                   // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    true,                    // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    false,                   // Does it catch fire?
    false,                   // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    false,                   // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    false,                   // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    true,                    // Can it be manufactured by the player?
    false,                   // Does it contain a crew?
    true,                    // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    1,                       // STRNTH:		Full strength of building.
    0,                       // SIGHTRANGE:	Range of sighting.
    100,                     // COST:			Cost to purchase.
    13,                      // SCENARIO:	Starting availability scenario.
    0,
    0, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_ALUMINUM), // ARMOR:		Armor type
    0,                    // CANENTER:	Units that can enter building.
    0,                    // CAPACITY:	Spice storage capacity.
    0,                    // POWER:		Power points required.
    0,                    // DRAIN:		Power points required.
    BSIZE_11,             // SIZE:			Building size.
    [],                   // Preferred exit cell list.
    List1,                // OCCUPYLIST:	List of active foundation squares.
    [],                   // OVERLAPLIST:List of overlap cell offset.
);
// Barbwire wall
const Barbwire: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_BARBWIRE_WALL,
    TXT_BARBWIRE_WALL,       // NAME:			Short name of the structure.
    "BARB",                  // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    98,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    false,                   // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    true,                    // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    false,                   // Does it catch fire?
    false,                   // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    false,                   // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    false,                   // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    true,                    // Can it be manufactured by the player?
    false,                   // Does it contain a crew?
    true,                    // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    1,                       // STRNTH:		Full strength of building.
    0,                       // SIGHTRANGE:	Range of sighting.
    25,                      // COST:			Cost to purchase.
    98,                      // SCENARIO:	Starting availability scenario.
    0,
    0, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_ALUMINUM), // ARMOR:		Armor type
    0,                    // CANENTER:	Units that can enter building.
    0,                    // CAPACITY:	Spice storage capacity.
    0,                    // POWER:		Power points required.
    0,                    // DRAIN:		Power points required.
    BSIZE_11,             // SIZE:			Building size.
    [],                   // Preferred exit cell list.
    List1,                // OCCUPYLIST:	List of active foundation squares.
    [],                   // OVERLAPLIST:List of overlap cell offset.
);
// Wood wall
const Wood: BuildingTypeClass<0, 2, 0> = BuildingTypeClass::new(
    STRUCT_WOOD_WALL,
    TXT_WOOD_WALL,           // NAME:			Short name of the structure.
    "WOOD",                  // NAME:			Short name of the structure.
    XYP_COORD!(0, 0) as u64, // Exit point for produced units.
    99,                      // Build level.
    STRUCTF::NONE.bits(),    // PREREQ:		Buildings that must exist first.
    false,                   // Has ability to detect adjacent cloaked objects?
    false,                   // Animation rate is regulated for constant speed?
    false,                   // Requires a bib dirt patch?
    true,                    // Always use the given name for the building?
    true,                    // Is this a wall type structure?
    false,                   // Is it a factory type building?
    false,                   // Can this building be captured?
    false,                   // Does it catch fire?
    false,                   // Simple (one frame) damage imagery?
    false,                   // Is it invisible to radar?
    false,                   // Can the player select this?
    true,                    // Is this a legal target for attack or move?
    true,                    // Is this an insignificant building?
    false,                   // Is it immune to normal combat damage?
    false,                   // Theater specific graphic image?
    false,                   // Does it have a rotating turret?
    false,                   // Fires multiple shots in quick succession?
    false,                   // Can it be repaired?
    true,                    // Can it be manufactured by the player?
    false,                   // Does it contain a crew?
    true,                    // Does building care less if placed on concrete?
    None,                    // The object type produced at this factory.
    DIR_N,                   // Starting idle frame to match construction.
    1,                       // STRNTH:		Full strength of building.
    0,                       // SIGHTRANGE:	Range of sighting.
    25,                      // COST:			Cost to purchase.
    98,                      // SCENARIO:	Starting availability scenario.
    0,
    0, // RISK/RWRD:	Risk/reward rating values.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::NEUTRAL.bits()
        | HOUSEF::GOOD.bits(), // OWNABLE:		Ownable by house (bit field).
    None,
    None,
    Some(ARMOR_ALUMINUM), // ARMOR:		Armor type
    0,                    // CANENTER:	Units that can enter building.
    0,                    // CAPACITY:	Spice storage capacity.
    0,                    // POWER:		Power points required.
    0,                    // DRAIN:		Power points required.
    BSIZE_11,             // SIZE:			Building size.
    [],                   // Preferred exit cell list.
    List1,                // OCCUPYLIST:	List of active foundation squares.
    [],                   // OVERLAPLIST:List of overlap cell offset.
);
