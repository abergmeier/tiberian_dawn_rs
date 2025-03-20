#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_variables)]

use crate::{
    abstract_::AbstractTypeClass, house::HousesType, ini::IniName, techno::TechnoTypeClass,
};

/// TeamMissionType: the various missions that a team can have.
pub enum TeamMissionType {
    //TMISSION_NONE=-1,
    TMISSION_ATTACKBASE,      // Attack nearest enemy base.
    TMISSION_ATTACKUNITS,     // Attack all enemy units.
    TMISSION_ATTACKCIVILIANS, // Attack all civilians
    TMISSION_RAMPAGE,         // attack & destroy anything that's not mine
    TMISSION_DEFENDBASE,      // Protect my base.
    //	TMISSION_HARVEST,					// stake out a Tiberium claim, defend & harvest it
    TMISSION_MOVE,         // moves to waypoint specified.
    TMISSION_MOVECELL,     // moves to cell # specified.
    TMISSION_RETREAT,      // order given by superior team, for coordinating
    TMISSION_GUARD,        // works like an infantry's guard mission
    TMISSION_LOOP,         // loop back to start of mission list
    TMISSION_ATTACKTARCOM, // attack tarcom
    TMISSION_UNLOAD,       // Unload at current location.
                           //TMISSION_COUNT,
                           //TMISSION_FIRST=0
}

/// This structure contains one team mission value & its argument.
pub struct TeamMissionStruct {
    Mission: TeamMissionType,
    Argument: i32,
}

pub struct TeamTypeClass {
    abstract_type_class: AbstractTypeClass,

    /// this teamtype object is active, then this flag will be true.
    /// TeamType objects that are not active are either not yet created or have
    /// been deleted after fulfilling their action.
    IsActive: bool,

    /// If RoundAbout, the team avoids high-threat areas
    IsRoundAbout: bool,

    /// If Learning, the team learns from mistakes
    IsLearning: bool,

    /// If Suicide, the team won't stop until it achieves its mission or it's
    /// dead
    IsSuicide: bool,

    /// Is this team type allowed to be created automatically by the computer
    /// when the appropriate trigger indicates?
    IsAutocreate: bool,

    /// Mercenaries will change sides if they start to lose.
    IsMercenary: bool,

    /// This flag tells the computer that it should build members to fill
    /// a team of this type regardless of whether there actually is a team
    /// of this type active.
    IsPrebuilt: bool,

    /// If this team should allow recruitment of new members, then this flag
    /// will be true. A false value results in a team that fights until it
    /// is dead. This is similar to IsSuicide, but they will defend themselves.
    IsReinforcable: bool,

    /// A transient team type was created exclusively to bring on reinforcements
    /// as a result of some special event. As soon as there are no teams
    /// existing of this type, then this team type should be deleted.
    IsTransient: bool,

    /// Priority given the team for recruiting purposes; higher priority means
    /// it can steal members from other teams (scale: 0 - 15)
    RecruitPriority: i32,

    /// Initial # of this type of team
    InitNum: u8,

    /// Max # of this type of team allowed at one time
    MaxAllowed: u8,

    /// Fear level of this team
    Fear: u8,

    /// House the team belongs to
    House: HousesType,

    /// The mission list for this team
    MissionCount: i32,
    MissionList: [TeamMissionStruct; 0],

    /// Number of different classes in the team
    ClassCount: u8,

    /// Array of object types comprising the team
    Class: [TechnoTypeClass; 0],

    /// Desired # of each type of object comprising the team
    DesiredNum: [u8; 0],
}

impl IniName for TeamTypeClass {
    fn INI_Name() -> &'static str {
        "TeamTypes"
    }
}
