#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables
)]

use crate::{animation::AnimType, object::ObjectTypeClass, speed::MPHType};

///	This enumerates the various bullet types. These types specify bullet's
///	visual and explosive characteristics.
pub enum BulletType {
    //BULLET_NONE=-1,
    BULLET_SNIPER,      // Sniper bullet.
    BULLET_BULLET,      // Small arms
    BULLET_APDS,        // Armor piercing projectile.
    BULLET_HE,          // High explosive shell.
    BULLET_SSM,         // Surface to surface small missile type.
    BULLET_SSM2,        // MLRS missile.
    BULLET_SAM,         // Fast homing anti-aircraft missile.
    BULLET_TOW,         // TOW anti-vehicle short range missile.
    BULLET_FLAME,       // Flame thrower flame.
    BULLET_CHEMSPRAY,   // Chemical weapon spray.
    BULLET_NAPALM,      // Napalm bomblet.
    BULLET_GRENADE,     // Hand tossed grenade.
    BULLET_LASER,       // Laser beam from obelisk
    BULLET_NUKE_UP,     // Nuclear Missile on its way down
    BULLET_NUKE_DOWN,   // Nuclear Missile on its way up
    BULLET_HONEST_JOHN, // SSM with napalm warhead.
    BULLET_SPREADFIRE,  // Chain gun bullets.
    BULLET_HEADBUTT,    // Stegosaurus, Triceratops head butt
    BULLET_TREXBITE,    // Tyrannosaurus Rex's bite - especially bad for infantry

                        //BULLET_COUNT,
                        //BULLET_FIRST=0
}

///	Damage, as inflicted by projectiles, has different characteristics.
//	These are the different "warhead" types that can be assigned to the
//	various projectiles in the game.
pub enum WarheadType {
    //WARHEAD_NONE=-1,
    WARHEAD_SA,           // Small arms -- good against infantry.
    WARHEAD_HE,           //	High explosive -- good against buildings & infantry.
    WARHEAD_AP,           // Amor piercing -- good against armor.
    WARHEAD_FIRE,         // Incendiary -- Good against flammables.
    WARHEAD_LASER,        // Light Amplification of Stimulated Emission of Radiation.
    WARHEAD_PB,           // Particle beam (neutron beam).
    WARHEAD_FIST,         // punching in hand-to-hand combat
    WARHEAD_FOOT,         // kicking in hand-to-hand combat
    WARHEAD_HOLLOW_POINT, // Sniper bullet type.
    WARHEAD_SPORE,        // Spores from blossom tree - affect infantry only
    WARHEAD_HEADBUTT,     // Other dinosaurs butt into people
    WARHEAD_FEEDME,       // T-Rex eats people, hurts vehicles/buildings

                          //WARHEAD_COUNT
}

pub struct BulletTypeClass {
    object_type_class: ObjectTypeClass,
    ///	Does this bullet type fly over walls?
    pub IsHigh: bool,

    ///	If this projecile is one that balistically arcs from ground level, up into the air and
    ///	then back to the ground, where it explodes. Typical uses of this are for grenades and
    ///	artillery shells.
    pub IsArcing: bool,

    ///	If the projectile changes course as it flies in order to home in on the
    ///	projectile's target, then this flag is true. Missiles are typically ones
    ///	that do this.
    pub IsHoming: bool,

    ///	Certain projectiles do not travel horizontally, but rather, vertically -- they drop
    ///	from a height. Bombs fall into this category and will have this value set to
    ///	true. Dropping projectiles do not calculate collision with terrain (such as walls).
    pub IsDropping: bool,

    ///	Is this projectile invisible?  Some bullets and weapon effects are not directly
    ///	rendered. Small caliber bullets and flame thrower flames are treated like
    ///	normal projectiles for damage purposes, but are displayed using custom
    ///	rules.
    pub IsInvisible: bool,

    ///	Does this bullet explode when near the target?  Some bullets only explode if
    ///	it actually hits the target. Some explode even if nearby.
    pub IsProximityArmed: bool,

    ///	Does this projectile spew puffs of smoke out its tail while it
    ///	travels? Missiles are prime examples of this projectile type.
    pub IsFlameEquipped: bool,

    ///	Should fuel consumption be tracked for this projectile?  Rockets are the primary
    ///	projectile with this characteristic, but even for bullets it should be checked so that
    ///	bullets don't travel too far.
    pub IsFueled: bool,

    ///	Is this projectile without different facing visuals?  Most plain bullets do not change
    ///	visual imagery if their facing changes. Rockets, on the other hand, are equipped with
    ///	the full 32 facing imagery.
    pub IsFaceless: bool,

    ///	If this is a typically inaccurate projectile, then this flag will be true. Artillery
    ///	is a prime example of this type.
    pub IsInaccurate: bool,

    ///	If the bullet contains translucent pixels, then this flag will be true. These
    ///	translucent pixels really are "shadow" pixels in the same style as the shadow
    ///	cast by regular ground units.
    pub IsTranslucent: bool,

    ///	If this bullet can be fired on aircraft, then this flag will be true.
    pub IsAntiAircraft: bool,

    ///	This element is a unique identification number for the bullet
    ///	type.
    Type: BulletType,

    ///	Maximum speed for this bullet type.
    MaxSpeed: MPHType,

    ///	This projectile has a certain style of warhead. This value specifies
    ///	what that warhead type is.
    Warhead: WarheadType,

    ///	This is the "explosion" effect to use when this projectile impacts
    Explosion: Option<AnimType>,

    ///	This is the rotation speed of the bullet. It only has practical value
    ///	for those projectiles that performing homing action during flight -- such
    ///	as with rockets.
    ROT: u8,

    ///	Some projectiles have a built in arming distance that must elapse before the
    ///	projectile may explode. If this value is non-zero, then this override is
    ///	applied.
    Arming: i32,

    ///	Some projectiles have a built in override for distance travelled before it
    ///	automatically explodes. This value, if non-zero, specifies this distance.
    Range: i32,
}

impl BulletTypeClass {
    pub const fn new(
        type_: BulletType,
        ininame: &str,
        is_high: bool,
        is_homing: bool,
        is_arcing: bool,
        is_dropping: bool,
        is_invisible: bool,
        is_proximity_armed: bool,
        is_flame_equipped: bool,
        is_fueled: bool,
        is_faceless: bool,
        is_inaccurate: bool,
        is_translucent: bool,
        is_antiaircraft: bool,
        arming: i32,
        range: i32,
        maxspeed: MPHType,
        rot: u8,
        warhead: WarheadType,
        explosion: Option<AnimType>,
    ) -> Self {
        Self {
            object_type_class: ObjectTypeClass::new(
                true, false, false, true, false, false, true, true, None, ininame, None, 0,
            ),
            Explosion: explosion,
            IsHigh: is_high,
            IsAntiAircraft: is_antiaircraft,
            IsTranslucent: is_translucent,
            IsArcing: is_arcing,
            IsHoming: is_homing,
            IsDropping: is_dropping,
            IsInvisible: is_invisible,
            IsProximityArmed: is_proximity_armed,
            IsFlameEquipped: is_flame_equipped,
            IsFueled: is_fueled,
            IsFaceless: is_faceless,
            IsInaccurate: is_inaccurate,
            Type: type_,
            Warhead: warhead,
            MaxSpeed: maxspeed,
            ROT: rot,
            Arming: arming,
            Range: range,
        }
    }
}
