use crate::abstract_::LookupByName;
use crate::mission::MissionType;
use strum::EnumCount;
use strum::IntoEnumIterator;

pub type MissionNames = [&'static str; MissionType::COUNT];

/// Unit order names. These names correspond to the player selectable orders
/// a unit can have. The system initiated orders have no use for the ASCII name
/// associated, but they are listed here for completeness sake.
///
/// Was Missions in C++ code.
const BORROWS: MissionNames = [
    "Sleep",
    "Attack",
    "Move",
    "Retreat",
    "Guard",
    "Sticky",
    "Enter",
    "Capture",
    "Harvest",
    "Area Guard",
    "Return",
    "Stop",
    "Ambush",
    "Hunt",
    "Timed Hunt",
    "Unload",
    "Sabotage",
    "Construction",
    "Selling",
    "Repair",
    "Rescue",
    "Missile",
];

impl LookupByName for MissionNames {
    type TypeEnum = MissionType;

    fn lookup_type_enum_variant_by_name(name: &str) -> Option<Self::TypeEnum> {
        for order in Self::TypeEnum::iter() {
            if BORROWS[order as usize] == name {
                return Some(order);
            }
        }
        None
    }
}
