#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables
)]

use crate::armor::ArmorType::*;
use crate::coords::XYP_COORD;
use crate::display::CELL_PIXEL_H;
use crate::display::CELL_PIXEL_W;
use crate::display::ICON_LEPTON_H;
use crate::display::ICON_LEPTON_W;
use crate::map::MAP_CELL_W;
use crate::terrain::{TerrainType::*, TerrainTypeClass};
use crate::text::IDs::*;
use crate::theater::THEATERF;
use crate::REFRESH_EOL;

const TREE_NORMAL: u16 = 600;
const TREE_WEAK: u16 = 400;
const TREE_STRONG: u16 = 800;

const _List1: [u16; 2] = [0, REFRESH_EOL];
const _List000010: [u16; 2] = [MAP_CELL_W as u16 + 1, REFRESH_EOL];
const _List000011101000: [u16; 5] = [
    MAP_CELL_W as u16,
    MAP_CELL_W as u16 + 1,
    MAP_CELL_W as u16 + 2,
    MAP_CELL_W as u16 * 2,
    REFRESH_EOL,
];
const _List00001: [u16; 2] = [4, REFRESH_EOL];
const _List000110: [u16; 3] = [MAP_CELL_W as u16, MAP_CELL_W as u16 + 1, REFRESH_EOL];
const _List000111: [u16; 4] = [
    MAP_CELL_W as u16,
    MAP_CELL_W as u16 + 1,
    MAP_CELL_W as u16 + 2,
    REFRESH_EOL,
];
const _List001011100110: [u16; 7] = [
    2,
    MAP_CELL_W as u16,
    MAP_CELL_W as u16 + 1,
    MAP_CELL_W as u16 + 2,
    MAP_CELL_W as u16 * 2 + 1,
    MAP_CELL_W as u16 * 2 + 2,
    REFRESH_EOL,
];
const _List0010: [u16; 2] = [MAP_CELL_W as u16, REFRESH_EOL];
const _List0011: [u16; 3] = [MAP_CELL_W as u16, MAP_CELL_W as u16 + 1, REFRESH_EOL];
const _List001: [u16; 2] = [2, REFRESH_EOL];
const _List010110: [u16; 4] = [1, MAP_CELL_W as u16, MAP_CELL_W as u16 + 1, REFRESH_EOL];
const _List01: [u16; 2] = [1, REFRESH_EOL];
const _List1001: [u16; 3] = [0, MAP_CELL_W as u16 + 1, REFRESH_EOL];
const _List101001: [u16; 4] = [0, 2, MAP_CELL_W as u16 + 2, REFRESH_EOL];
const _List10: [u16; 2] = [0, REFRESH_EOL];
const _List110000011001: [u16; 6] = [
    0,
    1,
    MAP_CELL_W as u16 + 3,
    MAP_CELL_W as u16 * 2,
    MAP_CELL_W as u16 * 2 + 3,
    REFRESH_EOL,
];
const _List110000: [u16; 3] = [0, 1, REFRESH_EOL];
const _List110001: [u16; 4] = [0, 1, MAP_CELL_W as u16 + 2, REFRESH_EOL];
const _List1100: [u16; 3] = [0, 1, REFRESH_EOL];
const _List110110: [u16; 5] = [0, 1, MAP_CELL_W as u16, MAP_CELL_W as u16 + 1, REFRESH_EOL];
const _List1101: [u16; 4] = [0, 1, MAP_CELL_W as u16 + 1, REFRESH_EOL];
const _List111000010110: [u16; 7] = [
    0,
    1,
    2,
    MAP_CELL_W as u16 + 3,
    MAP_CELL_W as u16 * 2 + 1,
    MAP_CELL_W as u16 * 2 + 2,
    REFRESH_EOL,
];
const _List111001: [u16; 5] = [0, 1, 2, MAP_CELL_W as u16 + 2, REFRESH_EOL];
const _List111101: [u16; 6] = [
    0,
    1,
    2,
    MAP_CELL_W as u16,
    MAP_CELL_W as u16 + 2,
    REFRESH_EOL,
];
const _List11110: [u16; 5] = [0, 1, 2, 3, REFRESH_EOL];

const Tree1Class: TerrainTypeClass<2, 3> = TerrainTypeClass::new(
    TERRAIN_TREE1,
    THEATERF::WINTER.bits() | THEATERF::TEMPERATE.bits(),
    XYP_COORD!(11, 41), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    true,               // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    true,               // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    false,              // Is it immune to normal combat damage?
    "T01",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List0010,
    _List1001,
);

const Tree2Class: TerrainTypeClass<2, 3> = TerrainTypeClass::new(
    TERRAIN_TREE2,
    THEATERF::WINTER.bits() | THEATERF::TEMPERATE.bits(),
    XYP_COORD!(11, 44), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    true,               // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    true,               // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    false,              // Is it immune to normal combat damage?
    "T02",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List0010,
    _List1001,
);

const Tree3Class: TerrainTypeClass<2, 3> = TerrainTypeClass::new(
    TERRAIN_TREE3,
    THEATERF::WINTER.bits() | THEATERF::TEMPERATE.bits(),
    XYP_COORD!(12, 45), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    true,               // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    true,               // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    false,              // Is it immune to normal combat damage?
    "T03",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List0010,
    _List1001,
);

const Tree4Class: TerrainTypeClass<2, 0> = TerrainTypeClass::new(
    TERRAIN_TREE4,
    THEATERF::DESERT.bits(),
    XYP_COORD!(8, 9), // Center base coordinate offset.
    false,            // Spawns Tiberium spontaneously?
    true,             // Does it have destruction animation?
    false,            // Does it have transformation (blossom tree) anim?
    true,             // Does it catch fire?
    false,            // Is this object crushable by heavy vehicles?
    false,            // Can this object be selected by the player?
    false,            // Can it be the target of a move or attack order?
    true,             // Don't make a big deal about it if it gets destroyed?
    false,            // Is it immune to normal combat damage?
    "T04",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List1,
    [],
);

const Tree5Class: TerrainTypeClass<2, 3> = TerrainTypeClass::new(
    TERRAIN_TREE5,
    THEATERF::WINTER.bits() | THEATERF::TEMPERATE.bits(),
    XYP_COORD!(15, 41), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    true,               // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    true,               // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    false,              // Is it immune to normal combat damage?
    "T05",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List0010,
    _List1001,
);

const Tree6Class: TerrainTypeClass<2, 3> = TerrainTypeClass::new(
    TERRAIN_TREE6,
    THEATERF::WINTER.bits() | THEATERF::TEMPERATE.bits(),
    XYP_COORD!(16, 37), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    true,               // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    true,               // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    false,              // Is it immune to normal combat damage?
    "T06",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List0010,
    _List1001,
);

const Tree7Class: TerrainTypeClass<2, 3> = TerrainTypeClass::new(
    TERRAIN_TREE7,
    THEATERF::WINTER.bits() | THEATERF::TEMPERATE.bits(),
    XYP_COORD!(15, 41), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    true,               // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    true,               // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    false,              // Is it immune to normal combat damage?
    "T07",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List0010,
    _List1001,
);

const Tree8Class: TerrainTypeClass<2, 2> = TerrainTypeClass::new(
    TERRAIN_TREE8,
    THEATERF::WINTER.bits() | THEATERF::TEMPERATE.bits() | THEATERF::DESERT.bits(),
    XYP_COORD!(14, 22), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    true,               // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    true,               // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    false,              // Is it immune to normal combat damage?
    "T08",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List10,
    _List01,
);

const Tree9Class: TerrainTypeClass<2, 2> = TerrainTypeClass::new(
    TERRAIN_TREE9,
    THEATERF::DESERT.bits(),
    XYP_COORD!(11, 22), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    true,               // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    true,               // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    false,              // Is it immune to normal combat damage?
    "T09",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List10,
    _List01,
);

const Tree10Class: TerrainTypeClass<3, 3> = TerrainTypeClass::new(
    TERRAIN_TREE10,
    THEATERF::WINTER.bits() | THEATERF::TEMPERATE.bits(),
    XYP_COORD!(25, 43), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    true,               // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    true,               // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    false,              // Is it immune to normal combat damage?
    "T10",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List0011,
    _List1100,
);

const Tree11Class: TerrainTypeClass<3, 3> = TerrainTypeClass::new(
    TERRAIN_TREE11,
    THEATERF::WINTER.bits() | THEATERF::TEMPERATE.bits(),
    XYP_COORD!(23, 44), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    true,               // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    true,               // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    false,              // Is it immune to normal combat damage?
    "T11",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List0011,
    _List1100,
);

const Tree12Class: TerrainTypeClass<2, 3> = TerrainTypeClass::new(
    TERRAIN_TREE12,
    THEATERF::WINTER.bits() | THEATERF::TEMPERATE.bits(),
    XYP_COORD!(14, 36), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    true,               // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    true,               // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    false,              // Is it immune to normal combat damage?
    "T12",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List0010,
    _List1001,
);

const Tree13Class: TerrainTypeClass<2, 4> = TerrainTypeClass::new(
    TERRAIN_TREE13,
    THEATERF::WINTER.bits() | THEATERF::TEMPERATE.bits(),
    XYP_COORD!(19, 40), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    true,               // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    true,               // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    false,              // Is it immune to normal combat damage?
    "T13",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List0010,
    _List1101,
);

const Tree14Class: TerrainTypeClass<3, 3> = TerrainTypeClass::new(
    TERRAIN_TREE14,
    THEATERF::WINTER.bits() | THEATERF::TEMPERATE.bits(),
    XYP_COORD!(19, 40), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    true,               // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    true,               // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    false,              // Is it immune to normal combat damage?
    "T14",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List0011,
    _List1100,
);

const Tree15Class: TerrainTypeClass<3, 3> = TerrainTypeClass::new(
    TERRAIN_TREE15,
    THEATERF::WINTER.bits() | THEATERF::TEMPERATE.bits(),
    XYP_COORD!(19, 40), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    true,               // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    true,               // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    false,              // Is it immune to normal combat damage?
    "T15",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List0011,
    _List1100,
);

const Tree16Class: TerrainTypeClass<2, 3> = TerrainTypeClass::new(
    TERRAIN_TREE16,
    THEATERF::WINTER.bits() | THEATERF::TEMPERATE.bits(),
    XYP_COORD!(13, 36), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    true,               // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    true,               // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    false,              // Is it immune to normal combat damage?
    "T16",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List0010,
    _List1001,
);

const Tree17Class: TerrainTypeClass<2, 3> = TerrainTypeClass::new(
    TERRAIN_TREE17,
    THEATERF::WINTER.bits() | THEATERF::TEMPERATE.bits(),
    XYP_COORD!(18, 44), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    true,               // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    true,               // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    false,              // Is it immune to normal combat damage?
    "T17",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List0010,
    _List1001,
);

const Tree18Class: TerrainTypeClass<2, 6> = TerrainTypeClass::new(
    TERRAIN_TREE18,
    THEATERF::DESERT.bits(),
    XYP_COORD!(33, 40), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    true,               // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    true,               // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    false,              // Is it immune to normal combat damage?
    "T18",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List000010,
    _List111101,
);

const Split1Class: TerrainTypeClass<2, 4> = TerrainTypeClass::new(
    TERRAIN_BLOSSOMTREE1,
    THEATERF::TEMPERATE.bits() | THEATERF::WINTER.bits(),
    XYP_COORD!(18, 44), // Center base coordinate offset.
    true,               // Spawns Tiberium spontaneously?
    false,              // Does it have destruction animation?
    true,               // Does it have transformation (blossom tree) anim?
    false,              // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    true,               // Is it immune to normal combat damage?
    "SPLIT2",
    TXT_BLOSSOM_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List0010,
    _List1101,
);

const Split2Class: TerrainTypeClass<2, 4> = TerrainTypeClass::new(
    TERRAIN_BLOSSOMTREE2,
    THEATERF::TEMPERATE.bits() | THEATERF::WINTER.bits() | THEATERF::DESERT.bits(),
    XYP_COORD!(18, 44), // Center base coordinate offset.
    true,               // Spawns Tiberium spontaneously?
    false,              // Does it have destruction animation?
    true,               // Does it have transformation (blossom tree) anim?
    false,              // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    true,               // Is it immune to normal combat damage?
    "SPLIT3",
    TXT_BLOSSOM_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List0010,
    _List1101,
);

const Clump1Class: TerrainTypeClass<3, 4> = TerrainTypeClass::new(
    TERRAIN_CLUMP1,
    THEATERF::WINTER.bits() | THEATERF::TEMPERATE.bits(),
    XYP_COORD!(28, 41), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    false,              // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    false,              // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    true,               // Is it immune to normal combat damage?
    "TC01",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List000110,
    _List110001,
);

const Clump2Class: TerrainTypeClass<4, 4> = TerrainTypeClass::new(
    TERRAIN_CLUMP2,
    THEATERF::WINTER.bits() | THEATERF::TEMPERATE.bits(),
    XYP_COORD!(38, 41), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    false,              // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    false,              // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    true,               // Is it immune to normal combat damage?
    "TC02",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List010110,
    _List101001,
);

const Clump3Class: TerrainTypeClass<5, 2> = TerrainTypeClass::new(
    TERRAIN_CLUMP3,
    THEATERF::WINTER.bits() | THEATERF::TEMPERATE.bits(),
    XYP_COORD!(33, 35), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    false,              // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    false,              // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    true,               // Is it immune to normal combat damage?
    "TC03",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List110110,
    _List001,
);

const Clump4Class: TerrainTypeClass<5, 7> = TerrainTypeClass::new(
    TERRAIN_CLUMP4,
    THEATERF::WINTER.bits() | THEATERF::TEMPERATE.bits(),
    XYP_COORD!(44, 49), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    false,              // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    false,              // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    true,               // Is it immune to normal combat damage?
    "TC04",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List000011101000,
    _List111000010110,
);

const Clump5Class: TerrainTypeClass<7, 6> = TerrainTypeClass::new(
    TERRAIN_CLUMP5,
    THEATERF::WINTER.bits() | THEATERF::TEMPERATE.bits(),
    XYP_COORD!(49, 58), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    false,              // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    false,              // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    true,               // Is it immune to normal combat damage?
    "TC05",
    TXT_TREE,
    TREE_NORMAL,
    ARMOR_WOOD,
    _List001011100110,
    _List110000011001,
);

const Rock1Class: TerrainTypeClass<3, 5> = TerrainTypeClass::new(
    TERRAIN_ROCK1,
    THEATERF::DESERT.bits(),
    XYP_COORD!(33, 41), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    false,              // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    false,              // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    true,               // Is it immune to normal combat damage?
    "ROCK1",
    TXT_ROCK,
    1000,
    ARMOR_STEEL,
    _List0011,
    _List111001,
);

const Rock2Class: TerrainTypeClass<3, 2> = TerrainTypeClass::new(
    TERRAIN_ROCK2,
    THEATERF::DESERT.bits(),
    XYP_COORD!(24, 23), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    false,              // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    false,              // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    true,               // Is it immune to normal combat damage?
    "ROCK2",
    TXT_ROCK,
    1000,
    ARMOR_STEEL,
    _List1100,
    _List001,
);

const Rock3Class: TerrainTypeClass<3, 4> = TerrainTypeClass::new(
    TERRAIN_ROCK3,
    THEATERF::DESERT.bits(),
    XYP_COORD!(20, 39), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    false,              // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    false,              // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    true,               // Is it immune to normal combat damage?
    "ROCK3",
    TXT_ROCK,
    1000,
    ARMOR_STEEL,
    _List000110,
    _List110001,
);

const Rock4Class: TerrainTypeClass<2, 2> = TerrainTypeClass::new(
    TERRAIN_ROCK4,
    THEATERF::DESERT.bits(),
    XYP_COORD!(12, 20), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    false,              // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    false,              // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    true,               // Is it immune to normal combat damage?
    "ROCK4",
    TXT_ROCK,
    1000,
    ARMOR_STEEL,
    _List10,
    _List01,
);

const Rock5Class: TerrainTypeClass<2, 2> = TerrainTypeClass::new(
    TERRAIN_ROCK5,
    THEATERF::DESERT.bits(),
    XYP_COORD!(17, 19), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    false,              // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    false,              // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    true,               // Is it immune to normal combat damage?
    "ROCK5",
    TXT_ROCK,
    1000,
    ARMOR_STEEL,
    _List10,
    _List01,
);

const Rock6Class: TerrainTypeClass<4, 3> = TerrainTypeClass::new(
    TERRAIN_ROCK6,
    THEATERF::DESERT.bits(),
    XYP_COORD!(28, 40), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    false,              // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    false,              // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    true,               // Is it immune to normal combat damage?
    "ROCK6",
    TXT_ROCK,
    1000,
    ARMOR_STEEL,
    _List000111,
    _List110000,
);

const Rock7Class: TerrainTypeClass<5, 2> = TerrainTypeClass::new(
    TERRAIN_ROCK7,
    THEATERF::DESERT.bits(),
    XYP_COORD!(57, 22), // Center base coordinate offset.
    false,              // Spawns Tiberium spontaneously?
    false,              // Does it have destruction animation?
    false,              // Does it have transformation (blossom tree) anim?
    false,              // Does it catch fire?
    false,              // Is this object crushable by heavy vehicles?
    false,              // Can this object be selected by the player?
    false,              // Can it be the target of a move or attack order?
    true,               // Don't make a big deal about it if it gets destroyed?
    true,               // Is it immune to normal combat damage?
    "ROCK7",
    TXT_ROCK,
    1000,
    ARMOR_STEEL,
    _List11110,
    _List00001,
);
