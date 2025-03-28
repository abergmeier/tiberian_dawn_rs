#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables
)]

#[derive(Clone, Copy, Debug)]
pub enum RTTIType {
    RTTI_INFANTRY,
    RTTI_INFANTRYTYPE,
    RTTI_UNIT,
    RTTI_UNITTYPE,
    RTTI_AIRCRAFT,
    RTTI_AIRCRAFTTYPE,
    RTTI_BUILDING,
    RTTI_BUILDINGTYPE,

    RTTI_TERRAIN,
    RTTI_ABSTRACTTYPE,
    RTTI_ANIM,
    RTTI_ANIMTYPE,
    RTTI_BULLET,
    RTTI_BULLETTYPE,
    RTTI_OVERLAY,
    RTTI_OVERLAYTYPE,
    RTTI_SMUDGE,
    RTTI_SMUDGETYPE,
    RTTI_TEAM,
    RTTI_TEMPLATE,
    RTTI_TEMPLATETYPE,
    RTTI_TERRAINTYPE,
    RTTI_OBJECT,
    RTTI_SPECIAL,
}
