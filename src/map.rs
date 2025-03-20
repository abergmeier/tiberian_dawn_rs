#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_variables)]

pub const MCW: u16 = MAP_CELL_W as u16;


/**********************************************************************
**	Map controls. The map is composed of square elements called 'cells'.
**	All larger elements are build upon these.
*/

// Size of the map in cells. The width of the map must be a power
//	of two. This is accomplished by setting the width by the number of
//	bits it occupies. The number of meta-cells will be a subset of the
//	cell width.
pub const MAP_CELL_MAX_X_BITS: u8 = 6;
pub const MAP_CELL_MAX_Y_BITS: u8 = 6;
pub const MAP_CELL_X_MASK: u16 = !(!0 << MAP_CELL_MAX_X_BITS);
//#define	MAP_CELL_Y_MASK			((~(~0 << MAP_CELL_MAX_Y_BITS)) << MAP_CELL_MAX_Y_BITS)

// Size of the map in cells.
pub const MAP_CELL_W: u8 = 1<<MAP_CELL_MAX_X_BITS;
pub const MAP_CELL_H: u8 = 1<<MAP_CELL_MAX_Y_BITS;
pub const MAP_CELL_TOTAL: u16 = MAP_CELL_W as u16 * MAP_CELL_H as u16;
