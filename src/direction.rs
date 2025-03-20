#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_variables)]

pub type DirType = u8;

pub const DIR_MIN: u8 = 0;
pub const DIR_N: u8 = 0;
pub const DIR_NE: u8 = 1<<5;
pub const DIR_E: u8 = 2<<5;
pub const DIR_SE: u8 = 3<<5;
pub const DIR_S: u8 = 4<<5;
pub const DIR_SW: u8 = 5<<5;
pub const DIR_SW_X1: u8 = (5<<5)-8;		// Direction of harvester while unloading.
pub const DIR_SW_X2: u8 = (5<<5)-16;		// Direction of harvester while unloading.
pub const DIR_W: u8 = 6<<5;
pub const DIR_NW: u8 = 7<<5;
pub const DIR_MAX: u8 = 254;
