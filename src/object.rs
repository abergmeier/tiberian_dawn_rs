#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_variables)]

use crate::{abstract_::{AbstractTypeClass, MatchesInternalControlName}, armor::ArmorType, text::IDs};

/// This the the common base class of game objects. Since these values
/// represent the unchanging object TYPES, this data is initialized at game
/// start and not changed during play. It is "const" data.
pub struct ObjectTypeClass {
    abstract_type_class: AbstractTypeClass,
    /// Is this object squashable by heavy vehicles?  If it is, then the vehicle
    /// can travel over this object and destroy it in the process.
    IsCrushable: bool,

    /// Does this object type NOT show up on radar scans?  If true, then in any
    /// radar display, only the underlying ground will be show, not this object.
    /// Most terrain falls into this category, but only a few special real units/buildings
    /// do.
    IsStealthy: bool,

    /// It is legal to "select" some objects in the game. If it is legal to select this
    /// object type then this flag will be true. Selected game objects typically display
    /// a floating health bar and allows special user I/O control.
    IsSelectable: bool,

    /// Can this object be the target of an attack or move command?  Typically, only objects
    /// that take damage or can be destroyed are allowed to be a target.
    IsLegalTarget: bool,

    /// "Insignificant" objects will not be announced when they are destroyed or when they
    /// appear. Terrain elements and some lesser vehicles have this characteristic.
    IsInsignificant: bool,

    /// Is this object immune to normal combat damage?  Rocks and other inert type terrain
    /// object are typically of this type.
    IsImmune: bool,

    /// If this terrain object is flammable (such as trees are) then this
    /// flag will be true. Flammable objects can catch fire if damaged by
    /// flame type weapons.
    IsFlammable: bool,

    /// "Sentient" objects are ones that have logic AI processing performed on them. All
    /// vehicles, buildings, infantry, and aircraft are so flagged. Terrain elements also
    /// fall under this category, but only because certain animation effects require this.
    IsSentient: bool,

    /// The defense of this object is greatly affected by the type of armor
    /// it possesses. This value specifies the type of armor.
    Armor: Option<ArmorType>,

    /// This is the maximum strength of this object type.
    MaxStrength: u16,

    /// These point to the shape imagery for this object type. Since the shape imagery
    /// exists in a separate file, the data is filled in after this object is constructed.
    /// The "mutable" keyword allows easy modification to this otherwise const object.
    ImageData: [u8; 0],

    /// This points to the radar imagery for this object.
    RadarIcon: [u8; 0],
}

impl MatchesInternalControlName for ObjectTypeClass {
    fn matches_internal_control_name(&self, other_internal_control_name: &str) -> bool {
        self.abstract_type_class.matches_internal_control_name(other_internal_control_name)
    }
}

impl ObjectTypeClass {
    pub const fn new(
        is_sentient: bool,
        is_flammable: bool,
        is_crushable: bool,
        is_stealthy: bool,
        is_selectable: bool,
        is_legal_target: bool,
        is_insignificant: bool,
        is_immune: bool,
        name: Option<IDs>,
        ini: &str,
        armor: Option<ArmorType>,
        strength: u16,
    ) -> Self {
        Self {
            abstract_type_class: AbstractTypeClass::new(name, ini),
            IsSentient: is_sentient,
            IsFlammable: is_flammable,
            IsCrushable: is_crushable,
            IsStealthy: is_stealthy,
            IsSelectable: is_selectable,
            IsLegalTarget: is_legal_target,
            IsInsignificant: is_insignificant,
            IsImmune: is_immune,
            Armor: armor,
            MaxStrength: strength,
            ImageData: [],
            RadarIcon: [],
        }
    }
}
