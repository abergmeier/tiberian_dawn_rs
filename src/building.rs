#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables
)]

use crate::speed::MPHType::MPH_IMMOBILE;
use crate::{animation::BStateType::*, techno::TechnoTypeClass};
use bitflags::bitflags;

///	All game buildings (structures) are enumerated here. This includes
///	civilian structures as well.
#[repr(u8)]
pub enum StructType {
    //None=-1,
    STRUCT_WEAP,
    STRUCT_GTOWER,
    STRUCT_ATOWER,
    STRUCT_OBELISK,
    STRUCT_RADAR,
    STRUCT_TURRET,
    STRUCT_CONST,
    STRUCT_REFINERY,
    STRUCT_STORAGE,
    STRUCT_HELIPAD,
    STRUCT_SAM,
    STRUCT_AIRSTRIP,
    STRUCT_POWER,
    STRUCT_ADVANCED_POWER,
    STRUCT_HOSPITAL,
    STRUCT_BARRACKS,
    STRUCT_TANKER,
    STRUCT_REPAIR,
    STRUCT_BIO_LAB,
    STRUCT_HAND,
    STRUCT_TEMPLE,
    STRUCT_EYE,
    STRUCT_MISSION,

    /*
    	**	All buildings that are never used as a prerequisite
    	**	for construction, follow this point. Typically, this is
    	**	limited to civilian structures.
    	*/
    STRUCT_V01,
    STRUCT_V02,
    STRUCT_V03,
    STRUCT_V04,
    STRUCT_V05,
    STRUCT_V06,
    STRUCT_V07,
    STRUCT_V08,
    STRUCT_V09,
    STRUCT_V10,
    STRUCT_V11,
    STRUCT_V12,
    STRUCT_V13,
    STRUCT_V14,
    STRUCT_V15,
    STRUCT_V16,
    STRUCT_V17,
    STRUCT_V18,
    STRUCT_PUMP,
    STRUCT_V20,
    STRUCT_V21,
    STRUCT_V22,
    STRUCT_V23,
    STRUCT_V24,
    STRUCT_V25,
    STRUCT_V26,
    STRUCT_V27,
    STRUCT_V28,
    STRUCT_V29,
    STRUCT_V30,
    STRUCT_V31,
    STRUCT_V32,
    STRUCT_V33,
    STRUCT_V34,
    STRUCT_V35,
    STRUCT_V36,
    STRUCT_V37,
    STRUCT_ROAD,
    STRUCT_SANDBAG_WALL,
    STRUCT_CYCLONE_WALL,
    STRUCT_BRICK_WALL,
    STRUCT_BARBWIRE_WALL,
    STRUCT_WOOD_WALL,
    //STRUCT_COUNT,
    //STRUCT_FIRST=0
}

use StructType::*;

use crate::{
    animation::{AnimControlType, BSTATE_COUNT},
    armor::ArmorType,
    coords::COORDINATE,
    direction::DirType,
    rtti::RTTIType,
    text::IDs,
    weapon::WeaponType,
};

bitflags! {
    pub struct Flags : u32 {
        const NONE           = 0;
        const ADVANCED_POWER = (1 << STRUCT_ADVANCED_POWER as u32);
        const REPAIR         = (1 << STRUCT_REPAIR as u32);
        const EYE            = (1 << STRUCT_EYE as u32);
        const TEMPLE         = (1 << STRUCT_TEMPLE as u32);
        const HAND           = (1 << STRUCT_HAND as u32);
        const BIO_LAB        = (1 << STRUCT_BIO_LAB as u32);
        const OBELISK        = (1 << STRUCT_OBELISK as u32);
        const ATOWER         = (1 << STRUCT_ATOWER as u32);
        const WEAP           = (1 << STRUCT_WEAP as u32);
        const GTOWER         = (1 << STRUCT_GTOWER as u32);
        const RADAR          = (1 << STRUCT_RADAR as u32);
        const TURRET         = (1 << STRUCT_TURRET as u32);
        // const CIV1           = (1 << STRUCT_CIV1 as u32);
        // const CIV2           = (1 << STRUCT_CIV2 as u32);
        // const CIV3           = (1 << STRUCT_CIV3 as u32);
        const CONST          = (1 << STRUCT_CONST as u32);
        const REFINERY       = (1 << STRUCT_REFINERY as u32);
        const STORAGE        = (1 << STRUCT_STORAGE as u32);
        const HELIPAD        = (1 << STRUCT_HELIPAD as u32);
        const SAM            = (1 << STRUCT_SAM as u32);
        const AIRSTRIP       = (1 << STRUCT_AIRSTRIP as u32);
        const POWER          = (1 << STRUCT_POWER as u32);
        const HOSPITAL       = (1 << STRUCT_HOSPITAL as u32);
        const BARRACKS       = (1 << STRUCT_BARRACKS as u32);
        const TANKER         = (1 << STRUCT_TANKER as u32);
        const MISSION        = (1 << STRUCT_MISSION as u32);
    }
}

pub type STRUCTF = Flags;

///	Each building has a predetermined size. These are the size numbers.
///	The trailing number is this define is the width and height (respectively)
///	of the building in cells.
#[repr(u8)]
pub enum BSizeType {
    //BSIZE_NONE=-1,
    BSIZE_11 = 0,
    BSIZE_21,
    BSIZE_12,
    BSIZE_22,
    BSIZE_23,
    BSIZE_32,
    BSIZE_33,
    BSIZE_42,
    BSIZE_55,
    //BSIZE_COUNT
}

pub struct BuildingTypeClass<const ELC: usize, const SLC: usize, const OLC: usize> {
    techno_type_class: TechnoTypeClass,
    ///	This flag controls whether the building is equiped with a dirt
    ///	bib or not. A building with a bib has a dirt patch automatically
    ///	attached to the structure when it is placed.
    IsBibbed: bool,

    ///
    ///	If this building is a special wall type, such that it exists as a building
    ///	for purposes of construction but transforms into an overlay wall object when
    ///	it is placed on the map, then this flag will be true.
    IsWall: bool,

    /// Buildings can have either simple or complex damage stages. If simple,
    /// then the second to the last frame is the half damage stage, and the last
    /// frame is the complete damage stage. For non-simple damage, buildings
    /// have a complete animation set for damaged as well as undamaged condition.
    /// Turrets, oil pumps, and repair facilities are a few examples.
    IsSimpleDamage: bool,

    ///
    ///	Some buildings can be placed directly on raw ground. Such buildings don't require
    ///	and are not affected by concrete or lack thereof. Typically, concrete itself is
    ///	considered sturdy. The same goes for walls and similar generic type structures.
    ///	The more sophisticated buildings are greatly affected by lack of concrete and thus
    ///	would have this flag set to false.
    IsSturdy: bool,

    ///	If this building really only has cosmetic idle animation, then this flag will be
    ///	true if this animation should run at a relatively constant rate regardless of game
    ///	speed setting.
    IsRegulated: bool,

    ///	For building that produce ground units (infantry and vehicles), there is a default
    ///	exit poit defined. This point is where the object is first placed on the map.
    ///	Typically, this is located next to a door. The unit will then travel on to a clear
    ///	terrain area and enter normal game processing.
    ExitPoint: COORDINATE,

    ///	When determine which cell to head toward when exiting a building, use the
    ///	list elaborated by this variable. There are directions of exit that are
    ///	more suitable than others. This list is here to inform the system which
    ///	directions those are.
    ExitList: [i16; ELC],

    ///	This is the structure type identifier. It can serve as a unique
    ///	identification number for building types.
    Type: StructType,

    ///	This is the starting facing to give this building when it first
    ///	gets constructed. The facing matches the final stage of the
    ///	construction animation.
    StartFace: DirType,

    ///	This is the size of the building. This size value is a rough indication
    ///	of the building's "footprint".
    Size: BSizeType,

    ///	For each stage that a building may be in, its animation is controlled
    ///	by this structure. It dictates the starting and length of the animation
    ///	frames needed for the specified state. In addition it specifies how long
    ///	to delay between changes in animation. With this data it is possible to
    ///	control the appearance of all normal buildings. Turrets and SAM sites are
    ///	an exception since their animation is not merely cosmetic.
    Anims: [AnimControlType; BSTATE_COUNT],

    ///	This is a pointer to a list of offsets (from the upper left corner) that
    ///	are used to indicate the building's "footprint". This footprint is used
    ///	to determine building placement legality and terrain passibility.
    OccupyList: [i16; SLC],

    ///	Buildings can often times overlap a cell but not actually "occupy" it for
    ///	purposes of movement. This points to a list of offsets that indicate which
    ///	cells the building has visual overlap but does not occupy.
    OverlapList: [i16; OLC],
}

impl<const ELC: usize, const SLC: usize, const OLC: usize> BuildingTypeClass<ELC, SLC, OLC> {
    pub const fn new(
        type_: StructType,
        name: IDs,
        ininame: &str,
        exitpoint: COORDINATE,
        level: u8,
        pre: <STRUCTF as bitflags::Flags>::Bits,
        is_scanner: bool,
        is_regulated: bool,
        is_bibbed: bool,
        is_nominal: bool,
        is_wall: bool,
        is_factory: bool,
        is_captureable: bool,
        is_flammable: bool,
        is_simpledamage: bool,
        is_stealthy: bool,
        is_selectable: bool,
        is_legal_target: bool,
        is_insignificant: bool,
        is_immune: bool,
        is_theater: bool,
        is_turret_equipped: bool,
        is_twoshooter: bool,
        is_repairable: bool,
        is_buildable: bool,
        is_crew: bool,
        is_sturdy: bool,
        tobuild: Option<RTTIType>,
        sframe: DirType,
        strength: u16,
        sightrange: u16,
        cost: u32,
        scenario: u8,
        risk: u32,
        reward: u32,
        ownable: u16,
        primary: Option<WeaponType>,
        secondary: Option<WeaponType>,
        armor: Option<ArmorType>,
        canenter: u64,
        capacity: u32,
        power: u32,
        drain: u32,
        size: BSizeType,
        exitlist: [i16; ELC],
        sizelist: [i16; SLC],
        overlap: [i16; OLC],
    ) -> Self {
        let bt = Self {
            techno_type_class: TechnoTypeClass::new(
                name,
                ininame,
                level,
                pre,
                false,
                is_scanner,
                is_nominal,
                false,
                is_flammable,
                false,
                is_stealthy,
                is_selectable,
                is_legal_target,
                is_insignificant,
                is_immune,
                is_theater,
                is_twoshooter,
                is_turret_equipped,
                is_repairable,
                is_buildable,
                is_crew,
                -1,
                strength * 2,
                MPH_IMMOBILE,
                sightrange,
                cost,
                scenario,
                risk,
                reward,
                ownable,
                primary,
                secondary,
                armor,
            ),
            ExitList: exitlist,
            ExitPoint: exitpoint,
            IsBibbed: is_bibbed,
            IsRegulated: is_regulated,
            IsSimpleDamage: is_simpledamage,
            IsSturdy: is_sturdy,
            IsWall: is_wall,
            OccupyList: sizelist,
            OverlapList: overlap,
            Size: size,
            StartFace: sframe,
            Type: type_,
            Anims: {
                let mut Anims: [AnimControlType; BSTATE_COUNT] = [
                    AnimControlType::default(),
                    AnimControlType::default(),
                    AnimControlType::default(),
                    AnimControlType::default(),
                    AnimControlType::default(),
                    AnimControlType::default(),
                ];
                Anims[BSTATE_CONSTRUCTION as usize].Start = 0;
                Anims[BSTATE_CONSTRUCTION as usize].Count = 1;
                Anims[BSTATE_CONSTRUCTION as usize].Rate = 0;

                Anims[BSTATE_IDLE as usize].Start = 0;
                Anims[BSTATE_IDLE as usize].Count = 1;
                Anims[BSTATE_IDLE as usize].Rate = 0;

                Anims[BSTATE_ACTIVE as usize].Start = 0;
                Anims[BSTATE_ACTIVE as usize].Count = 1;
                Anims[BSTATE_ACTIVE as usize].Rate = 0;

                Anims[BSTATE_AUX1 as usize].Start = 0;
                Anims[BSTATE_AUX1 as usize].Count = 1;
                Anims[BSTATE_AUX1 as usize].Rate = 0;

                Anims[BSTATE_AUX2 as usize].Start = 0;
                Anims[BSTATE_AUX2 as usize].Count = 1;
                Anims[BSTATE_AUX2 as usize].Rate = 0;
                Anims
            },
        };

        bt
    }
}
