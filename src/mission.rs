#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables
)]

use strum_macros::{EnumCount, EnumIter};

///	These missions enumerate the various state machines that can apply to
///	a game object. Only one of these state machines is active at any one
///	time.
#[derive(Clone, Copy, Debug, EnumCount, EnumIter, PartialEq)]
#[repr(u8)]
pub enum MissionType {
    //MISSION_NONE=-1,
    MISSION_SLEEP,          // Do nothing whatsoever.
    MISSION_ATTACK,         // Attack nearest enemy.
    MISSION_MOVE,           // Guard location or unit.
    MISSION_RETREAT,        // Return home for R & R.
    MISSION_GUARD,          // Stay still.
    MISSION_STICKY,         // Stay still -- never recruit.
    MISSION_ENTER,          // Move into object cooperatively.
    MISSION_CAPTURE,        //	Move into in order to capture.
    MISSION_HARVEST,        // Hunt for and collect nearby Tiberium.
    MISSION_GUARD_AREA,     // Active guard of area.
    MISSION_RETURN,         // Head back to refinery.
    MISSION_STOP,           // Sit still.
    MISSION_AMBUSH,         // Wait until discovered.
    MISSION_HUNT,           // Active search and destroy.
    MISSION_TIMED_HUNT,     // Wait a while, then go into HUNT (multiplayer AI)
    MISSION_UNLOAD,         // Search for and deliver cargo.
    MISSION_SABOTAGE,       //	Move into in order to destroy.
    MISSION_CONSTRUCTION,   // Building buildup operation.
    MISSION_DECONSTRUCTION, // Building builddown operation.
    MISSION_REPAIR,         // Repair process mission.
    MISSION_RESCUE,
    MISSION_MISSILE,
    //MISSION_COUNT,
    //MISSION_FIRST=0
}
