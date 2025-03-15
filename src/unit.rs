#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables
)]
use crate::{
    animation::AnimType,
    armor::ArmorType,
    building,
    house::HOUSEF,
    mission::MissionType,
    speed::{MPHType, SpeedType},
    techno::TechnoTypeClass,
    text::IDs,
    weapon::WeaponType,
};

/// The game units are enumerated here. These include not only traditional
/// vehicles, but also hovercraft and gunboats.
#[repr(u8)]
pub enum UnitType {
    //UNIT_NONE=-1,
    UNIT_HTANK,     // Heavy tank (Mammoth).
    UNIT_MTANK,     // Medium tank (M1).
    UNIT_LTANK,     // Light tank ('Bradly').
    UNIT_STANK,     // Stealth tank (Romulan).
    UNIT_FTANK,     // Flame thrower tank.
    UNIT_VICE,      // Visceroid
    UNIT_APC,       // APC.
    UNIT_MLRS,      // MLRS rocket launcher.
    UNIT_JEEP,      // 4x4 jeep replacement.
    UNIT_BUGGY,     // Rat patrol dune buggy type.
    UNIT_HARVESTER, // Resource gathering vehicle.
    UNIT_ARTY,      // Artillery unit.
    UNIT_MSAM,      // Anti-Aircraft vehicle.
    UNIT_HOVER,     // Hovercraft.
    UNIT_MHQ,       // Mobile Head Quarters.
    UNIT_GUNBOAT,   // Gunboat
    UNIT_MCV,       // Mobile construction vehicle.
    UNIT_BIKE,      // Nod recon motor-bike.
    UNIT_TRIC,      // Triceratops
    UNIT_TREX,      //	Tyranosaurus Rex
    UNIT_RAPT,      //	Velociraptor
    UNIT_STEG,      //	Stegasaurus

                    //UNIT_COUNT,
                    //UNIT_FIRST=0
}

#[repr(u16)]
enum UnitTypeClassRepairEnums {
    TIBERIUM_STEP = 25, // Credits per step of Tiberium.
    STEP_COUNT = 28,    // Number of steps a harvester can carry.
    FULL_LOAD_CREDITS = Self::TIBERIUM_STEP as u16 * Self::STEP_COUNT as u16,
    REPAIR_PERCENT = 102, // 40% fixed point number.
    REPAIR_STEP = 4,      // Number of damage points recovered per "step".
}

/// The various unit types need specific data that is unique to units as
/// opposed to buildings. This derived class elaborates these additional
/// data types.
pub struct UnitTypeClass {
    techno_type_class: TechnoTypeClass,
    /// If this unit can appear out of a crate, then this flag will be true.
    IsCrateGoodie: bool,

    /// Does this unit have only 8 facings? Special test units have limited
    /// facings.
    IsPieceOfEight: bool,

    /// Can this unit squash infantry?  If it can then if the player selects
    /// an (enemy) infantry unit as the movement target, it will ride over and
    /// squish the infantry unit.
    IsCrusher: bool,

    /// Does this unit go into harvesting mode when it stops on a tiberium
    /// field?  Typically, only one unit does this and that is the harvester.
    IsToHarvest: bool,

    /// Does this unit's shape data consist of "chunky" facings?  This kind of unit
    /// art has the unit in only 4 facings (N, W, S, and E) and in each of those
    /// directions, the unit's turrets rotates 32 facings (counter clockwise from north).
    /// This will result in 32 x 4 = 128 unit shapes in the shape data file.
    IsChunkyShape: bool,

    /// Some units are equipped with a rotating radar dish. These units have special
    /// animation processing. The rotating radar dish is similar to a turret, but
    /// always rotates and does not affect combat.
    IsRadarEquipped: bool,

    /// If this unit has a firing animation, this flag is true. Infantry and some special
    /// vehicles are the ones with firing animations.
    IsFireAnim: bool,

    /// Many vehicles have a turret with restricted motion. These vehicles must move the
    // turret into a locked down position while travelling. Rocket launchers and artillery
    // are good examples of this kind of unit.
    IsLockTurret: bool,

    /// Does this unit lay tracks when it travels?  Most tracked vehicles and some wheeled
    /// vehicles have this ability.
    IsTracked: bool,

    /// Is this unit of the humongous size?  Harvesters and mobile construction vehicles are
    /// of this size. If the vehicle is greater than 24 x 24 but less than 48 x 48, it is
    /// considered "Gigundo".
    IsGigundo: bool,

    /// Is the unit capable of cloaking?  Only Stealth Tank can do so now.
    IsCloakable: bool,

    /// Does this unit have a constant animation (like Visceroid?)
    IsAnimating: bool,

    /// This value represents the unit class. It can serve as a unique
    /// identification number for this unit class.
    Type: UnitType,

    /// This indicates the speed (locomotion) type for this unit. Through this
    /// value the movement capabilities are deduced.
    Speed: SpeedType,

    /// This is the rotational speed of the unit. This value represents the
    /// turret rotation speed.
    ROT: u8,

    /// This is the distance along the centerline heading in the direction the body
    /// is facing used to reach the center point of the turret. This distance is
    /// in leptons.
    TurretOffset: i8,

    /// This value is used to provide the unit with a default mission order when
    /// first created. Usually, this is a resting or idle type of order.
    Mission: MissionType,

    /// This is the default explosion to use when this vehicle is destroyed.
    Explosion: AnimType,

    /// The width or height of the largest dimension for this unit.
    MaxSize: i32,
    // This is a pointer to the wake shape (as needed by the gunboat).
    //static void const * WakeShapes;
}

impl UnitTypeClass {
    pub const fn new(
        type_: UnitType,
        name: IDs,
        ininame: &str,
        exp: AnimType,
        level: u8,
        pre: <building::Flags as bitflags::Flags>::Bits,
        is_goodie: bool,
        is_leader: bool,
        is_eight: bool,
        is_nominal: bool,
        is_transporter: bool,
        is_crushable: bool,
        is_crusher: bool,
        is_harvest: bool,
        is_stealthy: bool,
        is_selectable: bool,
        is_legal_target: bool,
        is_insignificant: bool,
        is_immune: bool,
        is_turret_equipped: bool,
        is_twoshooter: bool,
        is_repairable: bool,
        is_buildable: bool,
        is_crew: bool,
        is_radar_equipped: bool,
        is_fire_anim: bool,
        is_lock_turret: bool,
        is_tracked: bool,
        is_gigundo: bool,
        is_chunky: bool,
        is_cloakable: bool,
        is_animating: bool,
        ammo: i32,
        strength: u16,
        sightrange: u16,
        cost: u32,
        scenario: u8,
        risk: u32,
        reward: u32,
        ownable: <HOUSEF as bitflags::Flags>::Bits,
        primary: Option<WeaponType>,
        secondary: Option<WeaponType>,
        armor: ArmorType,
        speed: SpeedType,
        maxSpeed: MPHType,
        rot: u8,
        toffset: i8,
        order: MissionType,
    ) -> Self {
        Self {
            techno_type_class: TechnoTypeClass::new(
                name,
                ininame,
                level,
                pre,
                is_leader,
                false,
                is_nominal,
                is_transporter,
                false,
                is_crushable,
                is_stealthy,
                is_selectable,
                is_legal_target,
                is_insignificant,
                is_immune,
                false,
                is_twoshooter,
                is_turret_equipped,
                is_repairable,
                is_buildable,
                is_crew,
                ammo,
                strength,
                maxSpeed,
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
            Explosion: exp,
            IsCrateGoodie: is_goodie,
            IsPieceOfEight: is_eight,
            IsCloakable: is_cloakable,
            IsChunkyShape: is_chunky,
            IsCrusher: is_crusher,
            IsFireAnim: is_fire_anim,
            IsGigundo: is_gigundo,
            IsLockTurret: is_lock_turret,
            IsRadarEquipped: is_radar_equipped,
            IsToHarvest: is_harvest,
            IsTracked: is_tracked,
            IsAnimating: is_animating,
            Mission: order,
            ROT: rot,
            Speed: speed,
            TurretOffset: toffset,
            Type: type_,
            MaxSize: 0,
        }
    }
}
