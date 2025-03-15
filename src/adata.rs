#![allow(non_snake_case, non_upper_case_globals, unused_variables)]

use crate::animation::AnimType::*;
use crate::animation::AnimTypeClass;
use crate::audio::VocType::*;
// Dinosaur death animations
const TricDie: AnimTypeClass = AnimTypeClass::new(
	ANIM_TRIC_DIE,							// Animation number.
	"TRIC",									// Data name of animation.
	32,										// Maximum dimension of animation.
	4,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	true,										// Ground level animation?
	true,										// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	2,											// Delay between frames.
	176,										// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	20,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);

const TRexDie: AnimTypeClass = AnimTypeClass::new(
	ANIM_TREX_DIE,							// Animation number.
	"TREX",									// Data name of animation.
	48,										// Maximum dimension of animation.
	4,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	true,										// Ground level animation?
	true,										// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	2,											// Delay between frames.
	144,										// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	40,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);

const StegDie: AnimTypeClass = AnimTypeClass::new(
	ANIM_STEG_DIE,							// Animation number.
	"STEG",									// Data name of animation.
	33,										// Maximum dimension of animation.
	4,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	true,										// Ground level animation?
	true,										// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	2,											// Delay between frames.
	176,										// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	22,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);

const RaptDie: AnimTypeClass = AnimTypeClass::new(
	ANIM_RAPT_DIE,							// Animation number.
	"RAPT",									// Data name of animation.
	24,										// Maximum dimension of animation.
	4,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	true,										// Ground level animation?
	true,										// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	2,											// Delay between frames.
	144,										// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	40,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);

const SAMN: AnimTypeClass = AnimTypeClass::new(
	ANIM_SAM_N,								// Animation number.
	"SAMFIRE",								// Data name of animation.
	55,										// Maximum dimension of animation.
	4,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	18*0,										// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	18,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const SAMNW: AnimTypeClass = AnimTypeClass::new(
	ANIM_SAM_NW,							// Animation number.
	"SAMFIRE",								// Data name of animation.
	55,										// Maximum dimension of animation.
	22,										// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	18*1,										// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	18,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const SAMW: AnimTypeClass = AnimTypeClass::new(
	ANIM_SAM_W,								// Animation number.
	"SAMFIRE",								// Data name of animation.
	55,										// Maximum dimension of animation.
	40,										// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	18*2,										// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	18,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const SAMSW: AnimTypeClass = AnimTypeClass::new(
	ANIM_SAM_SW,							// Animation number.
	"SAMFIRE",								// Data name of animation.
	55,										// Maximum dimension of animation.
	58,										// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	18*3,										// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	18,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const SAMS: AnimTypeClass = AnimTypeClass::new(
	ANIM_SAM_S,								// Animation number.
	"SAMFIRE",								// Data name of animation.
	55,										// Maximum dimension of animation.
	76,										// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	18*4,										// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	18,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const SAMSE: AnimTypeClass = AnimTypeClass::new(
	ANIM_SAM_SE,							// Animation number.
	"SAMFIRE",								// Data name of animation.
	55,										// Maximum dimension of animation.
	94,										// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	18*5,										// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	18,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const SAME: AnimTypeClass = AnimTypeClass::new(
	ANIM_SAM_E,								// Animation number.
	"SAMFIRE",								// Data name of animation.
	55,										// Maximum dimension of animation.
	112,										// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	18*6,										// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	18,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const SAMNE: AnimTypeClass = AnimTypeClass::new(
	ANIM_SAM_NE,							// Animation number.
	"SAMFIRE",								// Data name of animation.
	55,										// Maximum dimension of animation.
	130,										// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	18*7,										// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	18,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);

const LZSmoke: AnimTypeClass = AnimTypeClass::new(
	ANIM_LZ_SMOKE,						// Animation number.
	"SMOKLAND",								// Data name of animation.
	32,										// Maximum dimension of animation.
	72,										// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	true,										// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	72,										// Loop start frame number.
	91,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	255,										// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);

/*
**	Flammable object burning animations. Primarily used on trees and buildings.
*/
const BurnSmall: AnimTypeClass = AnimTypeClass::new(
	ANIM_BURN_SMALL,						// Animation number.
	"BURN-S",								// Data name of animation.
	11,										// Maximum dimension of animation.
	13,										// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	true,										// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0008,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	30,										// Loop start frame number.
	62,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	4,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const BurnMed: AnimTypeClass = AnimTypeClass::new(
	ANIM_BURN_MED,							// Animation number.
	"BURN-M",								// Data name of animation.
	14,										// Maximum dimension of animation.
	13,										// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	true,										// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0010,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	30,										// Loop start frame number.
	62,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	4,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const BurnBig: AnimTypeClass = AnimTypeClass::new(
	ANIM_BURN_BIG,							// Animation number.
	"BURN-L",								// Data name of animation.
	23,										// Maximum dimension of animation.
	13,										// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	true,										// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	true,										// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0018,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	30,										// Loop start frame number.
	62,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	4,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);

/*
**	Flammable object burning animations that trail into smoke. Used for
**	buildings and the gunboat.
*/
const OnFireSmall: AnimTypeClass = AnimTypeClass::new(
	ANIM_ON_FIRE_SMALL,					// Animation number.
	"BURN-S",								// Data name of animation.
	11,										// Maximum dimension of animation.
	13,										// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	true,										// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0008,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	30,										// Loop start frame number.
	62,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	4,											// Number of times the animation loops.
	None,								// Sound effect to play.
	Some(ANIM_SMOKE_M)
);
const OnFireMed: AnimTypeClass = AnimTypeClass::new(
	ANIM_ON_FIRE_MED,						// Animation number.
	"BURN-M",								// Data name of animation.
	14,										// Maximum dimension of animation.
	13,										// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	true,										// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0010,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	30,										// Loop start frame number.
	62,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	4,											// Number of times the animation loops.
	None,								// Sound effect to play.
	Some(ANIM_ON_FIRE_SMALL)
);
const OnFireBig: AnimTypeClass = AnimTypeClass::new(
	ANIM_ON_FIRE_BIG,						// Animation number.
	"BURN-L",								// Data name of animation.
	23,										// Maximum dimension of animation.
	13,										// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	true,										// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	true,										// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0018,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	30,										// Loop start frame number.
	62,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	4,											// Number of times the animation loops.
	None,								// Sound effect to play.
	Some(ANIM_ON_FIRE_MED)
);

/*
**	Flame thrower animations. These are direction specific.
*/
const FlameN: AnimTypeClass = AnimTypeClass::new(
	ANIM_FLAME_N,							// Animation number.
	"FLAME-N",								// Data name of animation.
	0,											// Maximum dimension of animation.
	9,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	true,										// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const FlameNW: AnimTypeClass = AnimTypeClass::new(
	ANIM_FLAME_NW,							// Animation number.
	"FLAME-NW",								// Data name of animation.
	0,											// Maximum dimension of animation.
	9,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	true,										// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const FlameW: AnimTypeClass = AnimTypeClass::new(
	ANIM_FLAME_W,							// Animation number.
	"FLAME-W",								// Data name of animation.
	0,											// Maximum dimension of animation.
	9,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	true,										// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const FlameSW: AnimTypeClass = AnimTypeClass::new(
	ANIM_FLAME_SW,							// Animation number.
	"FLAME-SW",								// Data name of animation.
	0,											// Maximum dimension of animation.
	9,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	true,										// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const FlameS: AnimTypeClass = AnimTypeClass::new(
	ANIM_FLAME_S,							// Animation number.
	"FLAME-S",								// Data name of animation.
	0,											// Maximum dimension of animation.
	9,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	true,										// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const FlameSE: AnimTypeClass = AnimTypeClass::new(
	ANIM_FLAME_SE,							// Animation number.
	"FLAME-SE",								// Data name of animation.
	0,											// Maximum dimension of animation.
	9,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	true,										// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const FlameE: AnimTypeClass = AnimTypeClass::new(
	ANIM_FLAME_E,							// Animation number.
	"FLAME-E",								// Data name of animation.
	0,											// Maximum dimension of animation.
	9,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	true,										// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const FlameNE: AnimTypeClass = AnimTypeClass::new(
	ANIM_FLAME_NE,							// Animation number.
	"FLAME-NE",								// Data name of animation.
	0,											// Maximum dimension of animation.
	9,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	true,										// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);

/*
**	Chem sprayer animations. These are direction specific.
*/
const ChemN: AnimTypeClass = AnimTypeClass::new(
	ANIM_CHEM_N,							// Animation number.
	"CHEM-N",								// Data name of animation.
	0,											// Maximum dimension of animation.
	9,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const ChemNW: AnimTypeClass = AnimTypeClass::new(
	ANIM_CHEM_NW,							// Animation number.
	"CHEM-NW",								// Data name of animation.
	0,											// Maximum dimension of animation.
	9,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const ChemW: AnimTypeClass = AnimTypeClass::new(
	ANIM_CHEM_W,							// Animation number.
	"CHEM-W",								// Data name of animation.
	0,											// Maximum dimension of animation.
	9,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const ChemSW: AnimTypeClass = AnimTypeClass::new(
	ANIM_CHEM_SW,							// Animation number.
	"CHEM-SW",								// Data name of animation.
	0,											// Maximum dimension of animation.
	9,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const ChemS: AnimTypeClass = AnimTypeClass::new(
	ANIM_CHEM_S,							// Animation number.
	"CHEM-S",								// Data name of animation.
	0,											// Maximum dimension of animation.
	9,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const ChemSE: AnimTypeClass = AnimTypeClass::new(
	ANIM_CHEM_SE,							// Animation number.
	"CHEM-SE",								// Data name of animation.
	0,											// Maximum dimension of animation.
	9,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const ChemE: AnimTypeClass = AnimTypeClass::new(
	ANIM_CHEM_E,							// Animation number.
	"CHEM-E",								// Data name of animation.
	0,											// Maximum dimension of animation.
	9,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);
const ChemNE: AnimTypeClass = AnimTypeClass::new(
	ANIM_CHEM_NE,							// Animation number.
	"CHEM-NE",								// Data name of animation.
	0,											// Maximum dimension of animation.
	9,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);

const Grenade: AnimTypeClass = AnimTypeClass::new(
	ANIM_GRENADE,							// Animation number.
	"VEH-HIT2",								// Data name of animation.
	21,										// Maximum dimension of animation.
	1,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	true,										// Forms a crater?
	false,									// Sticks to unit in square?
	true,										// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	Some(VOC_GUN20),								// Sound effect to play.
	None
);

const FBall1: AnimTypeClass = AnimTypeClass::new(
	ANIM_FBALL1,							// Animation number.
	"FBALL1",								// Data name of animation.
	67,										// Maximum dimension of animation.
	6,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	true,										// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	Some(VOC_XPLOS),								// Sound effect to play.
	None
);

const Frag1: AnimTypeClass = AnimTypeClass::new(
	ANIM_FRAG1,								// Animation number.
	"FRAG1",									// Data name of animation.
	45,										// Maximum dimension of animation.
	3,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	true,										// Forms a crater?
	true,										// Sticks to unit in square?
	true,										// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	Some(VOC_XPLOBIG4),							// Sound effect to play.
	None
);

const Frag3: AnimTypeClass = AnimTypeClass::new(
	ANIM_FRAG2,								// Animation number.
	"FRAG3",									// Data name of animation.
	41,										// Maximum dimension of animation.
	3,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	true,										// Forms a crater?
	false,									// Sticks to unit in square?
	true,										// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	Some(VOC_XPLOBIG6),							// Sound effect to play.
	None
);

const VehHit1: AnimTypeClass = AnimTypeClass::new(
	ANIM_VEH_HIT1,							// Animation number.
	"VEH-HIT1",								// Data name of animation.
	30,										// Maximum dimension of animation.
	4,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	true,										// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	Some(VOC_XPLOS),								// Sound effect to play.
	None
);

const VehHit2: AnimTypeClass = AnimTypeClass::new(
	ANIM_VEH_HIT2,							// Animation number.
	"VEH-HIT2",								// Data name of animation.
	21,										// Maximum dimension of animation.
	1,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	true,										// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	Some(VOC_XPLOS),								// Sound effect to play.
	None
);

const VehHit3: AnimTypeClass = AnimTypeClass::new(
	ANIM_VEH_HIT3,							// Animation number.
	"VEH-HIT3",								// Data name of animation.
	19,										// Maximum dimension of animation.
	3,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	true,										// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	Some(VOC_XPLOS),								// Sound effect to play.
	None
);

const ArtExp1: AnimTypeClass = AnimTypeClass::new(
	ANIM_ART_EXP1,							// Animation number.
	"ART-EXP1",								// Data name of animation.
	41,										// Maximum dimension of animation.
	1,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	true,										// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	Some(VOC_XPLOSML2),							// Sound effect to play.
	None
);

const Napalm1: AnimTypeClass = AnimTypeClass::new(
	ANIM_NAPALM1,							// Animation number.
	"NAPALM1",								// Data name of animation.
	21,										// Maximum dimension of animation.
	5,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	true,										// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	Some(VOC_FLAMER1),							// Sound effect to play.
	None
);

const Napalm2: AnimTypeClass = AnimTypeClass::new(
	ANIM_NAPALM2,							// Animation number.
	"NAPALM2",								// Data name of animation.
	41,										// Maximum dimension of animation.
	5,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	true,										// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	Some(VOC_FLAMER1),							// Sound effect to play.
	None
);

const Napalm3: AnimTypeClass = AnimTypeClass::new(
	ANIM_NAPALM3,							// Animation number.
	"NAPALM3",								// Data name of animation.
	78,										// Maximum dimension of animation.
	5,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	true,										// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	Some(VOC_FLAMER1),							// Sound effect to play.
	None
);

const SmokePuff: AnimTypeClass = AnimTypeClass::new(
	ANIM_SMOKE_PUFF,						// Animation number.
	"SMOKEY",								// Data name of animation.
	24,										// Maximum dimension of animation.
	2,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	true,										// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);

const Piff: AnimTypeClass = AnimTypeClass::new(
	ANIM_PIFF,								// Animation number.
	"PIFF",									// Data name of animation.
	13,										// Maximum dimension of animation.
	1,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);

const PiffPiff: AnimTypeClass = AnimTypeClass::new(
	ANIM_PIFFPIFF,							// Animation number.
	"PIFFPIFF",								// Data name of animation.
	20,										// Maximum dimension of animation.
	2,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);

const Fire3: AnimTypeClass = AnimTypeClass::new(
	ANIM_FIRE_SMALL,						// Animation number.
	"FIRE3",									// Data name of animation.
	23,										// Maximum dimension of animation.
	0,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	true,										// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0008,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	2,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);

const Fire1: AnimTypeClass = AnimTypeClass::new(
	ANIM_FIRE_MED2,		 				// Animation number.
	"FIRE1",									// Data name of animation.
	23,										// Maximum dimension of animation.
	0,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	true,										// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	true,										// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0010,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	3,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);

const Fire4: AnimTypeClass = AnimTypeClass::new(
	ANIM_FIRE_TINY,		 				// Animation number.
	"FIRE4",									// Data name of animation.
	7,											// Maximum dimension of animation.
	0,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	true,										// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0008,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	3,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);

const Fire2: AnimTypeClass = AnimTypeClass::new(
	ANIM_FIRE_MED,							// Animation number.
	"FIRE2",									// Data name of animation.
	23,										// Maximum dimension of animation.
	0,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	true,										// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	true,										// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0010,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	3,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);

const OilFieldBurn: AnimTypeClass = AnimTypeClass::new(
	ANIM_OILFIELD_BURN,					// Animation number.
	"FLMSPT",								// Data name of animation.
	42,										// Maximum dimension of animation.
	58,										// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	true,										// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	33,										// Loop start frame number.
	99,										// Ending frame of loop back.
	66,										// Number of animation stages.
	65535,									// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);

const Gunfire: AnimTypeClass = AnimTypeClass::new(
	ANIM_MUZZLE_FLASH,					// Animation number.
	"GUNFIRE",								// Data name of animation.
	16,										// Maximum dimension of animation.
	0,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	true,										// Ground level animation?
	true,										// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	0,											// Number of times the animation loops.
//	2,											// Number of times the animation loops.
	1,											// Number of animation stages.
//	2,											// Number of animation stages.
	1,											// Ending frame of loop back.
	None,								// Sound effect to play.
	None
);

// #ifdef NEVER
// const E1RotFire: AnimTypeClass = AnimTypeClass::new(
// 	ANIM_E1_ROT_FIRE,						// Animation number.
// 	"E1ROT",									// Data name of animation.
// 	false,									// Normalized animation rate?
// 	false,									// Uses white translucent table?
// 	false,									// Scorches the ground?
// 	false,									// Forms a crater?
// 	false,									// Sticks to unit in square?
// 	true,										// Is a flat on the ground animation?
// 	true,										// Ground level animation?
// 	false,									// Translucent colors in this animation?
// 	false,									// Is this a flame thrower animation?
// 	0x0000,									// Damage to apply per tick (fixed point).
// 	30,										// Delay between frames.
// 	28,										// Starting frame number.
// 	0,											// Loop start frame number.
// 	0,											// Ending frame of loop back.
// 	4,											// Number of animation stages.
// 	1,											// Number of times the animation loops.
// 	None,								// Sound effect to play.
// 	None
// );
// const E1RotGrenade: AnimTypeClass = AnimTypeClass::new(
// 	ANIM_E1_ROT_GRENADE,					// Animation number.
// 	"E1ROT",									// Data name of animation.
// 	false,									// Normalized animation rate?
// 	false,									// Uses white translucent table?
// 	false,									// Scorches the ground?
// 	false,									// Forms a crater?
// 	false,									// Sticks to unit in square?
// 	true,										// Is a flat on the ground animation?
// 	true,										// Ground level animation?
// 	false,									// Translucent colors in this animation?
// 	false,									// Is this a flame thrower animation?
// 	0x0000,									// Damage to apply per tick (fixed point).
// 	30,										// Delay between frames.
// 	24,										// Starting frame number.
// 	0,											// Loop start frame number.
// 	0,											// Loopback frame number.
// 	4,											// Number of animation stages.
// 	1,											// Number of times the animation loops.
// 	None,								// Sound effect to play.
// 	None
// );
// const E1RotGun: AnimTypeClass = AnimTypeClass::new(
// 	ANIM_E1_ROT_GUN,						// Animation number.
// 	"E1ROT",									// Data name of animation.
// 	false,									// Normalized animation rate?
// 	false,									// Uses white translucent table?
// 	false,									// Scorches the ground?
// 	false,									// Forms a crater?
// 	false,									// Sticks to unit in square?
// 	true,										// Is a flat on the ground animation?
// 	true,										// Ground level animation?
// 	false,									// Translucent colors in this animation?
// 	false,									// Is this a flame thrower animation?
// 	0x0000,									// Damage to apply per tick (fixed point).
// 	30,										// Delay between frames.
// 	16,										// Starting frame number.
// 	0,											// Loop start frame number.
// 	0,											// Loopback frame number.
// 	4,											// Number of animation stages.
// 	1,											// Number of times the animation loops.
// 	None,								// Sound effect to play.
// 	None
// );
// const E1RotExp: AnimTypeClass = AnimTypeClass::new(
// 	ANIM_E1_ROT_EXP,						// Animation number.
// 	"E1ROT",									// Data name of animation.
// 	false,									// Normalized animation rate?
// 	false,									// Uses white translucent table?
// 	false,									// Scorches the ground?
// 	false,									// Forms a crater?
// 	false,									// Sticks to unit in square?
// 	true,										// Is a flat on the ground animation?
// 	true,										// Ground level animation?
// 	false,									// Translucent colors in this animation?
// 	false,									// Is this a flame thrower animation?
// 	0x0000,									// Damage to apply per tick (fixed point).
// 	30,										// Delay between frames.
// 	20,										// Starting frame number.
// 	0,											// Loop start frame number.
// 	0,											// Loopback frame number.
// 	4,											// Number of animation stages.
// 	1,											// Number of times the animation loops.
// 	None,								// Sound effect to play.
// 	None
// );

// const E2RotFire: AnimTypeClass = AnimTypeClass::new(
// 	ANIM_E2_ROT_FIRE,						// Animation number.
// 	"E2ROT",									// Data name of animation.
// 	false,									// Normalized animation rate?
// 	false,									// Uses white translucent table?
// 	false,									// Scorches the ground?
// 	false,									// Forms a crater?
// 	false,									// Sticks to unit in square?
// 	true,										// Is a flat on the ground animation?
// 	true,										// Ground level animation?
// 	false,									// Translucent colors in this animation?
// 	false,									// Is this a flame thrower animation?
// 	0x0000,									// Damage to apply per tick (fixed point).
// 	30,										// Delay between frames.
// 	28,										// Starting frame number.
// 	0,											// Loop start frame number.
// 	0,											// Loopback frame number.
// 	4,											// Number of animation stages.
// 	1,											// Number of times the animation loops.
// 	None,								// Sound effect to play.
// 	None
// );
// const E2RotGrenade: AnimTypeClass = AnimTypeClass::new(
// 	ANIM_E2_ROT_GRENADE,					// Animation number.
// 	"E2ROT",									// Data name of animation.
// 	false,									// Normalized animation rate?
// 	false,									// Uses white translucent table?
// 	false,									// Scorches the ground?
// 	false,									// Forms a crater?
// 	false,									// Sticks to unit in square?
// 	true,										// Is a flat on the ground animation?
// 	true,										// Ground level animation?
// 	false,									// Translucent colors in this animation?
// 	false,									// Is this a flame thrower animation?
// 	0x0000,									// Damage to apply per tick (fixed point).
// 	30,										// Delay between frames.
// 	24,										// Starting frame number.
// 	0,											// Loop start frame number.
// 	0,											// Loopback frame number.
// 	4,											// Number of animation stages.
// 	1,											// Number of times the animation loops.
// 	None,								// Sound effect to play.
// 	None
// );
// const E2RotGun: AnimTypeClass = AnimTypeClass::new(
// 	ANIM_E2_ROT_GUN,						// Animation number.
// 	"E2ROT",									// Data name of animation.
// 	false,									// Normalized animation rate?
// 	false,									// Uses white translucent table?
// 	false,									// Scorches the ground?
// 	false,									// Forms a crater?
// 	false,									// Sticks to unit in square?
// 	true,										// Is a flat on the ground animation?
// 	true,										// Ground level animation?
// 	false,									// Translucent colors in this animation?
// 	false,									// Is this a flame thrower animation?
// 	0x0000,									// Damage to apply per tick (fixed point).
// 	30,										// Delay between frames.
// 	16,										// Starting frame number.
// 	0,											// Loop start frame number.
// 	0,											// Loopback frame number.
// 	4,											// Number of animation stages.
// 	1,											// Number of times the animation loops.
// 	None,								// Sound effect to play.
// 	None
// );
// const E2RotExp: AnimTypeClass = AnimTypeClass::new(
// 	ANIM_E2_ROT_EXP,						// Animation number.
// 	"E2ROT",									// Data name of animation.
// 	false,									// Normalized animation rate?
// 	false,									// Uses white translucent table?
// 	false,									// Scorches the ground?
// 	false,									// Forms a crater?
// 	false,									// Sticks to unit in square?
// 	true,										// Is a flat on the ground animation?
// 	true,										// Ground level animation?
// 	false,									// Translucent colors in this animation?
// 	false,									// Is this a flame thrower animation?
// 	0x0000,									// Damage to apply per tick (fixed point).
// 	30,										// Delay between frames.
// 	20,										// Starting frame number.
// 	0,											// Loop start frame number.
// 	0,											// Loopback frame number.
// 	4,											// Number of animation stages.
// 	1,											// Number of times the animation loops.
// 	None,								// Sound effect to play.
// 	None
// );

// const E3RotFire: AnimTypeClass = AnimTypeClass::new(
// 	ANIM_E3_ROT_FIRE,						// Animation number.
// 	"E3ROT",									// Data name of animation.
// 	false,									// Normalized animation rate?
// 	false,									// Uses white translucent table?
// 	false,									// Scorches the ground?
// 	false,									// Forms a crater?
// 	false,									// Sticks to unit in square?
// 	true,										// Is a flat on the ground animation?
// 	true,										// Ground level animation?
// 	false,									// Translucent colors in this animation?
// 	false,									// Is this a flame thrower animation?
// 	0x0000,									// Damage to apply per tick (fixed point).
// 	30,										// Delay between frames.
// 	28,										// Starting frame number.
// 	0,											// Loop start frame number.
// 	0,											// Loopback frame number.
// 	4,											// Number of animation stages.
// 	1,											// Number of times the animation loops.
// 	None,								// Sound effect to play.
// 	None
// );
// const E3RotGrenade: AnimTypeClass = AnimTypeClass::new(
// 	ANIM_E3_ROT_GRENADE,					// Animation number.
// 	"E3ROT",									// Data name of animation.
// 	false,									// Normalized animation rate?
// 	false,									// Uses white translucent table?
// 	false,									// Scorches the ground?
// 	false,									// Forms a crater?
// 	false,									// Sticks to unit in square?
// 	true,										// Is a flat on the ground animation?
// 	true,										// Ground level animation?
// 	false,									// Translucent colors in this animation?
// 	false,									// Is this a flame thrower animation?
// 	0x0000,									// Damage to apply per tick (fixed point).
// 	30,										// Delay between frames.
// 	24,										// Starting frame number.
// 	0,											// Loop start frame number.
// 	0,											// Loopback frame number.
// 	4,											// Number of animation stages.
// 	1,											// Number of times the animation loops.
// 	None,								// Sound effect to play.
// 	None
// );
// const E3RotGun: AnimTypeClass = AnimTypeClass::new(
// 	ANIM_E3_ROT_GUN,						// Animation number.
// 	"E3ROT",									// Data name of animation.
// 	false,									// Normalized animation rate?
// 	false,									// Uses white translucent table?
// 	false,									// Scorches the ground?
// 	false,									// Forms a crater?
// 	false,									// Sticks to unit in square?
// 	true,										// Is a flat on the ground animation?
// 	true,										// Ground level animation?
// 	false,									// Translucent colors in this animation?
// 	false,									// Is this a flame thrower animation?
// 	0x0000,									// Damage to apply per tick (fixed point).
// 	30,										// Delay between frames.
// 	16,										// Starting frame number.
// 	0,											// Loop start frame number.
// 	0,											// Loopback frame number.
// 	4,											// Number of animation stages.
// 	1,											// Number of times the animation loops.
// 	None,								// Sound effect to play.
// 	None
// );
// const E3RotExp: AnimTypeClass = AnimTypeClass::new(
// 	ANIM_E3_ROT_EXP,						// Animation number.
// 	"E3ROT",									// Data name of animation.
// 	false,									// Normalized animation rate?
// 	false,									// Uses white translucent table?
// 	false,									// Scorches the ground?
// 	false,									// Forms a crater?
// 	false,									// Sticks to unit in square?
// 	true,										// Is a flat on the ground animation?
// 	true,										// Ground level animation?
// 	false,									// Translucent colors in this animation?
// 	false,									// Is this a flame thrower animation?
// 	0x0000,									// Damage to apply per tick (fixed point).
// 	30,										// Delay between frames.
// 	20,										// Starting frame number.
// 	0,											// Loop start frame number.
// 	0,											// Loopback frame number.
// 	4,											// Number of animation stages.
// 	1,											// Number of times the animation loops.
// 	None,								// Sound effect to play.
// 	None
// );

// const E4RotFire: AnimTypeClass = AnimTypeClass::new(
// 	ANIM_E4_ROT_FIRE,						// Animation number.
// 	"E4ROT",									// Data name of animation.
// 	false,									// Normalized animation rate?
// 	false,									// Uses white translucent table?
// 	false,									// Scorches the ground?
// 	false,									// Forms a crater?
// 	false,									// Sticks to unit in square?
// 	true,										// Is a flat on the ground animation?
// 	true,										// Ground level animation?
// 	false,									// Translucent colors in this animation?
// 	false,									// Is this a flame thrower animation?
// 	0x0000,									// Damage to apply per tick (fixed point).
// 	30,										// Delay between frames.
// 	28,										// Starting frame number.
// 	0,											// Loop start frame number.
// 	0,											// Loopback frame number.
// 	4,											// Number of animation stages.
// 	1,											// Number of times the animation loops.
// 	None,								// Sound effect to play.
// 	None
// );
// const E4RotGrenade: AnimTypeClass = AnimTypeClass::new(
// 	ANIM_E4_ROT_GRENADE,					// Animation number.
// 	"E4ROT",									// Data name of animation.
// 	false,									// Normalized animation rate?
// 	false,									// Uses white translucent table?
// 	false,									// Scorches the ground?
// 	false,									// Forms a crater?
// 	false,									// Sticks to unit in square?
// 	true,										// Is a flat on the ground animation?
// 	true,										// Ground level animation?
// 	false,									// Translucent colors in this animation?
// 	false,									// Is this a flame thrower animation?
// 	0x0000,									// Damage to apply per tick (fixed point).
// 	30,										// Delay between frames.
// 	24,										// Starting frame number.
// 	0,											// Loop start frame number.
// 	0,											// Loopback frame number.
// 	4,											// Number of animation stages.
// 	1,											// Number of times the animation loops.
// 	None,								// Sound effect to play.
// 	None
// );
// const E4RotGun: AnimTypeClass = AnimTypeClass::new(
// 	ANIM_E4_ROT_GUN,						// Animation number.
// 	"E4ROT",									// Data name of animation.
// 	false,									// Normalized animation rate?
// 	false,									// Uses white translucent table?
// 	false,									// Scorches the ground?
// 	false,									// Forms a crater?
// 	false,									// Sticks to unit in square?
// 	true,										// Is a flat on the ground animation?
// 	true,										// Ground level animation?
// 	false,									// Translucent colors in this animation?
// 	false,									// Is this a flame thrower animation?
// 	0x0000,									// Damage to apply per tick (fixed point).
// 	30,										// Delay between frames.
// 	16,										// Starting frame number.
// 	0,											// Loop start frame number.
// 	0,											// Loopback frame number.
// 	4,											// Number of animation stages.
// 	1,											// Number of times the animation loops.
// 	None,								// Sound effect to play.
// 	None
// );
// const E4RotExp: AnimTypeClass = AnimTypeClass::new(
// 	ANIM_E4_ROT_EXP,						// Animation number.
// 	"E4ROT",									// Data name of animation.
// 	false,									// Normalized animation rate?
// 	false,									// Uses white translucent table?
// 	false,									// Scorches the ground?
// 	false,									// Forms a crater?
// 	false,									// Sticks to unit in square?
// 	true,										// Is a flat on the ground animation?
// 	true,										// Ground level animation?
// 	false,									// Translucent colors in this animation?
// 	false,									// Is this a flame thrower animation?
// 	0x0000,									// Damage to apply per tick (fixed point).
// 	30,										// Delay between frames.
// 	20,										// Starting frame number.
// 	0,											// Loop start frame number.
// 	0,											// Loopback frame number.
// 	4,											// Number of animation stages.
// 	1,											// Number of times the animation loops.
// 	None,								// Sound effect to play.
// 	None
// );
// #endif

const SmokeM: AnimTypeClass = AnimTypeClass::new(
	ANIM_SMOKE_M,							// Animation number.
	"SMOKE_M",								// Data name of animation.
	28,										// Maximum dimension of animation.
	30,										// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	true,										// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	67,										// Loop start frame number.
	-1,										// Loopback frame number.
	-1,										// Number of animation stages.
	6,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);

/*
**	Mini-gun fire effect -- used by guard towers.
*/
const GUNN: AnimTypeClass = AnimTypeClass::new(
	ANIM_GUN_N,								// Animation number.
	"MINIGUN",								// Data name of animation.
	18,										// Maximum dimension of animation.
	0,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	0,											// Number of times the animation loops.
	6,											// Number of animation stages.
	0,											// Ending frame of loop back.
	None,								// Sound effect to play.
	None
);
const GUNNW: AnimTypeClass = AnimTypeClass::new(
	ANIM_GUN_NW,							// Animation number.
	"MINIGUN",								// Data name of animation.
	18,										// Maximum dimension of animation.
	0,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	6,											// Starting frame number.
	0,											// Loop start frame number.
	0,											// Number of times the animation loops.
	6,											// Number of animation stages.
	0,											// Ending frame of loop back.
	None,								// Sound effect to play.
	None
);
const GUNW: AnimTypeClass = AnimTypeClass::new(
	ANIM_GUN_W,								// Animation number.
	"MINIGUN",								// Data name of animation.
	18,										// Maximum dimension of animation.
	0,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	12,										// Starting frame number.
	0,											// Loop start frame number.
	0,											// Number of times the animation loops.
	6,											// Number of animation stages.
	0,											// Ending frame of loop back.
	None,								// Sound effect to play.
	None
);
const GUNSW: AnimTypeClass = AnimTypeClass::new(
	ANIM_GUN_SW,							// Animation number.
	"MINIGUN",								// Data name of animation.
	18,										// Maximum dimension of animation.
	0,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	18,										// Starting frame number.
	0,											// Loop start frame number.
	0,											// Number of times the animation loops.
	6,											// Number of animation stages.
	0,											// Ending frame of loop back.
	None,								// Sound effect to play.
	None
);
const GUNS: AnimTypeClass = AnimTypeClass::new(
	ANIM_GUN_S,								// Animation number.
	"MINIGUN",								// Data name of animation.
	18,										// Maximum dimension of animation.
	0,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	24,										// Starting frame number.
	0,											// Loop start frame number.
	0,											// Number of times the animation loops.
	6,											// Number of animation stages.
	0,											// Ending frame of loop back.
	None,								// Sound effect to play.
	None
);
const GUNSE: AnimTypeClass = AnimTypeClass::new(
	ANIM_GUN_SE,							// Animation number.
	"MINIGUN",								// Data name of animation.
	18,										// Maximum dimension of animation.
	0,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	30,										// Starting frame number.
	0,											// Loop start frame number.
	0,											// Number of times the animation loops.
	6,											// Number of animation stages.
	0,											// Ending frame of loop back.
	None,								// Sound effect to play.
	None
);
const GUNE: AnimTypeClass = AnimTypeClass::new(
	ANIM_GUN_E,								// Animation number.
	"MINIGUN",								// Data name of animation.
	18,										// Maximum dimension of animation.
	0,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	36,										// Starting frame number.
	0,											// Loop start frame number.
	0,											// Number of times the animation loops.
	6,											// Number of animation stages.
	0,											// Ending frame of loop back.
	None,								// Sound effect to play.
	None
);
const GUNNE: AnimTypeClass = AnimTypeClass::new(
	ANIM_GUN_NE,							// Animation number.
	"MINIGUN",								// Data name of animation.
	18,										// Maximum dimension of animation.
	0,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	42,										// Starting frame number.
	0,											// Loop start frame number.
	0,											// Number of times the animation loops.
	6,											// Number of animation stages.
	0,											// Ending frame of loop back.
	None,								// Sound effect to play.
	None
);
const IonCannon: AnimTypeClass = AnimTypeClass::new(
	ANIM_ION_CANNON,						// Animation number.
	"IONSFX",								// Data name of animation.
	48,										// Maximum dimension of animation.
	11,										// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	true,										// Scorches the ground?
	true,										// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	15,										// Number of animation stages.
	0,											// Number of times the animation loops.
	Some(VOC_ION_CANNON),						// Sound effect to play.
	Some(ANIM_ART_EXP1)
);

const AtomBomb: AnimTypeClass = AnimTypeClass::new(
	ANIM_ATOM_BLAST,						// Animation number.
	"ATOMSFX",								// Data name of animation.
	72,										// Maximum dimension of animation.
	19,										// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	true,										// Scorches the ground?
	true,										// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	-1,										// Number of animation stages.
	0,											// Number of times the animation loops.
	Some(VOC_NUKE_EXPLODE),						// Sound effect to play.
	None
);
const AtomDoor: AnimTypeClass = AnimTypeClass::new(
	ANIM_ATOM_DOOR,						// Animation number.
	"ATOMDOOR",								// Data name of animation.
	48,										// Maximum dimension of animation.
	19,										// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	true,										// Scorches the ground?
	true,										// Forms a crater?
	true,										// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	-1,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None
);

const CDeviator: AnimTypeClass = AnimTypeClass::new(
	ANIM_CRATE_DEVIATOR,					// Animation number.
	"DEVIATOR",								// Data name of animation.
	48,										// Maximum dimension of animation.
	0,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	2,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	-1,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None								// Follow up animation.
);
const CDollar: AnimTypeClass = AnimTypeClass::new(
	ANIM_CRATE_DOLLAR,					// Animation number.
	"DOLLAR",								// Data name of animation.
	48,										// Maximum dimension of animation.
	0,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	2,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	-1,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None								// Follow up animation.
);
const CEarth: AnimTypeClass = AnimTypeClass::new(
	ANIM_CRATE_EARTH,						// Animation number.
	"EARTH",									// Data name of animation.
	48,										// Maximum dimension of animation.
	0,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	2,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	-1,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None								// Follow up animation.
);
const CEmpulse: AnimTypeClass = AnimTypeClass::new(
	ANIM_CRATE_EMPULSE,					// Animation number.
	"EMPULSE",								// Data name of animation.
	48,										// Maximum dimension of animation.
	0,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	2,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	-1,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None								// Follow up animation.
);
const CInvun: AnimTypeClass = AnimTypeClass::new(
	ANIM_CRATE_INVUN,						// Animation number.
	"INVUN",									// Data name of animation.
	48,										// Maximum dimension of animation.
	0,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	2,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	-1,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None								// Follow up animation.
);
const CMine: AnimTypeClass = AnimTypeClass::new(
	ANIM_CRATE_MINE,						// Animation number.
	"MINE",									// Data name of animation.
	48,										// Maximum dimension of animation.
	0,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	2,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	-1,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None								// Follow up animation.
);
const CRapid: AnimTypeClass = AnimTypeClass::new(
	ANIM_CRATE_RAPID,						// Animation number.
	"RAPID",									// Data name of animation.
	48,										// Maximum dimension of animation.
	0,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	2,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	-1,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None								// Follow up animation.
);
const CStealth: AnimTypeClass = AnimTypeClass::new(
	ANIM_CRATE_STEALTH,					// Animation number.
	"STEALTH2",								// Data name of animation.
	48,										// Maximum dimension of animation.
	0,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	2,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	-1,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None								// Follow up animation.
);
const CMissile: AnimTypeClass = AnimTypeClass::new(
	ANIM_CRATE_MISSILE,					// Animation number.
	"MISSILE2",								// Data name of animation.
	48,										// Maximum dimension of animation.
	0,											// Biggest animation stage.
	true,										// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	2,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	-1,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None								// Follow up animation.
);

const MoveFlash: AnimTypeClass = AnimTypeClass::new(
	ANIM_MOVE_FLASH,							// Animation number.
	"MOVEFLSH",								// Data name of animation.
	24,										// Maximum dimension of animation.
	0,											// Biggest animation stage.
	true,										// Normalized animation rate?
	true,										// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	true,										// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	0,											// Ending frame of loop back.
	-1,										// Number of animation stages.
	0,											// Number of times the animation loops.
	None,								// Sound effect to play.
	None								// Follow up animation.
);

const ChemBall: AnimTypeClass = AnimTypeClass::new(
	ANIM_CHEM_BALL,						// Animation number.
	"CHEMBALL",								// Data name of animation.
	21,										// Maximum dimension of animation.
	5,											// Biggest animation stage.
	false,									// Normalized animation rate?
	false,									// Uses white translucent table?
	false,									// Scorches the ground?
	false,									// Forms a crater?
	false,									// Sticks to unit in square?
	false,									// Ground level animation?
	false,									// Translucent colors in this animation?
	false,									// Is this a flame thrower animation?
	0x0000,									// Damage to apply per tick (fixed point).
	1,											// Delay between frames.
	0,											// Starting frame number.
	0,											// Loop start frame number.
	-1,										// Ending frame of loop back.
	-1,										// Number of animation stages.
	1,											// Number of times the animation loops.
	Some(VOC_FLAMER1),							// Sound effect to play.
	None
);
