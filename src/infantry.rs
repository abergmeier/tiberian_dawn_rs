#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables
)]

use std::ops::Index;

use crate::{
    building::{self},
    techno::TechnoTypeClass,
};
use strum::EnumCount;
use strum_macros::EnumCount;

use crate::{speed::MPHType, text::IDs, weapon::WeaponType};

///	This specifies the infantry in the game. The "E" designation is
///	similar to the army classification of enlisted soldiers.
pub enum InfantryType {
    //INFANTRY_NONE=-1,
    INFANTRY_E1,    // Mini-gun armed.
    INFANTRY_E2,    // Grenade thrower.
    INFANTRY_E3,    // Rocket launcher.
    INFANTRY_E4,    // Flame thrower equipped.
    INFANTRY_E5,    // Chemical thrower equipped.
    INFANTRY_E7,    // Engineer.
    INFANTRY_RAMBO, // Commando.

    INFANTRY_C1,      // Civilian
    INFANTRY_C2,      // Civilian
    INFANTRY_C3,      // Civilian
    INFANTRY_C4,      // Civilian
    INFANTRY_C5,      // Civilian
    INFANTRY_C6,      // Civilian
    INFANTRY_C7,      // Civilian
    INFANTRY_C8,      // Civilian
    INFANTRY_C9,      // Civilian
    INFANTRY_C10,     // Nikumba
    INFANTRY_MOEBIUS, // Dr. Moebius
    INFANTRY_DELPHI,  // Agent "Delphi"
    INFANTRY_CHAN,    // Dr. Chan

                      //INFANTRY_COUNT,
                      //INFANTRY_FIRST=0
}

///	Infantry can be performing various activities. These can range from simple
///	idle animations to physical hand to hand combat.
#[derive(Debug, EnumCount)]
#[repr(u8)]
pub enum DoType {
    //DO_NOTHING=-1,				// Not performing any choreographed sequence.
    DO_STAND_READY = 0,
    DO_STAND_GUARD,
    DO_PRONE,
    DO_WALK,
    DO_FIRE_WEAPON,
    DO_LIE_DOWN,
    DO_CRAWL,
    DO_GET_UP,
    DO_FIRE_PRONE,
    DO_IDLE1,
    DO_IDLE2,
    DO_ON_GUARD,
    DO_FIGHT_READY,
    DO_PUNCH,
    DO_KICK,
    DO_PUNCH_HIT1,
    DO_PUNCH_HIT2,
    DO_PUNCH_DEATH,
    DO_KICK_HIT1,
    DO_KICK_HIT2,
    DO_KICK_DEATH,
    DO_READY_WEAPON,
    DO_GUN_DEATH,
    DO_EXPLOSION_DEATH,
    DO_EXPLOSION2_DEATH,
    DO_GRENADE_DEATH,
    DO_FIRE_DEATH,
    DO_GESTURE1,
    DO_SALUTE1,
    DO_GESTURE2,
    DO_SALUTE2,
    // Civilian actions
    DO_PULL_GUN,
    DO_PLEAD,
    DO_PLEAD_DEATH,
    //DO_COUNT,
    //DO_FIRST=0
}

pub struct InfantryTypeClass {
    techno_type_class: TechnoTypeClass,
    ///	If this civilian infantry type is female, then this flag
    ///	will be true. This information is used to get the correct
    ///	voice response.
    pub IsFemale: bool,

    ///	Does this infantry unit have crawling animation? If not, then this
    ///	means that the "crawling" frames are actually running animation frames.
    pub IsCrawling: bool,

    ///	For those infantry types that can capture buildings, this flag
    ///	will be set to true. Typically, this is minigun soldiers.
    pub IsCapture: bool,

    ///	For infantry types that will run away from any damage causing
    ///	events, this flag will be true. Typically, this is so for all
    ///	civilians as well as the flame thrower guys.
    pub IsFraidyCat: bool,

    ///	This flags whether this infantry is actually a civilian. A
    ///	civilian uses different voice responses, has less ammunition,
    ///	and runs from danger more often.
    pub IsCivilian: bool,

    ///	This value represents the unit class. It can serve as a unique
    ///	identification number for this unit class.
    pub Type: InfantryType,

    ///	This is an array of the various animation frame data for the actions that
    ///	the infantry may perform.
    pub DoControls: [DoInfoStruct; DoType::COUNT],

    ///	There are certain units with special animation sequences built into the
    ///	shape file. These values tell how many frames are used for the firing animation.
    pub FireLaunch: u8,
    pub ProneLaunch: u8,
}

impl Index<DoType> for [DoInfoStruct; DoType::COUNT] {
    type Output = DoInfoStruct;

    fn index(&self, index: DoType) -> &Self::Output {
        &self[index as usize]
    }
}

pub struct DoInfoStruct {
    pub Frame: u16, // Starting frame of the animation.
    pub Count: u8,  // Number of frames of animation.
    pub Jump: u8,   // Frames to jump between facings.
}

impl DoInfoStruct {
    pub const fn new(frame: u16, count: u8, jump: u8) -> Self {
        Self {
            Frame: frame,
            Count: count,
            Jump: jump,
        }
    }
}

impl InfantryTypeClass {
    /// This routine will construct the infantry type objects. It is use to create the static    *
    /// infantry types that are used to give each of the infantry objects their characteristics. *
    pub const fn new(
        type_: InfantryType,
        name: IDs,
        ininame: &str,
        level: u8,
        pre: <building::Flags as bitflags::Flags>::Bits,
        is_female: bool,
        is_leader: bool,
        is_crawling: bool,
        is_civilian: bool,
        is_nominal: bool,
        is_fraidycat: bool,
        is_capture: bool,
        is_theater: bool,
        ammo: i32,
        do_table: [DoInfoStruct; DoType::COUNT],
        firelaunch: u8,
        pronelaunch: u8,
        strength: u16,
        sightrange: u16,
        cost: u32,
        scenario: u8,
        risk: u32,
        reward: u32,
        ownable: u16,
        primary: Option<WeaponType>,
        secondary: Option<WeaponType>,
        maxspeed: MPHType,
    ) -> Self {
        let s = Self {
            techno_type_class: TechnoTypeClass::new(
                name, ininame, level, pre, is_leader, true, is_nominal, false, false, true, true,
                true, true, false, false, is_theater, false, false, false, true, false, ammo,
                strength, maxspeed, sightrange, cost, scenario, risk, reward, ownable, primary,
                secondary, None,
            ),
            DoControls: do_table,
            IsFemale: is_female,
            IsCrawling: is_crawling,
            IsCapture: is_capture,
            IsFraidyCat: is_fraidycat,
            IsCivilian: is_civilian,
            Type: type_,
            FireLaunch: firelaunch,
            ProneLaunch: pronelaunch,
        };
        /*
         **	Set the animation sequence custom values.
         */

        //#ifdef cuts		//ST - 10/3/95 10:09AM
        // s.DoControls[DO_STAND_READY].Frame = dostandready;
        // s.DoControls[DO_STAND_READY].Count = dostandreadyframe;
        // s.DoControls[DO_STAND_READY].Jump = dostandreadyjump;

        // s.DoControls[DO_STAND_GUARD].Frame = dostandguard;
        // s.DoControls[DO_STAND_GUARD].Count = dostandguardframe;
        // s.DoControls[DO_STAND_GUARD].Jump = dostandguardjump;

        // s.DoControls[DO_PRONE].Frame = doprone;
        // s.DoControls[DO_PRONE].Count = doproneframe;
        // s.DoControls[DO_PRONE].Jump = dopronejump;

        // s.DoControls[DO_WALK].Frame = dowalk;
        // s.DoControls[DO_WALK].Count = dowalkframe;
        // s.DoControls[DO_WALK].Jump = dowalkjump;

        // s.DoControls[DO_FIRE_WEAPON].Frame = dofireweapon;
        // s.DoControls[DO_FIRE_WEAPON].Count = dofireweaponframe;
        // s.DoControls[DO_FIRE_WEAPON].Jump = dofireweaponjump;

        // s.DoControls[DO_LIE_DOWN].Frame = doliedown;
        // s.DoControls[DO_LIE_DOWN].Count = doliedownframe;
        // s.DoControls[DO_LIE_DOWN].Jump = doliedownjump;

        // s.DoControls[DO_CRAWL].Frame = docrawl;
        // s.DoControls[DO_CRAWL].Count = docrawlframe;
        // s.DoControls[DO_CRAWL].Jump = docrawljump;

        // s.DoControls[DO_GET_UP].Frame = dogetup;
        // s.DoControls[DO_GET_UP].Count = dogetupframe;
        // s.DoControls[DO_GET_UP].Jump = dogetupjump;

        // s.DoControls[DO_FIRE_PRONE].Frame = dofireprone;
        // s.DoControls[DO_FIRE_PRONE].Count = dofireproneframe;
        // s.DoControls[DO_FIRE_PRONE].Jump = dofirepronejump;

        // s.DoControls[DO_IDLE1].Frame = doidle1;
        // s.DoControls[DO_IDLE1].Count = doidle1frame;
        // s.DoControls[DO_IDLE1].Jump = doidle1jump;

        // s.DoControls[DO_IDLE2].Frame = doidle2;
        // s.DoControls[DO_IDLE2].Count = doidle2frame;
        // s.DoControls[DO_IDLE2].Jump = doidle2jump;

        // s.DoControls[DO_ON_GUARD].Frame = doonguard;
        // s.DoControls[DO_ON_GUARD].Count = doonguardframe;
        // s.DoControls[DO_ON_GUARD].Jump = doonguardjump;

        // s.DoControls[DO_FIGHT_READY].Frame = dofightready;
        // s.DoControls[DO_FIGHT_READY].Count = dofightreadyframe;
        // s.DoControls[DO_FIGHT_READY].Jump = dofightreadyjump;

        // s.DoControls[DO_PUNCH].Frame = dopunch;
        // s.DoControls[DO_PUNCH].Count = dopunchframe;
        // s.DoControls[DO_PUNCH].Jump = dopunchjump;

        // s.DoControls[DO_KICK].Frame = dokick;
        // s.DoControls[DO_KICK].Count = dokickframe;
        // s.DoControls[DO_KICK].Jump = dokickjump;

        // s.DoControls[DO_PUNCH_HIT1].Frame = dopunchhit1;
        // s.DoControls[DO_PUNCH_HIT1].Count = dopunchhit1frame;
        // s.DoControls[DO_PUNCH_HIT1].Jump = dopunchhit1jump;

        // s.DoControls[DO_PUNCH_HIT2].Frame = dopunchhit2;
        // s.DoControls[DO_PUNCH_HIT2].Count = dopunchhit2frame;
        // s.DoControls[DO_PUNCH_HIT2].Jump = dopunchhit2jump;

        // s.DoControls[DO_PUNCH_DEATH].Frame = dopunchdeath;
        // s.DoControls[DO_PUNCH_DEATH].Count = dopunchdeathframe;
        // s.DoControls[DO_PUNCH_DEATH].Jump = dopunchdeathjump;

        // s.DoControls[DO_KICK_HIT1].Frame = dokickhit1;
        // s.DoControls[DO_KICK_HIT1].Count = dokickhit1frame;
        // s.DoControls[DO_KICK_HIT1].Jump = dokickhit1jump;

        // s.DoControls[DO_KICK_HIT2].Frame = dokickhit2;
        // s.DoControls[DO_KICK_HIT2].Count = dokickhit2frame;
        // s.DoControls[DO_KICK_HIT2].Jump = dokickhit2jump;

        // s.DoControls[DO_KICK_DEATH].Frame = dokickdeath;
        // s.DoControls[DO_KICK_DEATH].Count = dokickdeathframe;
        // s.DoControls[DO_KICK_DEATH].Jump = dokickdeathjump;

        // s.DoControls[DO_READY_WEAPON].Frame = doreadyweapon;
        // s.DoControls[DO_READY_WEAPON].Count = doreadyweaponframe;
        // s.DoControls[DO_READY_WEAPON].Jump = doreadyweaponjump;

        // s.DoControls[DO_GUN_DEATH].Frame = dogundeath;
        // s.DoControls[DO_GUN_DEATH].Count = dogundeathframe;
        // s.DoControls[DO_GUN_DEATH].Jump = dogundeathjump;

        // s.DoControls[DO_EXPLOSION_DEATH].Frame = doexplosiondeath;
        // s.DoControls[DO_EXPLOSION_DEATH].Count = doexplosiondeathframe;
        // s.DoControls[DO_EXPLOSION_DEATH].Jump = doexplosiondeathjump;

        // s.DoControls[DO_EXPLOSION2_DEATH].Frame = doexplosion2death;
        // s.DoControls[DO_EXPLOSION2_DEATH].Count = doexplosion2deathframe;
        // s.DoControls[DO_EXPLOSION2_DEATH].Jump = doexplosion2deathjump;

        // s.DoControls[DO_GRENADE_DEATH].Frame = dogrenadedeath;
        // s.DoControls[DO_GRENADE_DEATH].Count = dogrenadedeathframe;
        // s.DoControls[DO_GRENADE_DEATH].Jump = dogrenadedeathjump;

        // s.DoControls[DO_FIRE_DEATH].Frame = dofiredeath;
        // s.DoControls[DO_FIRE_DEATH].Count = dofiredeathframe;
        // s.DoControls[DO_FIRE_DEATH].Jump = dofiredeathjump;

        // s.DoControls[DO_GESTURE1].Frame = dogesture1;
        // s.DoControls[DO_GESTURE1].Count = dogesture1frame;
        // s.DoControls[DO_GESTURE1].Jump = dogesture1jump;

        // s.DoControls[DO_SALUTE1].Frame = dosalute1;
        // s.DoControls[DO_SALUTE1].Count = dosalute1frame;
        // s.DoControls[DO_SALUTE1].Jump = dosalute1jump;

        // s.DoControls[DO_GESTURE2].Frame = dogesture2;
        // s.DoControls[DO_GESTURE2].Count = dogesture2frame;
        // s.DoControls[DO_GESTURE2].Jump = dogesture2jump;

        // s.DoControls[DO_SALUTE2].Frame = dosalute2;
        // s.DoControls[DO_SALUTE2].Count = dosalute2frame;
        // s.DoControls[DO_SALUTE2].Jump = dosalute2jump;

        // s.DoControls[DO_PULL_GUN].Frame = dopullgun;
        // s.DoControls[DO_PULL_GUN].Count = dopullgunframe;
        // s.DoControls[DO_PULL_GUN].Jump = dopullgunjump;

        // s.DoControls[DO_PLEAD].Frame = doplead;
        // s.DoControls[DO_PLEAD].Count = dopleadframe;
        // s.DoControls[DO_PLEAD].Jump = dopleadjump;

        // s.DoControls[DO_PLEAD_DEATH].Frame = dopleaddeath;
        // s.DoControls[DO_PLEAD_DEATH].Count = dopleaddeathframe;
        // s.DoControls[DO_PLEAD_DEATH].Jump = dopleaddeathjump;
        // #endif	//cuts
        s
    }
}
