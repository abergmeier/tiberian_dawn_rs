#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables
)]

use strum_macros::{EnumCount, EnumIter};

use crate::{
    abstract_::MatchesInternalControlName, armor::ArmorType, building::STRUCTF, ini::IniName,
    mission::MissionType, speed::MPHType, techno::TechnoTypeClass, text::IDs, weapon::WeaponType,
};

///	The variuos aircraft types are enumerated here. These include helicopters
///	as well as traditional aircraft.
#[derive(Clone, Copy, EnumCount, EnumIter)]
#[repr(u8)]
pub enum AircraftType {
    AIRCRAFT_TRANSPORT,  // Transport helicopter.
    AIRCRAFT_A10,        // Ground attack plane.
    AIRCRAFT_HELICOPTER, // Apache gunship.
    AIRCRAFT_CARGO,      // Cargo plane.
    AIRCRAFT_ORCA,       // Nod attack helicopter.
}

enum UnitTypeClassRepairEnums {
    REPAIR_PERCENT = 102, // 40% fixed point number.
    REPAIR_STEP = 2,      // Number of damage points recovered per "step".
}

/// The various aircraft types are controlled by this list.
pub struct AircraftTypeClass {
    techno_type_class: TechnoTypeClass,

    /// Fixed wing aircraft (ones that cannot hover) have this flag set to true.
    ///	Such aircraft will not vary speed while it is flying.
    IsFixedWing: bool,

    ///	Can this aircraft land?  If it can land it is presumed to be controllable by the player.
    IsLandable: bool,

    ///	Does this aircraft have a rotor blade (helicopter) type propulsion?
    IsRotorEquipped: bool, // Is a rotor attached?

    ///	Is there a custom rotor animation stage set for each facing of the aircraft?
    IsRotorCustom: bool, // Custom rotor sets for each facing?

    Type: AircraftType,
    ROT: u8,
    Mission: MissionType,
}

impl IniName for AircraftTypeClass {
    fn INI_Name() -> &'static str {
        "AIRCRAFT"
    }
}

impl AircraftTypeClass {
    pub const fn new(
        airtype: AircraftType,
        name: IDs,
        ininame: &str,
        level: u8,
        pre: <STRUCTF as bitflags::Flags>::Bits,
        is_leader: bool,
        is_twoshooter: bool,
        is_transporter: bool,
        is_fixedwing: bool,
        is_rotorequipped: bool,
        is_rotorcustom: bool,
        is_landable: bool,
        is_crushable: bool,
        is_stealthy: bool,
        is_selectable: bool,
        is_legal_target: bool,
        is_insignificant: bool,
        is_immune: bool,
        is_theater: bool,
        is_repairable: bool,
        is_buildable: bool,
        is_crew: bool,
        ammo: i32,
        strength: u16,
        sightrange: u16,
        cost: u32,
        scenario: u8,
        risk: u32,
        reward: u32,
        ownable: u16,
        primary: Option<WeaponType>,
        secondary: Option<WeaponType>,
        armor: ArmorType,
        maxspeed: MPHType,
        rot: u8,
        deforder: MissionType,
    ) -> Self {
        Self {
            techno_type_class: TechnoTypeClass::new(
                name,
                ininame,
                level,
                pre,
                is_leader,
                false,
                false,
                is_transporter,
                false,
                is_crushable,
                is_stealthy,
                is_selectable,
                is_legal_target,
                is_insignificant,
                is_immune,
                is_theater,
                is_twoshooter,
                false,
                is_repairable,
                is_buildable,
                is_crew,
                ammo,
                strength,
                maxspeed,
                sightrange,
                cost,
                scenario,
                risk,
                reward,
                ownable,
                primary,
                secondary,
                Some(armor),
            ),
            IsRotorEquipped: is_rotorequipped,
            IsRotorCustom: is_rotorcustom,
            IsLandable: is_landable,
            IsFixedWing: is_fixedwing,
            Type: airtype,
            ROT: rot,
            Mission: deforder,
        }
    }
}
impl MatchesInternalControlName for AircraftTypeClass {
    fn matches_internal_control_name(&self, other_internal_control_name: &str) -> bool {
        self.techno_type_class
            .matches_internal_control_name(other_internal_control_name)
    }
}
