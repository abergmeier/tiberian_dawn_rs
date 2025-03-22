use crate::abstract_::LookupByName;
use crate::team::TeamMissionType;
use strum::EnumCount;
use strum::IntoEnumIterator;

pub type TeamMissionNames = [&'static str; TeamMissionType::COUNT];

/// Was TMissions in C++ code.
const BORROWS: TeamMissionNames = [
    "Attack Base",
    "Attack Units",
    "Attack Civil.",
    "Rampage",
    "Defend Base",
    //	"Harvest",
    "Move",
    "Move to Cell",
    "Retreat",
    "Guard",
    "Loop",
    "Attack Tarcom",
    "Unload",
];

impl LookupByName for TeamMissionNames {
    type TypeEnum = TeamMissionType;

    fn lookup_type_enum_variant_by_name(name: &str) -> Option<Self::TypeEnum> {
        for order in Self::TypeEnum::iter() {
            if BORROWS[order as usize] == name {
                return Some(order);
            }
        }
        None
    }
}
