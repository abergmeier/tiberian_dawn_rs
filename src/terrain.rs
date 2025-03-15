#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables
)]

use crate::armor::ArmorType;
use crate::coords::COORDINATE;
use crate::object::ObjectTypeClass;
use crate::text::IDs;
use crate::theater::{self, THEATERF};

/// Terrain can be of these different classes. At any point in the game
/// a particular piece of ground must fall under one of these classifications.
/// This is true, even if it is undergoing a temporary transition.
pub enum LandType {
    LAND_CLEAR,    // "Clear" terrain.
    LAND_ROAD,     // Road terrain.
    LAND_WATER,    // Water.
    LAND_ROCK,     // Impassable rock.
    LAND_WALL,     // Wall (blocks movement).
    LAND_TIBERIUM, // Tiberium field.
    LAND_BEACH,    //	Beach terrain.

                   //LAND_COUNT
}

/// The three dimensional terrain objects are enumerated here. These
/// objects function similar to buildings in that they can be driven
/// behind and can take damage on an individual basis.
#[repr(u8)]
pub enum TerrainType {
    //TERRAIN_NONE=-1,
    TERRAIN_TREE1,
    TERRAIN_TREE2,
    TERRAIN_TREE3,
    TERRAIN_TREE4,
    TERRAIN_TREE5,
    TERRAIN_TREE6,
    TERRAIN_TREE7,
    TERRAIN_TREE8,
    TERRAIN_TREE9,
    TERRAIN_TREE10,
    TERRAIN_TREE11,
    TERRAIN_TREE12,
    TERRAIN_TREE13,
    TERRAIN_TREE14,
    TERRAIN_TREE15,
    TERRAIN_TREE16,
    TERRAIN_TREE17,
    TERRAIN_TREE18,
    TERRAIN_BLOSSOMTREE1,
    TERRAIN_BLOSSOMTREE2,
    TERRAIN_CLUMP1,
    TERRAIN_CLUMP2,
    TERRAIN_CLUMP3,
    TERRAIN_CLUMP4,
    TERRAIN_CLUMP5,
    TERRAIN_ROCK1,
    TERRAIN_ROCK2,
    TERRAIN_ROCK3,
    TERRAIN_ROCK4,
    TERRAIN_ROCK5,
    TERRAIN_ROCK6,
    TERRAIN_ROCK7,
    //TERRAIN_COUNT,
    //TERRAIN_FIRST=0
}

/// These are the different TYPES of terrain objects. Every terrain object must
/// be one of these types.
pub struct TerrainTypeClass<const OCC: usize, const OVC: usize> {
    object_type_class: ObjectTypeClass,
    /// Which terrain object does this class type represent.
    Type: TerrainType,

    /// Does this terrain element consist of a normal frame followed by a
    /// series of crumble frames?  Trees fall under this case.
    IsDestroyable: bool,

    /// Does this object have the capability to transform after a period
    /// of time (such as a blossom tree?
    IsTransformable: bool,

    /// Does this terrain object spawn the growth of Tiberium? Blossom trees are
    /// a good example of this.
    IsTiberiumSpawn: bool,

    /// This is the fully translated name for the terrain element.
    FullName: i16,

    /// This is the coordinate offset (from upper left) of where the center base
    /// position of the terrain object lies. For trees, this would be the base of
    /// the trunk. This is used for sorting purposes.
    CenterBase: COORDINATE,

    /// This is the bitfield control that tells which theater this terrain object is
    /// valid for. If the bit (1 << TheaterType) is true, then this terrain object
    /// is allowed.
    Theater: u8,

    Occupy: [u16; OCC],
    Overlap: [u16; OVC],
}

impl<const OCC: usize, const OVC: usize> TerrainTypeClass<OCC, OVC> {
    pub const fn new(
        terrain: TerrainType,
        theater: <THEATERF as bitflags::Flags>::Bits,
        centerbase: COORDINATE,
        is_spawn: bool,
        is_destroyable: bool,
        is_transformable: bool,
        is_flammable: bool,
        is_crushable: bool,
        is_selectable: bool,
        is_legal_target: bool,
        is_insignificant: bool,
        is_immune: bool,
        ininame: &str,
        fullname: IDs,
        strength: u16,
        armor: ArmorType,
        occupy: [u16; OCC],
        overlap: [u16; OVC],
    ) -> Self {
        Self {
            object_type_class: ObjectTypeClass::new(
                true,
                is_flammable,
                is_crushable,
                true,
                is_selectable,
                is_legal_target,
                is_insignificant,
                is_immune,
                Some(fullname),
                ininame,
                Some(armor),
                strength,
            ),
            CenterBase: centerbase,
            IsTiberiumSpawn: is_spawn,
            IsDestroyable: is_destroyable,
            IsTransformable: is_transformable,
            Theater: theater,
            Type: terrain,
            Occupy: occupy,
            Overlap: overlap,
            FullName: 0,
        }
    }
}
