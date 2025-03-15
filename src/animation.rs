#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables
)]

use crate::{audio::VocType, object::ObjectTypeClass};

#[derive(Debug)]
pub struct AnimControlType {
    pub Start: i32, // Starting frame of animation.
    pub Count: i32, // Number of frames in this animation.
    pub Rate: i32,  // Number of ticks to delay between each frame.
}

impl AnimControlType {
    pub const fn default() -> Self {
        Self {
            Start: 0,
            Count: 0,
            Rate: 0,
        }
    }
}

///
///	These are the enumerated animation sequences that a building may
///	be processing. These serve to control the way that a building
///	appears.
#[repr(u8)]
pub enum BStateType {
    //BSTATE_NONE=-1,
    BSTATE_CONSTRUCTION, // Construction animation.
    BSTATE_IDLE,         // Idle animation.
    BSTATE_ACTIVE,       // Animation when building is "doing its thing".
    BSTATE_FULL,         // Special alternate active state.
    BSTATE_AUX1,         // Auxiliary animation.
    BSTATE_AUX2,         // Auxiliary animation.
}

pub const BSTATE_COUNT: usize = 6;

///Animations are enumerated here. Animations are the high speed and
/// short lived effects that occur with explosions and fire.
#[repr(u8)]
pub enum AnimType {
    //ANIM_NONE=-1,
    ANIM_FBALL1,       // Large fireball explosion (bulges rightward).
    ANIM_GRENADE,      // Genade (dirt type) explosion.
    ANIM_FRAG1,        // Medium fragment throwing explosion -- short decay.
    ANIM_FRAG2,        // Medium fragment throwing explosion -- long decay.
    ANIM_VEH_HIT1,     //	Small fireball explosion (bulges rightward).
    ANIM_VEH_HIT2,     //	Small fragment throwing explosion -- pop & sparkles.
    ANIM_VEH_HIT3,     // Small fragment throwing explosion -- burn/exp mix.
    ANIM_ART_EXP1,     // Large fragment throwing explosion -- many sparkles.
    ANIM_NAPALM1,      // Small napalm burn.
    ANIM_NAPALM2,      // Medium napalm burn.
    ANIM_NAPALM3,      // Large napalm burn.
    ANIM_SMOKE_PUFF,   // Small rocket smoke trail puff.
    ANIM_PIFF,         // Machine gun impact piffs.
    ANIM_PIFFPIFF,     // Chaingun impact piffs.
    ANIM_FLAME_N,      // Flame thrower firing north.
    ANIM_FLAME_NE,     //	Flame thrower firing north east.
    ANIM_FLAME_E,      // Flame thrower firing east.
    ANIM_FLAME_SE,     // Flame thrower firing south east.
    ANIM_FLAME_S,      // Flame thrower firing south.
    ANIM_FLAME_SW,     // Flame thrower firing south west.
    ANIM_FLAME_W,      // Flame thrower firing west.
    ANIM_FLAME_NW,     // Flame thrower firing north west.
    ANIM_CHEM_N,       // Chem sprayer firing north.
    ANIM_CHEM_NE,      //	Chem sprayer firing north east.
    ANIM_CHEM_E,       // Chem sprayer firing east.
    ANIM_CHEM_SE,      // Chem sprayer firing south east.
    ANIM_CHEM_S,       // Chem sprayer firing south.
    ANIM_CHEM_SW,      // Chem sprayer firing south west.
    ANIM_CHEM_W,       // Chem sprayer firing west.
    ANIM_CHEM_NW,      // Chem sprayer firing north west.
    ANIM_FIRE_SMALL,   // Small flame animation.
    ANIM_FIRE_MED,     // Medium flame animation.
    ANIM_FIRE_MED2,    // Medium flame animation (oranger).
    ANIM_FIRE_TINY,    // Very tiny flames.
    ANIM_MUZZLE_FLASH, // Big cannon flash (with translucency).
    // #ifdef NEVER
    // 	ANIM_E1_ROT_FIRE,				// Infantry decay animations.
    // 	ANIM_E1_ROT_GRENADE,
    // 	ANIM_E1_ROT_GUN,
    // 	ANIM_E1_ROT_EXP,
    // 	ANIM_E2_ROT_FIRE,
    // 	ANIM_E2_ROT_GRENADE,
    // 	ANIM_E2_ROT_GUN,
    // 	ANIM_E2_ROT_EXP,
    // 	ANIM_E3_ROT_FIRE,
    // 	ANIM_E3_ROT_GRENADE,
    // 	ANIM_E3_ROT_GUN,
    // 	ANIM_E3_ROT_EXP,
    // 	ANIM_E4_ROT_FIRE,
    // 	ANIM_E4_ROT_GRENADE,
    // 	ANIM_E4_ROT_GUN,
    // 	ANIM_E4_ROT_EXP,
    // #endif
    ANIM_SMOKE_M,       // Smoke rising from ground.
    ANIM_BURN_SMALL,    // Small combustable fire effect (with trail off).
    ANIM_BURN_MED,      // Medium combustable fire effect (with trail off).
    ANIM_BURN_BIG,      // Large combustable fire effect (with trail off).
    ANIM_ON_FIRE_SMALL, // Burning effect for buildings.
    ANIM_ON_FIRE_MED,   // Burning effect for buildings.
    ANIM_ON_FIRE_BIG,   // Burning effect for buildings.
    ANIM_SAM_N,
    ANIM_SAM_NE,
    ANIM_SAM_E,
    ANIM_SAM_SE,
    ANIM_SAM_S,
    ANIM_SAM_SW,
    ANIM_SAM_W,
    ANIM_SAM_NW,
    ANIM_GUN_N,
    ANIM_GUN_NE,
    ANIM_GUN_E,
    ANIM_GUN_SE,
    ANIM_GUN_S,
    ANIM_GUN_SW,
    ANIM_GUN_W,
    ANIM_GUN_NW,
    ANIM_LZ_SMOKE,
    ANIM_ION_CANNON,
    ANIM_ATOM_BLAST,
    ANIM_CRATE_DEVIATOR, // Red finned missile.
    ANIM_CRATE_DOLLAR,   // Dollar sign.
    ANIM_CRATE_EARTH,    // Cracked Earth.
    ANIM_CRATE_EMPULSE,  // Plasma ball.
    ANIM_CRATE_INVUN,    // Orange sphere with green rings.
    ANIM_CRATE_MINE,     // Spiked mine.
    ANIM_CRATE_RAPID,    // Red skull.
    ANIM_CRATE_STEALTH,  // Cloaking sphere.
    ANIM_CRATE_MISSILE,  // Green finned missile.
    ANIM_ATOM_DOOR,
    ANIM_MOVE_FLASH,
    ANIM_OILFIELD_BURN,
    ANIM_TRIC_DIE,
    ANIM_TREX_DIE,
    ANIM_STEG_DIE,
    ANIM_RAPT_DIE,
    ANIM_CHEM_BALL, // Chemical warrior explosion.

                    //ANIM_COUNT,
                    //ANIM_FIRST=0
}

pub struct AnimTypeClass {
    objectTypeClass: ObjectTypeClass,
    /// If this animation should run at a constant apparent rate regardless
    /// of game speed setting, then this flag will be set to true.
    IsNormalized: bool,

    ///	If this animation should be rendered and sorted with the other ground
    ///	units, then this flag is true. Typical of this would be fire and other
    //	low altitude animation effects.
    IsGroundLayer: bool,

    ///	If this animation should be rendered in a translucent fashion, this flag
    ///	will be true. Translucent colors are some of the reds and some of the
    ///	greys. Typically, smoke and some fire effects have this flag set.
    IsTranslucent: bool,

    ///	If this animation uses the white translucent table, then this flag
    ///	will be true.
    IsWhiteTrans: bool,

    ///	If this is the special flame thrower animation, then custom affects
    ///	occur as it is playing. Specifically, scorch marks and little fire
    ///	pieces appear as the flame jets forth.
    IsFlameThrower: bool,

    ///	Some animations leave a scorch mark behind. Napalm and other flame
    ///	type explosions are typical of this type.
    IsScorcher: bool,

    ///	Some explosions are of such violence that they leave craters behind.
    ///	This flag will be true for those types.
    IsCraterForming: bool,

    ///	If this animation should attach itself to any unit that is in the same
    ///	location as itself, then this flag will be true. Most vehicle impact
    //	explosions are of this type.
    IsSticky: bool,

    ///	This is the type number for this animation kind. It can be used as
    //	a unique identifier for animation types.
    Type: AnimType,

    ///	This specified the maximum dimension of the shape (edge to edge). This dimension
    ///	is used to build the appropriate cell refresh list. Keep this value as small
    ///	as possible to ensure maximum performance. This is especially critical, since
    ///	animations always cause the cells under them to be redrawn every frame.
    Size: usize,

    ///	This is the frame that the animation is biggest. The biggest frame of animation
    ///	will hide any changes to underlying ground (e.g., craters) that the animation
    ///	causes, so these effects are delayed until this frame is reached. The end result
    ///	is to prevent the player from seeing craters "pop" into existance.
    Biggest: u8,

    ///	Some animations (when attached to another object) damage the object it
    ///	is in contact with. Fire is a good example of this. This value is a
    ///	fixed point number of the damage that is applied to the attached object
    ///	every game tick.
    Damage: usize,

    ///	Simple animation delay value between advancing of frames. This can
    ///	be overridden by the control list.
    Delay: u8,

    ///	The starting frame number for each animation sequence. Usually this is
    ///	zero, but can sometimes be different if this animation is a sub sequence
    ///	of a larger animation file.
    Start: i32,

    ///	Looping animations might start at a different frame than the initial one.
    ///	This is true for smoke effects that have a startup sequence followed by a
    ///	continuous looping sequence.
    LoopStart: i32,

    ///	For looping animations, this is the frame that will end all the middle loops
    ///	of the animation. The last loop of the animation will proceed until the Stages
    ///	has been fully completed.
    LoopEnd: i32,

    ///	The number of stages that this animation sequence will progress through
    ///	before it loops or ends.
    Stages: i32,

    ///	This is the normal loop count for this animation. Usually this is one, but
    ///	for some animations, it may be larger.
    Loops: u16,

    ///	This is the sound effect to play when this animation starts. Usually, this
    ///	applies to explosion animations.
    Sound: Option<VocType>,

    ///	If the animation is to launch into another animation, then
    ///	the secondary animation will be defined here.
    ChainTo: Option<AnimType>,
}

impl AnimTypeClass {
    /// This is the constructor for static objects that elaborate the various animation types
    /// allowed in the game. Each animation in the game is of one of these types.
    pub const fn new(
        anim: AnimType,
        name: &str,
        size: usize,
        biggest: u8,
        isnormal: bool,
        iswhitetrans: bool,
        isscorcher: bool,
        iscrater: bool,
        issticky: bool,
        ground: bool,
        istrans: bool,
        isflame: bool,
        damage: usize,
        delaytime: u8,
        start: i32,
        loopstart: i32,
        loopend: i32,
        stages: i32,
        loops: u16,
        sound: Option<VocType>,
        chainto: Option<AnimType>,
    ) -> Self {
        Self {
            objectTypeClass: ObjectTypeClass::new(
                true, false, false, true, false, false, true, true, None, name, None, 0,
            ),
            Biggest: biggest,
            ChainTo: chainto,
            Damage: damage,
            Delay: delaytime,
            IsCraterForming: iscrater,
            IsFlameThrower: isflame,
            IsGroundLayer: ground,
            IsNormalized: isnormal,
            IsScorcher: isscorcher,
            IsSticky: issticky,
            IsTranslucent: istrans,
            IsWhiteTrans: iswhitetrans,
            LoopEnd: loopend,
            LoopStart: loopstart,
            Loops: loops,
            Size: size,
            Sound: sound,
            Stages: stages,
            Start: start,
            Type: anim,
        }
    }
}
