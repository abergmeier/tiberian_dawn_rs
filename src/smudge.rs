#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables
)]

use crate::{object::ObjectTypeClass, text::IDs};

/// Smudges are enumerated here. Smudges are transparent icons that are
/// drawn over the underlying terrain in order to give the effect of
/// alterations to the terrin. Craters are a good example of this.
#[repr(u8)]
pub enum SmudgeType {
    //SMUDGE_NONE=-1,
    SMUDGE_CRATER1,
    SMUDGE_CRATER2,
    SMUDGE_CRATER3,
    SMUDGE_CRATER4,
    SMUDGE_CRATER5,
    SMUDGE_CRATER6,
    SMUDGE_SCORCH1,
    SMUDGE_SCORCH2,
    SMUDGE_SCORCH3,
    SMUDGE_SCORCH4,
    SMUDGE_SCORCH5,
    SMUDGE_SCORCH6,
    SMUDGE_BIB1,
    SMUDGE_BIB2,
    SMUDGE_BIB3,
    //SMUDGE_COUNT,
    //SMUDGE_FIRST=0
}

/// This type elaborates the various "smudge" effects that can occur. Smudges are
/// those elements which are on top off all the ground icons, but below anything
/// that is "above" it. This includes scorch marks, craters, and infantry bodies.
/// Smudges, be definition, contain transparency. The are modifiers to underlying
/// terrain imagery.
pub struct SmudgeTypeClass {
    object_type_class: ObjectTypeClass,
    /// What overlay is this.
    Type: SmudgeType,

    ///	This is the fully translated smudge name.
    FullName: i32,

    /// Some smudges are larger than one cell. If this is the case, then
    /// these dimensions specify the number of cells wide and tall the
    /// smudge is.
    Width: i32,
    Height: i32,

    /// Is this smudge a crater type? If so, then a second crater can be added to
    /// this smudge so that a more cratered landscape results.
    IsCrater: bool,

    /// Is this overlay used as the attached road piece for buildings (bib)?
    IsBib: bool,
}

impl SmudgeTypeClass {
    pub const fn new(
        smudge: SmudgeType,
        ininame: &str,
        fullname: IDs,
        width: i32,
        height: i32,
        isbib: bool,
        iscrater: bool,
    ) -> Self {
        Self {
            object_type_class: ObjectTypeClass::new(
                false,
                false,
                false,
                true,
                false,
                false,
                true,
                true,
                Some(fullname),
                ininame,
                None,
                0,
            ),
            IsBib: isbib,
            Width: width,
            Height: height,
            IsCrater: iscrater,
            Type: smudge,
            FullName: 0,
        }
    }
}
