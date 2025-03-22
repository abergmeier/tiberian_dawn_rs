#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_variables)]

use crate::animation::AnimType::*;
use crate::bullet::BulletType::*;
use crate::bullet::BulletTypeClass;
use crate::bullet::WarheadType::*;
use crate::speed::MPHType::*;

/***************************************************************************
**	Detailed information about each class of bullet (projectile) in the game.
*/
const ClassSniper: BulletTypeClass = BulletTypeClass::new(
    BULLET_BULLET,
    "50cal",              // NAME:			Text name of this unit type.
    false,                // Flies over tall walls?
    false,                // Homes in on target?
    false,                // Projectile arcs to the target?
    false,                // Is this a dropping bomb-like object?
    true,                 // Is this projectile invisible?
    false,                // Will it blow up even if it gets just NEAR to target?
    false,                // Does it have flickering flame animation?
    false,                // Can it run out of fuel?
    true,                 // Is there no visual difference between projectile facings?
    false,                // Is projectile inherently inaccurate?
    false,                // Translucent colors are used?
    false,                // Good against aircraft?
    0,                    // ARMING:		Time to arm projectile after launch.
    0,                    // RANGE:		Inherent override range factor.
    MPH_LIGHT_SPEED,      // SPEED:		Miles per hour.
    0,                    // ROT:			Rate of turn (degrees per tick).
    WARHEAD_HOLLOW_POINT, // WARHEAD:		If fires weapon, warhead type
    Some(ANIM_PIFF),      // Explosion to use upon impact.
);

const ClassBullet: BulletTypeClass = BulletTypeClass::new(
    BULLET_BULLET,
    "50cal",         // NAME:			Text name of this unit type.
    false,           // Flies over tall walls?
    false,           // Homes in on target?
    false,           // Projectile arcs to the target?
    false,           // Is this a dropping bomb-like object?
    true,            // Is this projectile invisible?
    false,           // Will it blow up even if it gets just NEAR to target?
    false,           // Does it have flickering flame animation?
    false,           // Can it run out of fuel?
    true,            // Is there no visual difference between projectile facings?
    false,           // Is projectile inherently inaccurate?
    false,           // Translucent colors are used?
    false,           // Good against aircraft?
    0,               // ARMING:		Time to arm projectile after launch.
    0,               // RANGE:		Inherent override range factor.
    MPH_LIGHT_SPEED, // SPEED:		Miles per hour.
    0,               // ROT:			Rate of turn (degrees per tick).
    WARHEAD_SA,      // WARHEAD:		If fires weapon, warhead type
    Some(ANIM_PIFF), // Explosion to use upon impact.
);

const ClassSpreadfire: BulletTypeClass = BulletTypeClass::new(
    BULLET_SPREADFIRE,
    "50cal",             // NAME:			Text name of this unit type.
    true,                // Flies over tall walls?
    false,               // Homes in on target?
    false,               // Projectile arcs to the target?
    false,               // Is this a dropping bomb-like object?
    true,                // Is this projectile invisible?
    false,               // Will it blow up even if it gets just NEAR to target?
    false,               // Does it have flickering flame animation?
    false,               // Can it run out of fuel?
    true,                // Is there no visual difference between projectile facings?
    false,               // Is projectile inherently inaccurate?
    false,               // Translucent colors are used?
    false,               // Good against aircraft?
    0,                   // ARMING:		Time to arm projectile after launch.
    0,                   // RANGE:		Inherent override range factor.
    MPH_LIGHT_SPEED,     // SPEED:		Miles per hour.
    0,                   // ROT:			Rate of turn (degrees per tick).
    WARHEAD_HE,          // WARHEAD:		If fires weapon, warhead type
    Some(ANIM_PIFFPIFF), // Explosion to use upon impact.
);

const ClassAPDS: BulletTypeClass = BulletTypeClass::new(
    BULLET_APDS,
    "120mm",             // NAME:			Text name of this unit type.
    false,               // Flies over tall walls?
    false,               // Homes in on target?
    false,               // Projectile arcs to the target?
    false,               // Is this a dropping bomb-like object?
    false,               // Is this projectile invisible?
    false,               // Will it blow up even if it gets just NEAR to target?
    false,               // Does it have flickering flame animation?
    false,               // Can it run out of fuel?
    true,                // Is there no visual difference between projectile facings?
    false,               // Is projectile inherently inaccurate?
    false,               // Translucent colors are used?
    false,               // Good against aircraft?
    0,                   // ARMING:		Time to arm projectile after launch.
    0,                   // RANGE:		Inherent override range factor.
    MPH_VERY_FAST,       // SPEED:		Miles per hour.
    0,                   // ROT:			Rate of turn (degrees per tick).
    WARHEAD_AP,          // WARHEAD:		If fires weapon, warhead type
    Some(ANIM_VEH_HIT3), // Explosion to use upon impact.
);

const Class120mm: BulletTypeClass = BulletTypeClass::new(
    BULLET_HE,
    "120mm",             // NAME:			Text name of this unit type.
    true,                // Flies over tall walls?
    false,               // Homes in on target?
    true,                // Projectile arcs to the target?
    false,               // Is this a dropping bomb-like object?
    false,               // Is this projectile invisible?
    false,               // Will it blow up even if it gets just NEAR to target?
    false,               // Does it have flickering flame animation?
    false,               // Can it run out of fuel?
    true,                // Is there no visual difference between projectile facings?
    true,                // Is projectile inherently inaccurate?
    false,               // Translucent colors are used?
    false,               // Good against aircraft?
    0,                   // ARMING:		Time to arm projectile after launch.
    0,                   // RANGE:		Inherent override range factor.
    MPH_MEDIUM_FAST,     // SPEED:		Miles per hour.
    0,                   // ROT:			Rate of turn (degrees per tick).
    WARHEAD_HE,          // WARHEAD:		If fires weapon, warhead type
    Some(ANIM_ART_EXP1), // Explosion to use upon impact.
);

const ClassMissile: BulletTypeClass = BulletTypeClass::new(
    BULLET_SSM,
    "DRAGON",         // NAME:			Text name of this unit type.
    true,             // Flies over tall walls?
    true,             // Homes in on target?
    false,            // Projectile arcs to the target?
    false,            // Is this a dropping bomb-like object?
    false,            // Is this projectile invisible?
    true,             // Will it blow up even if it gets just NEAR to target?
    true,             // Does it have flickering flame animation?
    true,             // Can it run out of fuel?
    false,            // Is there no visual difference between projectile facings?
    true,             // Is projectile inherently inaccurate?
    true,             // Translucent colors are used?
    true,             // Good against aircraft?
    7,                // ARMING:		Time to arm projectile after launch.
    0,                // RANGE:		Inherent override range factor.
    MPH_ROCKET,       // SPEED:		Miles per hour.
    5,                // ROT:			Rate of turn (degrees per tick).
    WARHEAD_HE,       // WARHEAD:		If fires weapon, warhead type
    Some(ANIM_FRAG1), // Explosion to use upon impact.
);

const ClassMissile2: BulletTypeClass = BulletTypeClass::new(
    BULLET_SSM2,
    "DRAGON",         // NAME:			Text name of this unit type.
    true,             // Flies over tall walls?
    true,             // Homes in on target?
    false,            // Projectile arcs to the target?
    false,            // Is this a dropping bomb-like object?
    false,            // Is this projectile invisible?
    true,             // Will it blow up even if it gets just NEAR to target?
    true,             // Does it have flickering flame animation?
    true,             // Can it run out of fuel?
    false,            // Is there no visual difference between projectile facings?
    true,             // Is projectile inherently inaccurate?
    true,             // Translucent colors are used?
    true,             // Good against aircraft?
    9,                // ARMING:		Time to arm projectile after launch.
    0,                // RANGE:		Inherent override range factor.
    MPH_ROCKET,       // SPEED:		Miles per hour.
    7,                // ROT:			Rate of turn (degrees per tick).
    WARHEAD_HE,       // WARHEAD:		If fires weapon, warhead type
    Some(ANIM_FRAG1), // Explosion to use upon impact.
);

const ClassPatriot: BulletTypeClass = BulletTypeClass::new(
    BULLET_SAM,
    "MISSILE",           // NAME:			Text name of this unit type.
    true,                // Flies over tall walls?
    true,                // Homes in on target?
    false,               // Projectile arcs to the target?
    false,               // Is this a dropping bomb-like object?
    false,               // Is this projectile invisible?
    true,                // Will it blow up even if it gets just NEAR to target?
    true,                // Does it have flickering flame animation?
    true,                // Can it run out of fuel?
    false,               // Is there no visual difference between projectile facings?
    false,               // Is projectile inherently inaccurate?
    false,               // Translucent colors are used?
    true,                // Good against aircraft?
    0,                   // ARMING:		Time to arm projectile after launch.
    0,                   // RANGE:		Inherent override range factor.
    MPH_VERY_FAST,       // SPEED:		Miles per hour.
    10,                  // ROT:			Rate of turn (degrees per tick).
    WARHEAD_AP,          // WARHEAD:		If fires weapon, warhead type
    Some(ANIM_VEH_HIT1), // Explosion to use upon impact.
);

const ClassDragon: BulletTypeClass = BulletTypeClass::new(
    BULLET_TOW,
    "DRAGON",            // NAME:			Text name of this unit type.
    true,                // Flies over tall walls?
    true,                // Homes in on target?
    false,               // Projectile arcs to the target?
    false,               // Is this a dropping bomb-like object?
    false,               // Is this projectile invisible?
    true,                // Will it blow up even if it gets just NEAR to target?
    true,                // Does it have flickering flame animation?
    true,                // Can it run out of fuel?
    false,               // Is there no visual difference between projectile facings?
    false,               // Is projectile inherently inaccurate?
    true,                // Translucent colors are used?
    true,                // Good against aircraft?
    3,                   // ARMING:		Time to arm projectile after launch.
    0,                   // RANGE:		Inherent override range factor.
    MPH_ROCKET,          // SPEED:		Miles per hour.
    5,                   // ROT:			Rate of turn (degrees per tick).
    WARHEAD_AP,          // WARHEAD:		If fires weapon, warhead type
    Some(ANIM_VEH_HIT2), // Explosion to use upon impact.
);

const ClassFlame: BulletTypeClass = BulletTypeClass::new(
    BULLET_FLAME,
    "FLAME",      // NAME:			Text name of this unit type.
    false,        // Flies over tall walls?
    false,        // Homes in on target?
    false,        // Projectile arcs to the target?
    false,        // Is this a dropping bomb-like object?
    true,         // Is this projectile invisible?
    false,        // Will it blow up even if it gets just NEAR to target?
    false,        // Does it have flickering flame animation?
    true,         // Can it run out of fuel?
    false,        // Is there no visual difference between projectile facings?
    false,        // Is projectile inherently inaccurate?
    false,        // Translucent colors are used?
    false,        // Good against aircraft?
    12,           // ARMING:		Time to arm projectile after launch.
    12,           // RANGE:		Inherent override range factor.
    MPH_FAST,     // SPEED:		Miles per hour.
    0,            // ROT:			Rate of turn (degrees per tick).
    WARHEAD_FIRE, // WARHEAD:		If fires weapon, warhead type
    None,         // Explosion to use upon impact.
);

const ClassChem: BulletTypeClass = BulletTypeClass::new(
    BULLET_CHEMSPRAY,
    "FLAME",    // NAME:			Text name of this unit type.
    false,      // Flies over tall walls?
    false,      // Homes in on target?
    false,      // Projectile arcs to the target?
    false,      // Is this a dropping bomb-like object?
    true,       // Is this projectile invisible?
    false,      // Will it blow up even if it gets just NEAR to target?
    false,      // Does it have flickering flame animation?
    true,       // Can it run out of fuel?
    false,      // Is there no visual difference between projectile facings?
    false,      // Is projectile inherently inaccurate?
    false,      // Translucent colors are used?
    false,      // Good against aircraft?
    12,         // ARMING:		Time to arm projectile after launch.
    12,         // RANGE:		Inherent override range factor.
    MPH_FAST,   // SPEED:		Miles per hour.
    0,          // ROT:			Rate of turn (degrees per tick).
    WARHEAD_HE, // WARHEAD:		If fires weapon, warhead type
    None,       // Explosion to use upon impact.
);

const ClassNapalm: BulletTypeClass = BulletTypeClass::new(
    BULLET_NAPALM,
    "BOMBLET",          // NAME:			Text name of this unit type.
    true,               // Flies over tall walls?
    false,              // Homes in on target?
    false,              // Projectile arcs to the target?
    true,               // Is this a dropping bomb-like object?
    false,              // Is this projectile invisible?
    false,              // Will it blow up even if it gets just NEAR to target?
    false,              // Does it have flickering flame animation?
    false,              // Can it run out of fuel?
    true,               // Is there no visual difference between projectile facings?
    false,              // Is projectile inherently inaccurate?
    true,               // Translucent colors are used?
    false,              // Good against aircraft?
    24,                 // ARMING:		Time to arm projectile after launch.
    24,                 // RANGE:		Inherent override range factor.
    MPH_MEDIUM_SLOW,    // SPEED:		Miles per hour.
    0,                  // ROT:			Rate of turn (degrees per tick).
    WARHEAD_FIRE,       // WARHEAD:		If fires weapon, warhead type
    Some(ANIM_NAPALM2), // Explosion to use upon impact.
);

const ClassGrenade: BulletTypeClass = BulletTypeClass::new(
    BULLET_GRENADE,
    "BOMB",              // NAME:			Text name of this unit type.
    true,                // Flies over tall walls?
    false,               // Homes in on target?
    true,                // Projectile arcs to the target?
    false,               // Is this a dropping bomb-like object?
    false,               // Is this projectile invisible?
    false,               // Will it blow up even if it gets just NEAR to target?
    false,               // Does it have flickering flame animation?
    false,               // Can it run out of fuel?
    true,                // Is there no visual difference between projectile facings?
    true,                // Is projectile inherently inaccurate?
    true,                // Translucent colors are used?
    false,               // Good against aircraft?
    0,                   // ARMING:		Time to arm projectile after launch.
    0,                   // RANGE:		Inherent override range factor.
    MPH_MEDIUM_SLOW,     // SPEED:		Miles per hour.
    0,                   // ROT:			Rate of turn (degrees per tick).
    WARHEAD_HE,          // WARHEAD:		If fires weapon, warhead type
    Some(ANIM_VEH_HIT2), // Explosion to use upon impact.
);

const ClassLaser: BulletTypeClass = BulletTypeClass::new(
    BULLET_LASER,
    "Laser",         // NAME:			Text name of this unit type.
    true,            // Flies over tall walls?
    false,           // Homes in on target?
    false,           // Projectile arcs to the target?
    false,           // Is this a dropping bomb-like object?
    true,            // Is this projectile invisible?
    false,           // Will it blow up even if it gets just NEAR to target?
    false,           // Does it have flickering flame animation?
    false,           // Can it run out of fuel?
    true,            // Is there no visual difference between projectile facings?
    false,           // Is projectile inherently inaccurate?
    false,           // Translucent colors are used?
    false,           // Good against aircraft?
    0,               // ARMING:		Time to arm projectile after launch.
    0,               // RANGE:		Inherent override range factor.
    MPH_LIGHT_SPEED, // SPEED:		Miles per hour.
    0,               // ROT:			Rate of turn (degrees per tick).
    WARHEAD_LASER,   // WARHEAD:		If fires weapon, warhead type
    None,            // Explosion to use upon impact.
);

const ClassNukeUp: BulletTypeClass = BulletTypeClass::new(
    BULLET_NUKE_UP,
    "ATOMICUP",       // NAME:			Text name of this unit type.
    true,             // Flies over tall walls?
    false,            // Homes in on target?
    false,            // Projectile arcs to the target?
    false,            // Is this a dropping bomb-like object?
    false,            // Is this projectile invisible?
    true,             // Will it blow up even if it gets just NEAR to target?
    false,            // Does it have flickering flame animation?
    false,            // Can it run out of fuel?
    true,             // Is there no visual difference between projectile facings?
    false,            // Is projectile inherently inaccurate?
    false,            // Translucent colors are used?
    false,            // Good against aircraft?
    0,                // ARMING:		Time to arm projectile after launch.
    0,                // RANGE:		Inherent override range factor.
    MPH_VERY_FAST,    // SPEED:		Miles per hour.
    0,                // ROT:			Rate of turn (degrees per tick).
    WARHEAD_HE,       // WARHEAD:		If fires weapon, warhead type
    Some(ANIM_FRAG1), // Explosion to use upon impact.
);

const ClassNukeDown: BulletTypeClass = BulletTypeClass::new(
    BULLET_NUKE_DOWN,
    "ATOMICDN",            // NAME:			Text name of this unit type.
    true,                  // Flies over tall walls?
    false,                 // Homes in on target?
    false,                 // Projectile arcs to the target?
    false,                 // Is this a dropping bomb-like object?
    false,                 // Is this projectile invisible?
    true,                  // Will it blow up even if it gets just NEAR to target?
    false,                 // Does it have flickering flame animation?
    false,                 // Can it run out of fuel?
    true,                  // Is there no visual difference between projectile facings?
    false,                 // Is projectile inherently inaccurate?
    false,                 // Translucent colors are used?
    false,                 // Good against aircraft?
    0,                     // ARMING:		Time to arm projectile after launch.
    0,                     // RANGE:		Inherent override range factor.
    MPH_VERY_FAST,         // SPEED:		Miles per hour.
    0,                     // ROT:			Rate of turn (degrees per tick).
    WARHEAD_HE,            // WARHEAD:		If fires weapon, warhead type
    Some(ANIM_ATOM_BLAST), // Explosion to use upon impact.
);

const ClassHonestJohn: BulletTypeClass = BulletTypeClass::new(
    BULLET_HONEST_JOHN,
    "MISSILE",          // NAME:			Text name of this unit type.
    true,               // Flies over tall walls?
    false,              // Homes in on target?
    false,              // Projectile arcs to the target?
    false,              // Is this a dropping bomb-like object?
    false,              // Is this projectile invisible?
    true,               // Will it blow up even if it gets just NEAR to target?
    true,               // Does it have flickering flame animation?
    true,               // Can it run out of fuel?
    false,              // Is there no visual difference between projectile facings?
    false,              // Is projectile inherently inaccurate?
    false,              // Translucent colors are used?
    false,              // Good against aircraft?
    10,                 // ARMING:		Time to arm projectile after launch.
    0,                  // RANGE:		Inherent override range factor.
    MPH_FAST,           // SPEED:		Miles per hour.
    10,                 // ROT:			Rate of turn (degrees per tick).
    WARHEAD_FIRE,       // WARHEAD:		If fires weapon, warhead type
    Some(ANIM_NAPALM3), // Explosion to use upon impact.
);

const ClassHeadButt: BulletTypeClass = BulletTypeClass::new(
    BULLET_HEADBUTT,
    "GORE",           // NAME:			Text name of this unit type.
    false,            // Flies over tall walls?
    false,            // Homes in on target?
    false,            // Projectile arcs to the target?
    false,            // Is this a dropping bomb-like object?
    true,             // Is this projectile invisible?
    false,            // Will it blow up even if it gets just NEAR to target?
    false,            // Does it have flickering flame animation?
    false,            // Can it run out of fuel?
    true,             // Is there no visual difference between projectile facings?
    false,            // Is projectile inherently inaccurate?
    false,            // Translucent colors are used?
    false,            // Good against aircraft?
    0,                // ARMING:		Time to arm projectile after launch.
    0,                // RANGE:		Inherent override range factor.
    MPH_LIGHT_SPEED,  // SPEED:		Miles per hour.
    0,                // ROT:			Rate of turn (degrees per tick).
    WARHEAD_HEADBUTT, // WARHEAD:		If fires weapon, warhead type
    None,             // Explosion to use upon impact.
);

const ClassTRexBite: BulletTypeClass = BulletTypeClass::new(
    BULLET_TREXBITE,
    "CHEW",          // NAME:			Text name of this unit type.
    false,           // Flies over tall walls?
    false,           // Homes in on target?
    false,           // Projectile arcs to the target?
    false,           // Is this a dropping bomb-like object?
    true,            // Is this projectile invisible?
    false,           // Will it blow up even if it gets just NEAR to target?
    false,           // Does it have flickering flame animation?
    false,           // Can it run out of fuel?
    true,            // Is there no visual difference between projectile facings?
    false,           // Is projectile inherently inaccurate?
    false,           // Translucent colors are used?
    false,           // Good against aircraft?
    0,               // ARMING:		Time to arm projectile after launch.
    0,               // RANGE:		Inherent override range factor.
    MPH_LIGHT_SPEED, // SPEED:		Miles per hour.
    0,               // ROT:			Rate of turn (degrees per tick).
    WARHEAD_FEEDME,  // WARHEAD:		If fires weapon, warhead type
    None,            // Explosion to use upon impact.
);
