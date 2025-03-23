#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables
)]

use crate::abstract_::LookupByName;
use regex::{Captures, Regex};
use std::{num::ParseIntError, sync::LazyLock};
use strum_macros::{EnumCount, EnumIter};

use crate::{
    abstract_::{AbstractTypeClass, LookupByInternalControlName},
    aircraft::AircraftTypeClass,
    house::{HouseTypeClass, HousesType},
    infantry::InfantryTypeClass,
    ini::IniName,
    tmdata::TeamMissionNames,
    unit::UnitTypeClass,
};

/// TeamMissionType: the various missions that a team can have.
#[derive(Clone, Copy, Debug, EnumCount, EnumIter, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TeamMissionStruct {
    Mission: Option<TeamMissionType>,
    Argument: i32,
}

enum TechnoTypeClasses {
    None,
    Aircraft(&'static AircraftTypeClass),
    Infantry(&'static InfantryTypeClass),
    Unit(&'static UnitTypeClass),
}

impl Default for TechnoTypeClasses {
    fn default() -> Self {
        Self::None
    }
}

impl From<&'static AircraftTypeClass> for TechnoTypeClasses {
    fn from(value: &'static AircraftTypeClass) -> Self {
        Self::Aircraft(value)
    }
}

impl From<&'static InfantryTypeClass> for TechnoTypeClasses {
    fn from(value: &'static InfantryTypeClass) -> Self {
        Self::Infantry(value)
    }
}

impl From<&'static UnitTypeClass> for TechnoTypeClasses {
    fn from(value: &'static UnitTypeClass) -> Self {
        Self::Unit(value)
    }
}

const MAX_TEAM_CLASSCOUNT: usize = 5;
const MAX_TEAM_MISSIONS: usize = 20;

#[derive(Default)]
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
    House: Option<HousesType>,

    /// The mission list for this team
    MissionCount: u8,
    MissionList: [TeamMissionStruct; MAX_TEAM_MISSIONS],

    /// Number of different classes in the team
    ClassCount: u8,

    /// Array of object types comprising the team
    Class: [TechnoTypeClasses; MAX_TEAM_CLASSCOUNT],

    /// Desired # of each type of object comprising the team
    DesiredNum: [u8; MAX_TEAM_CLASSCOUNT],
}

impl IniName for TeamTypeClass {
    fn INI_Name() -> &'static str {
        "TeamTypes"
    }
}

pub enum NewError {
    NewFailedDueToCapacity,
}

impl TeamTypeClass {
    /// Creates a new instance of TeamTypeClass in Scenario
    ///
    /// Was new in C++ code.
    pub fn new(buf: &mut TeamTypeClass) -> &mut Self {
        *buf = Self {
            IsActive: true,
            ..Default::default()
        };
        buf
    }
}

#[derive(Debug, PartialEq)]
pub enum FillError {
    ParseIntError(ParseIntError),
}

static INI_LINE_PARSER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?<house>\w+),(?<roundabout>\d+),(?<learning>\d+),(?<suicide>\d+),(?<autocreate>\d+),(?<mercenary>\d+),(?<recruit>\d+),(?<max>\d+),(?<init>\d+),(?<fear>\d+),(?<classcount>\d+),(.*)").unwrap()
});

impl TeamTypeClass {
    fn parse_bool(s: &str) -> Result<bool, FillError> {
        Ok(Self::i32_to_bool(Self::parse_i32(s)?))
    }

    fn parse_i32(s: &str) -> Result<i32, FillError> {
        s.parse::<i32>()
            .map_err(|err| FillError::ParseIntError(err))
    }

    fn parse_u8(s: &str) -> Result<u8, FillError> {
        s.parse::<u8>().map_err(|err| FillError::ParseIntError(err))
    }

    fn parse_bool_group(captures: &Captures, i: usize) -> Result<bool, FillError> {
        Self::parse_bool(captures.get(i).unwrap().as_str())
    }

    fn parse_i32_group(captures: &Captures, i: usize) -> Result<i32, FillError> {
        Self::parse_i32(captures.get(i).unwrap().as_str())
    }

    fn parse_u8_group(captures: &Captures, i: usize) -> Result<u8, FillError> {
        Self::parse_u8(captures.get(i).unwrap().as_str())
    }

    fn i32_to_bool(v: i32) -> bool {
        v != 0
    }

    /// fills in trigger from the given INI entry
    /// This routine fills in the given teamtype with the given name, and values from
    /// the given INI entry.
    /// (This routine is used by the scenario editor, to import teams from the MASTER.INI file.)
    ///
    /// Parameters:
    /// * name: mnemonic for the desired trigger
    /// * entry: comma separated value to parse. Contains
    ///   Housename,Roundabout,Learning,Suicide,Spy,Mercenary,
    ///   RecruitPriority,MaxAllowed,InitNum,Fear,
    ///   ClassCount,Class:Num,Class:Num,...,
    ///   MissionCount,Mission:Arg,Mission:Arg,Mission:Arg,...
    fn Fill_In(&mut self, name: &str, entry: &str) -> Result<(), FillError> {
        let captured = INI_LINE_PARSER
            .captures(entry)
            .expect("entry should contain proper values");

        // ------------------------------ Set its name ------------------------------
        self.abstract_type_class.Set_Name(name);

        // ---------------------------- 1st token: House ----------------------------
        self.House = HouseTypeClass::lookup_type_enum_variant_by_internal_control_name(
            captured
                .get(1)
                .expect("House should always be specified")
                .as_str(),
        );

        // -------------------------- 2nd token: RoundAbout -------------------------
        self.IsRoundAbout = Self::parse_bool_group(&captured, 2)?;

        // --------------------------- 3rd token: Learning --------------------------
        self.IsLearning = Self::parse_bool_group(&captured, 3)?;

        // --------------------------- 4th token: Suicide ---------------------------
        self.IsSuicide = Self::parse_bool_group(&captured, 4)?;

        // ----------------------------- 5th token: Spy -----------------------------
        self.IsAutocreate = Self::parse_bool_group(&captured, 5)?;

        // -------------------------- 6th token: Mercenary --------------------------
        self.IsMercenary = Self::parse_bool_group(&captured, 6)?;

        // ----------------------- 7th token: RecruitPriority -----------------------
        self.RecruitPriority = Self::parse_i32_group(&captured, 7)?;

        // -------------------------- 8th token: MaxAllowed -------------------------
        self.MaxAllowed = Self::parse_u8_group(&captured, 8)?;

        // --------------------------- 9th token: InitNum ---------------------------
        self.InitNum = Self::parse_u8_group(&captured, 9)?;

        // ------------------------- 10th token: Fear level -------------------------
        self.Fear = Self::parse_u8_group(&captured, 10)?;

        //------------------------ 11th token: Class count -------------------------
        let num_classes = Self::parse_u8_group(&captured, 11)?;

        let rest = captured.get(12).unwrap().as_str();

        let it = rest.split(',');

        // -------------- Loop through entries, setting class borrows & num -------------
        self.ClassCount = 0;
        for s in it.clone().take(num_classes.into()) {
            let mut it = s.splitn(2, ':');
            let p1 = it.next().unwrap();
            let p2 = it.next().unwrap();

            // ------------------- See if this is an infantry name -------------------
            match InfantryTypeClass::lookup_type_enum_variant_by_internal_control_name(p1) {
                None => (),
                Some(i_id) => {
                    // --------------- If the name was resolved, add this class --------------
                    self.register_techno_class(p2, InfantryTypeClass::As_Reference(i_id).into())
                        .map_err(|err| FillError::ParseIntError(err))?;
                }
            }

            // ---------------------- See if this is a unit name ---------------------
            match UnitTypeClass::lookup_type_enum_variant_by_internal_control_name(p1) {
                None => (),
                Some(u_id) => {
                    // --------------- If the name was resolved, add this class --------------
                    self.register_techno_class(p2, UnitTypeClass::As_Reference(u_id).into())
                        .map_err(|err| FillError::ParseIntError(err))?;
                }
            }

            // ------------------- See if this is an aircraft name -------------------
            match AircraftTypeClass::lookup_type_enum_variant_by_internal_control_name(p1) {
                None => (),
                Some(a_id) => {
                    // --------------- If the name was resolved, add this class --------------
                    self.register_techno_class(p2, AircraftTypeClass::As_Reference(a_id).into())
                        .map_err(|err| FillError::ParseIntError(err))?;
                }
            }
        }

        let mut rest_it = it.skip(num_classes.into());

        self.MissionCount = Self::parse_u8(rest_it.next().unwrap())?;

        // ----------------------- next token: Mission count ------------------------
        for (i, s) in rest_it.clone().take(self.MissionCount.into()).enumerate() {
            let mut it = s.splitn(2, ':');
            let p1 = it.next().unwrap();
            let p2 = it.next().unwrap();
            let mission = &mut self.MissionList[i];
            mission.Mission = TeamMissionNames::lookup_type_enum_variant_by_name(p1);
            mission.Argument = p2.parse().map_err(|err| FillError::ParseIntError(err))?;
        }

        let mut opt_it = rest_it.skip(self.MissionCount.into());
        match opt_it.next() {
            None => return Ok(()),
            Some(reinforcable) => {
                self.IsReinforcable = Self::parse_bool(reinforcable)?;
            }
        }
        match opt_it.next() {
            None => return Ok(()),
            Some(prebuilt) => {
                self.IsPrebuilt = Self::parse_bool(prebuilt)?;
            }
        }
        Ok(())
    }

    fn register_techno_class(
        &mut self,
        p2: &str,
        techno_type_classable: TechnoTypeClasses,
    ) -> Result<(), ParseIntError> {
        self.Class[self.ClassCount as usize] = techno_type_classable;
        self.DesiredNum[self.ClassCount as usize] = p2.parse()?;
        self.ClassCount += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_in_works() {
        let mut buf: [TeamTypeClass; 1] = [Default::default(); 1];
        let ttc = &mut buf[0];
        TeamTypeClass::new(ttc);
        let result = ttc.Fill_In(
            "NOD7",
            "BadGuy,1,0,0,0,0,15,0,0,0,2,E1:2,LTNK:1,1,Attack Units:40,0,0",
        );
        assert_eq!(result, Ok(()));
        assert_eq!(ttc.House, Some(HousesType::HOUSE_BAD));
        assert_eq!(ttc.IsActive, true);
        assert_eq!(ttc.IsRoundAbout, true);
        assert_eq!(ttc.IsLearning, false);
        assert_eq!(ttc.IsSuicide, false);
        assert_eq!(ttc.IsAutocreate, false);
        assert_eq!(ttc.IsMercenary, false);
        assert_eq!(ttc.RecruitPriority, 15);
        assert_eq!(ttc.MaxAllowed, 0);
        assert_eq!(ttc.ClassCount, 2);
        assert_eq!(ttc.MissionCount, 1);
        assert_eq!(ttc.MissionList, {
            let mut ms = [TeamMissionStruct {
                ..Default::default()
            }; MAX_TEAM_MISSIONS];
            ms[0] = TeamMissionStruct {
                Mission: Some(TeamMissionType::TMISSION_ATTACKUNITS),
                Argument: 40,
            };
            ms
        });
        assert_eq!(ttc.IsReinforcable, false);
        assert_eq!(ttc.IsPrebuilt, false);
    }
}
