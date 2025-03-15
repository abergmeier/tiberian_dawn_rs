#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_variables)]


use crate::{terrain::LandType, text::IDs};

/// The overlays are enumerated here. An overlay functions similarly to
/// a transparent icon. It is placed over the terrain but usually falls
/// "under" buildings, trees, and units.
#[repr(u8)]
pub  enum OverlayType {
	//OVERLAY_NONE=-1,
	OVERLAY_CONCRETE,			// Concrete.
	OVERLAY_SANDBAG_WALL,	// Piled sandbags.
	OVERLAY_CYCLONE_WALL,	// Chain-link fence.
	OVERLAY_BRICK_WALL,		// Solid concrete wall.
	OVERLAY_BARBWIRE_WALL,	// Barbed-wire wall.
	OVERLAY_WOOD_WALL,		// Wooden fence.
	OVERLAY_TIBERIUM1,		// Tiberium patch.
	OVERLAY_TIBERIUM2,		// Tiberium patch.
	OVERLAY_TIBERIUM3,		// Tiberium patch.
	OVERLAY_TIBERIUM4,		// Tiberium patch.
	OVERLAY_TIBERIUM5,		// Tiberium patch.
	OVERLAY_TIBERIUM6,		// Tiberium patch.
	OVERLAY_TIBERIUM7,		// Tiberium patch.
	OVERLAY_TIBERIUM8,		// Tiberium patch.
	OVERLAY_TIBERIUM9,		// Tiberium patch.
	OVERLAY_TIBERIUM10,		// Tiberium patch.
	OVERLAY_TIBERIUM11,		// Tiberium patch.
	OVERLAY_TIBERIUM12,		// Tiberium patch.
	OVERLAY_ROAD,				// Road/concrete piece.
	OVERLAY_SQUISH,			// Squish mark for overran infantry.
	OVERLAY_V12,				// Haystacks
	OVERLAY_V13,				// Haystack
	OVERLAY_V14,				// Wheat field
	OVERLAY_V15,				// Fallow field
	OVERLAY_V16,				//	Corn field
	OVERLAY_V17,				// Celery field
	OVERLAY_V18,				// Potato field
	OVERLAY_FLAG_SPOT,		// Flag start location.
	OVERLAY_WOOD_CRATE,		// Wooden goodie crate.
	OVERLAY_STEEL_CRATE,		//	Steel goodie crate.

	//OVERLAY_COUNT,
	//OVERLAY_FIRST=0
}

/// This controls the overlay object types. These object types include walls
/// and concrete. They are always considered to be one icon in size and
/// are processed on an icon by icon basis. This is different from normal
/// templates which can be an arbitrary size. Other than this they are
/// mostly similar to normal templates but with some characteristics of
/// structures (they can be destroyed).
pub struct OverlayTypeClass {
    /// What overlay is this.
    Type: OverlayType,

    /// This is the fully translated name for the terrain element.
    FullName: i32,

    /// What type of ground does this make the cell it occupies?
    Land: LandType,

    /// If this overlay is a wall, how many stages of destruction are there
    /// for this wall type? i.e. sandbags = 2, concrete = 4, etc.
    DamageLevels: i32,

    /// If this overlay is a wall, what amount of damage is necessary
    /// before the wall takes damage?
    DamagePoints: i32,

    /// Is this overlay graphic theater specific. This means that if there is
    /// custom art for this overlay that varies between different theaters, then
    /// this flag will be true.
    IsTheater: bool,

    /// Is this a wall type overlay?  Wall types change their shape
    /// depending on the existence of adjacent walls of the same type.
    IsWall: bool,

    /// If this overlay is actually a wall and this wall type is tall enough that
    ///	normal ground based straight line weapons will be blocked by it, then this
    ///	flag will be true. Brick fences are typical of this type.
    IsHigh: bool,

    /// If this overlay represents harvestable tiberium, then this flag
    /// will be true.
    IsTiberium: bool,

    /// If this is a wall that is made of wood, then this flag will be
    /// true. Such walls are affected by fire damage.
    IsWooden: bool,

    /// Is this a crate? If it is, then goodies may come out of it.
    IsCrate: bool,

    /// If this is true, then the overlay will not show up on the radar map.
    IsRadarVisible: bool,
}

impl OverlayTypeClass {
    pub const fn new(
        iconset: OverlayType,
        ininame: &str,
        fullname: IDs,
        ground: LandType,
        damagelevels: i32,
        damagepoints: i32,
        isradarvisible: bool,
        iswooden: bool,
        istarget: bool,
        iscrushable: bool,
        istiberium: bool,
        high: bool,
        theater: bool,
        walltype: bool,
        iscrate: bool,
    ) -> Self {
        /*ObjectTypeClass(false,
        false,
        iscrushable,
        true,
        false,
        istarget,
        true,
        false,
        fullname,
        ininame,
        ARMOR_NONE,
        0)*/
        Self {
            IsRadarVisible: isradarvisible,
            IsCrate: iscrate,
            IsWooden: iswooden,
            IsHigh: high,
            IsTheater: theater,
            IsTiberium: istiberium,
            Type: iconset,
            Land: ground,
            IsWall: walltype,
            DamageLevels: damagelevels,
            DamagePoints: damagepoints,
            FullName: 0,
        }
    }
}
