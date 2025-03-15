#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables
)]
/// Units that move can move at different speeds. These enumerate the
/// different speeds that a unit can move.
#[repr(u8)]
pub enum MPHType{
	MPH_IMMOBILE=0,
	MPH_VERY_SLOW=5,
	MPH_KINDA_SLOW=6,
	MPH_SLOW=8,
	MPH_SLOW_ISH=10,
	MPH_MEDIUM_SLOW=12,
	MPH_MEDIUM=18,
	MPH_MEDIUM_FAST=30,
	MPH_MEDIUM_FASTER=35,
	MPH_FAST=40,
	MPH_ROCKET=60,
	MPH_VERY_FAST=100,
	MPH_LIGHT_SPEED=255
}

/// Each vehicle is give a speed rating. This is a combination of not only
/// its physical speed, but the means by which it travels (wheels, tracks,
/// wings, etc). This is used to determine the movement table.
#[repr(u8)]
pub enum SpeedType {
	//SPEED_NONE=-1,

	SPEED_FOOT=0,					// Bipedal.
	SPEED_TRACK,				// Tracked locomotion.
	SPEED_HARVESTER,			// Harvester speed rating.
	SPEED_WHEEL,				// Balloon tires.
	SPEED_WINGED,				// Lifter's, 'thopters, and rockets.
	SPEED_HOVER,				// Typical hovercraft logic.
	SPEED_FLOAT,				// Ships.

	//SPEED_COUNT,
	//SPEED_FIRST=SPEED_FOOT
}
