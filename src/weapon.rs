#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables
)]

use crate::weapon::AnimType::*;
use crate::weapon::BulletType::*;
use crate::weapon::VocType::*;
use std::ops::Index;
use strum::EnumCount;
use strum_macros::EnumCount;

use crate::{animation::AnimType, audio::VocType, bullet::BulletType};

/// This enumerates the various weapon types. The weapon is characterized
/// by the projectile it launches, the damage it does, and the rate of
/// fire.
#[derive(Clone, Copy, EnumCount)]
#[repr(u8)]
pub enum WeaponType {
    //None=-1,
    WEAPON_RIFLE,
    WEAPON_CHAIN_GUN,
    WEAPON_PISTOL,
    WEAPON_M16,
    WEAPON_DRAGON,
    WEAPON_FLAMETHROWER,
    WEAPON_FLAME_TONGUE,
    WEAPON_CHEMSPRAY,
    WEAPON_GRENADE,
    WEAPON_75MM,
    WEAPON_105MM,
    WEAPON_120MM,
    WEAPON_TURRET_GUN,
    WEAPON_MAMMOTH_TUSK,
    WEAPON_MLRS,
    WEAPON_155MM,
    WEAPON_M60MG,
    WEAPON_TOMAHAWK,
    WEAPON_TOW_TWO,
    WEAPON_NAPALM,
    WEAPON_OBELISK_LASER,
    WEAPON_NIKE,
    WEAPON_HONEST_JOHN,
    WEAPON_STEG,
    WEAPON_TREX,
    //WEAPON_COUNT
}

/// This is the constant data associated with a weapon. Some objects
/// can have multiple weapons and this class is used to isolate and
/// specify this data in a convenient and selfcontained way.
pub struct WeaponTypeClass {
    /// This is the unit class of the projectile fired. A subset of the unit types
    /// represent projectiles. It is one of these classes that is specified here.
    /// If this object does not fire anything, then this value will be UNIT_NONE.
    Fires: BulletType,

    /// This is the damage (explosive load) to be assigned to the projectile that
    /// this object fires.
    pub Attack: u8,

    /// Objects that fire (which can be buildings as well) will fire at a
    /// frequency controlled by this value. This value serves as a count
    /// down timer between shots. The smaller the value, the faster the
    /// rate of fire.
    pub ROF: u8,

    /// When this object fires, the range at which it's projectiles travel is
    /// controlled by this value. The value represents the number of cells the
    /// projectile will travel. Objects outside of this range will not be fired
    /// upon (in normal circumstances).
    pub Range: u32,

    /// This is the typical sound generated when firing.
    Sound: Option<VocType>,

    /// This is the animation to display at the firing coordinate.
    Anim: Option<AnimType>,
}

impl WeaponTypeClass {
    pub const fn new(
        fires: BulletType,
        attack: u8,
        rof: u8,
        range: u32,
        sound: Option<VocType>,
        anim: Option<AnimType>,
    ) -> Self {
        Self {
            Fires: fires,
            Attack: attack,
            ROF: rof,
            Range: range,
            Sound: sound,
            Anim: anim,
        }
    }
}

///	These are the various weapons and their characteristics.
///
///bullet type				dmg,	rof,	range,	sound
pub const Weapons: [WeaponTypeClass; WeaponType::COUNT] = [
    WeaponTypeClass::new(BULLET_SNIPER, 125, 40, 0x0580, Some(VOC_SNIPER), None), //	WEAPON_RIFLE
    WeaponTypeClass::new(
        BULLET_SPREADFIRE,
        25,
        50,
        0x0400,
        Some(VOC_MINI),
        Some(ANIM_GUN_N),
    ), //	WEAPON_CHAIN_GUN
    WeaponTypeClass::new(BULLET_BULLET, 1, 7, 0x01C0, Some(VOC_RIFLE), None),     //	WEAPON_PISTOL
    WeaponTypeClass::new(BULLET_BULLET, 15, 20, 0x0200, Some(VOC_MGUN2), None),   //	WEAPON_M16
    WeaponTypeClass::new(BULLET_TOW, 30, 60, 0x0400, Some(VOC_BAZOOKA), None),    //	WEAPON_DRAGON
    WeaponTypeClass::new(
        BULLET_FLAME,
        35,
        50,
        0x0200,
        Some(VOC_FLAMER1),
        Some(ANIM_FLAME_N),
    ), //	WEAPON_FLAMETHROWER
    WeaponTypeClass::new(
        BULLET_FLAME,
        50,
        50,
        0x0200,
        Some(VOC_FLAMER1),
        Some(ANIM_FLAME_N),
    ), //	WEAPON_FLAME_TONGUE
    WeaponTypeClass::new(
        BULLET_CHEMSPRAY,
        80,
        70,
        0x0200,
        Some(VOC_FLAMER1),
        Some(ANIM_CHEM_N),
    ), //	WEAPON_CHEMSPRAY
    WeaponTypeClass::new(BULLET_GRENADE, 50, 60, 0x0340, Some(VOC_TOSS), None),   //	WEAPON_GRENADE
    WeaponTypeClass::new(
        BULLET_APDS,
        25,
        60,
        0x0400,
        Some(VOC_TANK2),
        Some(ANIM_MUZZLE_FLASH),
    ), //	WEAPON_75MM
    WeaponTypeClass::new(
        BULLET_APDS,
        30,
        50,
        0x04C0,
        Some(VOC_TANK3),
        Some(ANIM_MUZZLE_FLASH),
    ), //	WEAPON_105MM
    WeaponTypeClass::new(
        BULLET_APDS,
        40,
        80,
        0x04C0,
        Some(VOC_TANK4),
        Some(ANIM_MUZZLE_FLASH),
    ), //	WEAPON_120MM
    WeaponTypeClass::new(
        BULLET_APDS,
        40,
        60,
        0x0600,
        Some(VOC_TANK4),
        Some(ANIM_MUZZLE_FLASH),
    ), //	WEAPON_TURRET_GUN
    WeaponTypeClass::new(BULLET_SSM, 75, 80, 0x0500, Some(VOC_ROCKET1), None), //	WEAPON_MAMMOTH_TUSK
    WeaponTypeClass::new(BULLET_SSM2, 75, 80, 0x0600, Some(VOC_ROCKET1), None), //	WEAPON_MLRS
    WeaponTypeClass::new(
        BULLET_HE,
        150,
        65,
        0x0600,
        Some(VOC_TANK1),
        Some(ANIM_MUZZLE_FLASH),
    ), //	WEAPON_155MM
    WeaponTypeClass::new(
        BULLET_BULLET,
        15,
        30,
        0x0400,
        Some(VOC_MGUN11),
        Some(ANIM_GUN_N),
    ), //	WEAPON_M60MG
    WeaponTypeClass::new(BULLET_SSM, 60, 35, 0x0780, Some(VOC_ROCKET2), None), //	WEAPON_TOMAHAWK
    WeaponTypeClass::new(BULLET_SSM, 60, 40, 0x0680, Some(VOC_ROCKET2), None), //	WEAPON_TOW_TWO
    WeaponTypeClass::new(BULLET_NAPALM, 100, 20, 0x0480, None, None),          //	WEAPON_NAPALM
    WeaponTypeClass::new(BULLET_LASER, 200, 90, 0x0780, Some(VOC_LASER), None), //	WEAPON_OBELISK_LASER
    WeaponTypeClass::new(BULLET_SAM, 50, 50, 0x0780, Some(VOC_ROCKET2), None),  //	WEAPON_NIKE
    WeaponTypeClass::new(
        BULLET_HONEST_JOHN,
        100,
        200,
        0x0A00,
        Some(VOC_ROCKET1),
        None,
    ), //	WEAPON_HONEST_JOHN
    WeaponTypeClass::new(BULLET_HEADBUTT, 100, 30, 0x0180, Some(VOC_DINOATK1), None), // WEAPON_STEG
    WeaponTypeClass::new(BULLET_TREXBITE, 155, 30, 0x0180, Some(VOC_DINOATK1), None), // WEAPON_TREX
];

impl Index<WeaponType> for [WeaponTypeClass; WeaponType::COUNT] {
    type Output = WeaponTypeClass;

    fn index(&self, index: WeaponType) -> &Self::Output {
        &self[index as usize]
    }
}
