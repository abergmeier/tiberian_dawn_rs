#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_variables)]

use crate::{
    armor::ArmorType,
    building::STRUCTF,
    object::ObjectTypeClass,
    speed::MPHType,
    text::IDs,
    weapon::{WeaponType, Weapons},
};

/// This class is the common data for all objects that can be owned, produced,
/// or delivered as reinforcements. These are objects that typically contain
/// crews and weapons -- the fighting objects of the game.
pub struct TechnoTypeClass {
    object_type_class: ObjectTypeClass,
    /// If this object can serve as a good leader for a group selected
    /// series of objects, then this flag will be true. Unarmed or
    /// ability challenged units do not make good leaders.
    IsLeader: bool,

    /// Does this object have the ability to detect the presence of a nearby
    /// cloaked object?
    IsScanner: bool,

    /// If this object is always given its proper name rather than a generic
    /// name, then this flag will be true. Typically, civilians and Dr. Mobius
    /// fall under this catagory.
    IsNominal: bool,

    /// If the artwork for this object (only for generics) is theater specific, then
    /// this flag will be true. Civilian buildings are a good example of this.
    IsTheater: bool,

    /// Does this object type contain a rotating turret?  Gun emplacements, SAM launchers,
    /// and many vehicles contain a turret. If a turret is present, special rendering and
    /// combat logic must be performed.
    IsTurretEquipped: bool,

    /// Certain units and buildings fire two shots in quick succession. If this is
    /// the case, then this flag is true.
    IsTwoShooter: bool,

    /// Certain objects can be repaired. For buildings, they repair "in place". For units,
    /// they must travel to a repair center to be repaired. If this flag is true, then
    /// allow the player or computer AI to repair the object.
    IsRepairable: bool,

    /// Is this object possible to be constructed?  Certain buildings and units cannot
    /// be constructed using normal means. They are either initially placed in the scenario
    /// or can only arrive by pre arranged reinforcement scheduling. Civilian buildings and
    /// vehicles are typical examples of this type of object. They would set this flag to
    ///false.
    IsBuildable: bool,

    /// Does this object contain a crew?  If it does, then when the object is destroyed, there
    /// is a distinct possibility that infantry will "pop out". Only units with crews can
    /// become "heros".
    IsCrew: bool,

    /// Is this object typically used to transport reinforcements or other cargo?
    /// Transport aircraft, helicopters, and hovercraft are typicall examples of
    /// this.
    IsTransporter: bool,

    /// Most objects have the ability to reveal the terrain around themselves.
    /// This sight range (expressed in cell distance) is specified here. If
    /// this value is 0, then this unit never reveals terrain. Bullets are
    /// typically of this nature.
    SightRange: u16,

    /// These values control the cost to produce, the time to produce, and
    /// the scenario when production can first start.
    Cost: u32,
    Scenario: u8,

    /// Special build prerequisite control values. These are primarily used for
    /// multi-player or special events.
    Level: u8,
    Pre: <STRUCTF as bitflags::Flags>::Bits,

    /// The risk and reward values are used to determine targets and paths
    /// toward targets. When heading toward a target, a path of least
    /// risk will be followed. When picking a target, the object of
    /// greatest reward will be selected. The values assigned are
    /// arbitrary.
    Risk: u32,
    Reward: u32,

    /// This value indicates the maximum speed that this object can achieve.
    MaxSpeed: MPHType,

    /// This is the maximum number of ammo shots this object can hold. If
    /// this number is -1, then this indicates unlimited ammo.
    MaxAmmo: i32,

    /// This is a bit field representing the houses that are allowed to
    /// own (by normal means) this particular object type. This value is
    /// typically used in production contexts. It is possible for a side
    /// to take possession of an object type otherwise not normally allowed.
    /// This event usually occurs as a result of capture.
    Ownable: u16,

    /// This is the small icon image that is used to display the object in
    /// the sidebar for construction selection purposes.
    CameoData: [u8; 0],

    /// These are the weapons that this techno object is armed with.
    Primary: Option<WeaponType>,
    Secondary: Option<WeaponType>,
}

impl TechnoTypeClass {
    pub const fn new(
        name: IDs,
        ininame: &str,
        level: u8,
        pre: <STRUCTF as bitflags::Flags>::Bits,
        is_leader: bool,
        is_scanner: bool,
        is_nominal: bool,
        is_transporter: bool,
        is_flammable: bool,
        is_crushable: bool,
        is_stealthy: bool,
        is_selectable: bool,
        is_legal_target: bool,
        is_insignificant: bool,
        is_immune: bool,
        is_theater: bool,
        is_twoshooter: bool,
        is_turret_equipped: bool,
        is_repairable: bool,
        is_buildable: bool,
        is_crew: bool,
        ammo: i32,
        strength: u16,
        maxspeed: MPHType,
        sightrange: u16,
        cost: u32,
        scenario: u8,
        risk: u32,
        reward: u32,
        ownable: u16,
        primary: Option<WeaponType>,
        secondary: Option<WeaponType>,
        armor: Option<ArmorType>,
    ) -> Self {
        Self {
            object_type_class: ObjectTypeClass::new(
                true,
                is_flammable,
                is_crushable,
                is_stealthy,
                is_selectable,
                is_legal_target,
                is_insignificant,
                is_immune,
                Some(name),
                ininame,
                armor,
                strength,
            ),
            Level: level,
            Pre: pre,
            MaxAmmo: ammo,
            MaxSpeed: maxspeed,
            CameoData: [],
            Primary: primary,
            Secondary: secondary,
            Cost: cost,
            IsLeader: is_leader,
            IsScanner: is_scanner,
            IsTransporter: is_transporter,
            IsTwoShooter: is_twoshooter,
            IsBuildable: is_buildable,
            IsCrew: is_crew,
            IsTheater: is_theater,
            IsRepairable: is_repairable,
            IsTurretEquipped: is_turret_equipped,
            IsNominal: is_nominal,
            Ownable: ownable,
            Reward: reward,
            Scenario: scenario,
            SightRange: sightrange,
            /*
             ** Units risk value is based on the type of weapon he has and the
             ** rate of fire it shoots at.
             */
            Risk: {
                match primary {
                    None => 0,
                    Some(primary) => {
                        (Weapons[primary as usize].Attack as u32
                            * (Weapons[primary as usize].Range as u32 >> 4))
                            / Weapons[primary as usize].ROF as u32
                    }
                }
            },
        }
    }
}
