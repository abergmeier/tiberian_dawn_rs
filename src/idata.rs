#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_variables)]
use strum::{EnumCount, IntoEnumIterator};

use crate::abstract_::{LookupByInternalControlName, MatchesInternalControlName};
use crate::house::HOUSEF;
use crate::infantry::{
    DoInfoStruct, DoType,
    InfantryType::{self, *},
};
use crate::speed::MPHType::*;
use crate::text::IDs::*;
use crate::weapon::WeaponType::*;
use crate::{building::STRUCTF, infantry::InfantryTypeClass};

/*
 * There were too many parameters for the InfantryTypeClass constructor so I have
 * created a table of Do Controls for each unit type and I am passing a pointer
 * to the table to the constructor instead of passing each value as it was before.
 */

// Minigunners

const MiniGunnerDos: [DoInfoStruct; DoType::COUNT] = [
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_READY
    DoInfoStruct::new(8, 1, 1),    // DO_STAND_GUARD
    DoInfoStruct::new(192, 1, 8),  // DO_PRONE
    DoInfoStruct::new(16, 6, 6),   // DO_WALK
    DoInfoStruct::new(64, 8, 8),   // DO_FIRE_WEAPON
    DoInfoStruct::new(128, 2, 2),  // DO_LIE_DOWN
    DoInfoStruct::new(144, 4, 4),  // DO_CRAWL
    DoInfoStruct::new(176, 2, 2),  // DO_GET_UP
    DoInfoStruct::new(192, 6, 8),  // DO_FIRE_PRONE
    DoInfoStruct::new(256, 16, 0), // DO_IDLE1
    DoInfoStruct::new(272, 16, 0), // DO_IDLE2
    DoInfoStruct::new(288, 13, 0), // DO_ON_GUARD
    DoInfoStruct::new(292, 10, 0), // DO_FIGHT_READY
    DoInfoStruct::new(301, 2, 0),  // DO_PUNCH
    DoInfoStruct::new(303, 6, 0),  // DO_KICK
    DoInfoStruct::new(309, 2, 0),  // DO_PUNCH_HIT1
    DoInfoStruct::new(311, 4, 0),  // DO_PUNCH_HIT2
    DoInfoStruct::new(315, 5, 0),  // DO_PUNCH_DEATH
    DoInfoStruct::new(319, 2, 0),  // DO_KICK_HIT1
    DoInfoStruct::new(321, 4, 0),  // DO_KICK_HIT2
    DoInfoStruct::new(325, 5, 0),  // DO_KICK_DEATH
    DoInfoStruct::new(330, 5, 0),  // DO_READY_WEAPON
    DoInfoStruct::new(382, 8, 0),  // DO_GUN_DEATH
    DoInfoStruct::new(398, 8, 0),  // DO_EXPLOSION_DEATH
    DoInfoStruct::new(398, 8, 0),  // DO_EXPLOSION2_DEATH
    DoInfoStruct::new(406, 12, 0), // DO_GRENADE_DEATH
    DoInfoStruct::new(418, 18, 0), // DO_FIRE_DEATH
    DoInfoStruct::new(436, 3, 3),  // DO_GESTURE1
    DoInfoStruct::new(460, 3, 3),  // DO_SALUTE1
    DoInfoStruct::new(484, 3, 3),  // DO_GESTURE2
    DoInfoStruct::new(508, 3, 3),  // DO_SALUTE2
    DoInfoStruct::new(0, 1, 1),    // DO_PULL_GUN			// N/A
    DoInfoStruct::new(0, 1, 1),    //	DO_PLEAD				// N/A
    DoInfoStruct::new(0, 1, 1),    //	DO_PLEAD_DEATH		// N/A
];

const E1: InfantryTypeClass = InfantryTypeClass::new(
    INFANTRY_E1,          // Infantry type number.
    TXT_E1,               // Translate name number for infantry type.
    "E1",                 // INI name for infantry.
    1,                    // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Is this a female type?
    true,                 // Is a leader type?
    true,                 // Has crawling animation frames?
    false,                // Is this a civlian?
    false,                // Always use the given name for the infantry?
    false,                // Is this a "fraidycat" run-away type infantry?
    false,                // Can this infantry type capture a building?
    false,                // Theater specific graphic image?
    -1,                   // Number of shots it has (default).
    MiniGunnerDos,        // Ptr to minigunner 'DO' table above
    2,                    // Frame of projectile launch.
    2,                    // Frame of projectile launch while prone.
    50,                   // Strength of infantry (in damage points).
    1,                    // Sight range.
    100,                  // Cost of infantry (in credits).
    1,                    // Scenario when they first appear.
    80,
    10, // Risk/Reward of this infantry unit.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::BAD.bits(), // Who can own this infantry unit.
    Some(WEAPON_M16),
    None,
    MPH_SLOW, // Maximum speed of infantry.
);

// Grenadiers

const GrenadierDos: [DoInfoStruct; DoType::COUNT] = [
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_READY
    DoInfoStruct::new(8, 1, 1),    // DO_STAND_GUARD
    DoInfoStruct::new(288, 1, 12), // DO_PRONE
    DoInfoStruct::new(16, 6, 6),   // DO_WALK
    DoInfoStruct::new(64, 20, 20), // DO_FIRE_WEAPON
    DoInfoStruct::new(224, 2, 2),  // DO_LIE_DOWN
    DoInfoStruct::new(240, 4, 4),  // DO_CRAWL
    DoInfoStruct::new(272, 2, 2),  // DO_GET_UP
    DoInfoStruct::new(288, 8, 12), // DO_FIRE_PRONE
    DoInfoStruct::new(384, 16, 0), // DO_IDLE1
    DoInfoStruct::new(400, 16, 0), // DO_IDLE2
    DoInfoStruct::new(416, 13, 0), // DO_ON_GUARD
    DoInfoStruct::new(420, 10, 0), // DO_FIGHT_READY
    DoInfoStruct::new(429, 2, 0),  // DO_PUNCH
    DoInfoStruct::new(431, 6, 0),  // DO_KICK
    DoInfoStruct::new(437, 2, 0),  // DO_PUNCH_HIT1
    DoInfoStruct::new(439, 4, 0),  // DO_PUNCH_HIT2
    DoInfoStruct::new(443, 4, 0),  // DO_PUNCH_DEATH
    DoInfoStruct::new(447, 2, 0),  // DO_KICK_HIT1
    DoInfoStruct::new(449, 4, 0),  // DO_KICK_HIT2
    DoInfoStruct::new(453, 5, 0),  // DO_KICK_DEATH
    DoInfoStruct::new(458, 5, 0),  // DO_READY_WEAPON
    DoInfoStruct::new(510, 8, 0),  // DO_GUN_DEATH
    DoInfoStruct::new(526, 8, 0),  // DO_EXPLOSION_DEATH
    DoInfoStruct::new(526, 8, 0),  // DO_EXPLOSION2_DEATH
    DoInfoStruct::new(534, 12, 0), // DO_GRENADE_DEATH
    DoInfoStruct::new(546, 18, 0), // DO_FIRE_DEATH
    DoInfoStruct::new(564, 3, 3),  // DO_GESTURE1
    DoInfoStruct::new(588, 3, 3),  // DO_SALUTE1
    DoInfoStruct::new(612, 3, 3),  // DO_GESTURE2
    DoInfoStruct::new(636, 3, 3),  // DO_SALUTE2
    DoInfoStruct::new(0, 1, 1),    // DO_PULL_GUN			// N/A
    DoInfoStruct::new(0, 1, 1),    //	DO_PLEAD				// N/A
    DoInfoStruct::new(0, 1, 1),    //	DO_PLEAD_DEATH		// N/A
];

const E2: InfantryTypeClass = InfantryTypeClass::new(
    INFANTRY_E2,          // Infantry type number.
    TXT_E2,               // Translate name number for infantry type.
    "E2",                 // INI name for infantry.
    1,                    // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Is this a female type?
    true,                 // Is a leader type?
    true,                 // Has crawling animation frames?
    false,                // Is this a civlian?
    false,                // Always use the given name for the infantry?
    false,                // Is this a "fraidycat" run-away type infantry?
    false,                // Can this infantry type capture a building?
    false,                // Theater specific graphic image?
    -1,                   // Number of shots it has (default).
    GrenadierDos,         // Ptr to grenadier DO table (above)
    14,                   // Frame of projectile launch.
    6,                    // Frame of projectile launch while prone.
    50,                   // Strength of infantry (in damage points).
    1,                    // Sight range.
    160,                  // Cost of infantry (in credits).
    3,                    // Scenario when they first appear.
    80,
    10, // Risk/Reward of this infantry unit.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits(), // Who can own this infantry unit.
    Some(WEAPON_GRENADE),
    None,
    MPH_SLOW_ISH, // Maximum speed of infantry.
);

// Bazooka

const BazookaDos: [DoInfoStruct; DoType::COUNT] = [
    DoInfoStruct::new(0, 1, 1),     // DO_STAND_READY
    DoInfoStruct::new(8, 1, 1),     // DO_STAND_GUARD
    DoInfoStruct::new(192, 1, 10),  // DO_PRONE
    DoInfoStruct::new(16, 6, 6),    // DO_WALK
    DoInfoStruct::new(64, 8, 8),    // DO_FIRE_WEAPON
    DoInfoStruct::new(128, 2, 2),   // DO_LIE_DOWN
    DoInfoStruct::new(144, 4, 4),   // DO_CRAWL
    DoInfoStruct::new(176, 2, 2),   // DO_GET_UP
    DoInfoStruct::new(192, 10, 10), // DO_FIRE_PRONE
    DoInfoStruct::new(272, 16, 0),  // DO_IDLE1
    DoInfoStruct::new(288, 16, 0),  // DO_IDLE2
    DoInfoStruct::new(304, 13, 0),  // DO_ON_GUARD
    DoInfoStruct::new(308, 10, 0),  // DO_FIGHT_READY
    DoInfoStruct::new(307, 2, 0),   // DO_PUNCH
    DoInfoStruct::new(319, 6, 0),   // DO_KICK
    DoInfoStruct::new(325, 2, 0),   // DO_PUNCH_HIT1
    DoInfoStruct::new(327, 4, 0),   // DO_PUNCH_HIT2
    DoInfoStruct::new(331, 4, 0),   // DO_PUNCH_DEATH
    DoInfoStruct::new(335, 2, 0),   // DO_KICK_HIT1
    DoInfoStruct::new(337, 4, 0),   // DO_KICK_HIT2
    DoInfoStruct::new(341, 5, 0),   // DO_KICK_DEATH
    DoInfoStruct::new(346, 5, 0),   // DO_READY_WEAPON
    DoInfoStruct::new(398, 8, 0),   // DO_GUN_DEATH
    DoInfoStruct::new(414, 8, 0),   // DO_EXPLOSION_DEATH
    DoInfoStruct::new(414, 8, 0),   // DO_EXPLOSION2_DEATH
    DoInfoStruct::new(422, 12, 0),  // DO_GRENADE_DEATH
    DoInfoStruct::new(434, 18, 0),  // DO_FIRE_DEATH
    DoInfoStruct::new(452, 3, 3),   // DO_GESTURE1
    DoInfoStruct::new(476, 3, 3),   // DO_SALUTE1
    DoInfoStruct::new(500, 3, 3),   // DO_GESTURE2
    DoInfoStruct::new(524, 3, 3),   // DO_SALUTE2
    DoInfoStruct::new(0, 1, 1),     // DO_PULL_GUN			// N/A
    DoInfoStruct::new(0, 1, 1),     //	DO_PLEAD				// N/A
    DoInfoStruct::new(0, 1, 1),     //	DO_PLEAD_DEATH		// N/A
];

const E3: InfantryTypeClass = InfantryTypeClass::new(
    INFANTRY_E3,          // Infantry type number.
    TXT_E3,               // Translate name number for infantry type.
    "E3",                 // INI name for infantry.
    2,                    // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Is this a female type?
    true,                 // Is a leader type?
    true,                 // Has crawling animation frames?
    false,                // Is this a civlian?
    false,                // Always use the given name for the infantry?
    false,                // Is this a "fraidycat" run-away type infantry?
    false,                // Can this infantry type capture a building?
    false,                // Theater specific graphic image?
    -1,                   // Number of shots it has (default).
    BazookaDos,           // Ptr to DO table (above)
    3,                    // Frame of projectile launch.
    3,                    // Frame of projectile launch while prone.
    25,                   // Strength of infantry (in damage points).
    2,                    // Sight range.
    300,                  // Cost of infantry (in credits).
    3,                    // Scenario when they first appear.
    80,
    10, // Risk/Reward of this infantry unit.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::BAD.bits(), // Who can own this infantry unit.
    Some(WEAPON_DRAGON),
    None,
    MPH_KINDA_SLOW, // Maximum speed of infantry.
);

// Flamethrower

const FlamethrowerDos: [DoInfoStruct; DoType::COUNT] = [
    DoInfoStruct::new(0, 1, 1),     // DO_STAND_READY
    DoInfoStruct::new(8, 1, 1),     // DO_STAND_GUARD
    DoInfoStruct::new(256, 1, 16),  // DO_PRONE
    DoInfoStruct::new(16, 6, 6),    // DO_WALK
    DoInfoStruct::new(64, 16, 16),  // DO_FIRE_WEAPON
    DoInfoStruct::new(192, 2, 2),   // DO_LIE_DOWN
    DoInfoStruct::new(208, 4, 4),   // DO_CRAWL
    DoInfoStruct::new(240, 2, 2),   // DO_GET_UP
    DoInfoStruct::new(256, 16, 16), // DO_FIRE_PRONE
    DoInfoStruct::new(384, 16, 0),  // DO_IDLE1
    DoInfoStruct::new(400, 16, 0),  // DO_IDLE2
    DoInfoStruct::new(416, 13, 0),  // DO_ON_GUARD
    DoInfoStruct::new(420, 10, 0),  // DO_FIGHT_READY
    DoInfoStruct::new(429, 2, 0),   // DO_PUNCH
    DoInfoStruct::new(431, 6, 0),   // DO_KICK
    DoInfoStruct::new(437, 2, 0),   // DO_PUNCH_HIT1
    DoInfoStruct::new(439, 4, 0),   // DO_PUNCH_HIT2
    DoInfoStruct::new(443, 4, 0),   // DO_PUNCH_DEATH
    DoInfoStruct::new(447, 2, 0),   // DO_KICK_HIT1
    DoInfoStruct::new(449, 4, 0),   // DO_KICK_HIT2
    DoInfoStruct::new(453, 5, 0),   // DO_KICK_DEATH
    DoInfoStruct::new(458, 5, 0),   // DO_READY_WEAPON
    DoInfoStruct::new(510, 8, 0),   // DO_GUN_DEATH
    DoInfoStruct::new(526, 8, 0),   // DO_EXPLOSION_DEATH
    DoInfoStruct::new(526, 8, 0),   // DO_EXPLOSION2_DEATH
    DoInfoStruct::new(534, 12, 0),  // DO_GRENADE_DEATH
    DoInfoStruct::new(546, 18, 0),  // DO_FIRE_DEATH
    DoInfoStruct::new(564, 3, 3),   // DO_GESTURE1
    DoInfoStruct::new(588, 3, 3),   // DO_SALUTE1
    DoInfoStruct::new(612, 3, 3),   // DO_GESTURE2
    DoInfoStruct::new(636, 3, 3),   // DO_SALUTE2
    DoInfoStruct::new(0, 1, 1),     // DO_PULL_GUN			// N/A
    DoInfoStruct::new(0, 1, 1),     //	DO_PLEAD				// N/A
    DoInfoStruct::new(0, 1, 1),     //	DO_PLEAD_DEATH		// N/A
];

const E4: InfantryTypeClass = InfantryTypeClass::new(
    INFANTRY_E4,          // Infantry type number.
    TXT_E4,               // Translate name number for infantry type.
    "E4",                 // INI name for infantry.
    1,                    // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Is this a female type?
    true,                 // Is a leader type?
    true,                 // Has crawling animation frames?
    false,                // Is this a civlian?
    false,                // Always use the given name for the infantry?
    false,                // Is this a "fraidycat" run-away type infantry?
    false,                // Can this infantry type capture a building?
    false,                // Theater specific graphic image?
    -1,                   // Number of shots it has (default).
    FlamethrowerDos,      // ptr to DO table (above)
    2,                    // Frame of projectile launch.
    0,                    // Frame of projectile launch while prone.
    70,                   // Strength of infantry (in damage points).
    1,                    // Sight range.
    200,                  // Cost of infantry (in credits).
    5,                    // Scenario when they first appear.
    80,
    10, // Risk/Reward of this infantry unit.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits(), // Who can own this infantry unit.
    Some(WEAPON_FLAMETHROWER),
    None,
    MPH_SLOW_ISH,
);

// Chemwarrior

const ChemwarriorDos: [DoInfoStruct; DoType::COUNT] = [
    DoInfoStruct::new(0, 1, 1),     // DO_STAND_READY
    DoInfoStruct::new(8, 1, 1),     // DO_STAND_GUARD
    DoInfoStruct::new(256, 1, 16),  // DO_PRONE
    DoInfoStruct::new(16, 6, 6),    // DO_WALK
    DoInfoStruct::new(64, 16, 16),  // DO_FIRE_WEAPON
    DoInfoStruct::new(192, 2, 2),   // DO_LIE_DOWN
    DoInfoStruct::new(208, 4, 4),   // DO_CRAWL
    DoInfoStruct::new(240, 2, 2),   // DO_GET_UP
    DoInfoStruct::new(256, 16, 16), // DO_FIRE_PRONE
    DoInfoStruct::new(384, 16, 0),  // DO_IDLE1
    DoInfoStruct::new(400, 16, 0),  // DO_IDLE2
    DoInfoStruct::new(416, 13, 0),  // DO_ON_GUARD
    DoInfoStruct::new(420, 10, 0),  // DO_FIGHT_READY
    DoInfoStruct::new(429, 2, 0),   // DO_PUNCH
    DoInfoStruct::new(431, 6, 0),   // DO_KICK
    DoInfoStruct::new(437, 2, 0),   // DO_PUNCH_HIT1
    DoInfoStruct::new(439, 4, 0),   // DO_PUNCH_HIT2
    DoInfoStruct::new(443, 4, 0),   // DO_PUNCH_DEATH
    DoInfoStruct::new(447, 2, 0),   // DO_KICK_HIT1
    DoInfoStruct::new(449, 4, 0),   // DO_KICK_HIT2
    DoInfoStruct::new(453, 5, 0),   // DO_KICK_DEATH
    DoInfoStruct::new(458, 5, 0),   // DO_READY_WEAPON
    DoInfoStruct::new(510, 8, 0),   // DO_GUN_DEATH
    DoInfoStruct::new(526, 8, 0),   // DO_EXPLOSION_DEATH
    DoInfoStruct::new(526, 8, 0),   // DO_EXPLOSION2_DEATH
    DoInfoStruct::new(534, 12, 0),  // DO_GRENADE_DEATH
    DoInfoStruct::new(546, 18, 0),  // DO_FIRE_DEATH
    DoInfoStruct::new(564, 3, 3),   // DO_GESTURE1
    DoInfoStruct::new(588, 3, 3),   // DO_SALUTE1
    DoInfoStruct::new(612, 3, 3),   // DO_GESTURE2
    DoInfoStruct::new(636, 3, 3),   // DO_SALUTE2
    DoInfoStruct::new(0, 1, 1),     // DO_PULL_GUN			// N/A
    DoInfoStruct::new(0, 1, 1),     //	DO_PLEAD				// N/A
    DoInfoStruct::new(0, 1, 1),     //	DO_PLEAD_DEATH		// N/A
];

const E5: InfantryTypeClass = InfantryTypeClass::new(
    INFANTRY_E5,         // Infantry type number.
    TXT_E5,              // Translate name number for infantry type.
    "E5",                // INI name for infantry.
    7,                   // Build level.
    STRUCTF::EYE.bits(), // Building prerequisite.
    false,               // Is this a female type?
    true,                // Is a leader type?
    true,                // Has crawling animation frames?
    false,               // Is this a civlian?
    false,               // Always use the given name for the infantry?
    true,                // Is this a "fraidycat" run-away type infantry?
    false,               // Can this infantry type capture a building?
    false,               // Theater specific graphic image?
    -1,                  // Number of shots it has (default).
    ChemwarriorDos,      // ptr to DO table
    2,                   // Frame of projectile launch.
    0,                   // Frame of projectile launch while prone.
    70,                  // Strength of infantry (in damage points).
    1,                   // Sight range.
    300,                 // Cost of infantry (in credits).
    99,                  // Scenario when they first appear.
    80,
    10, // Risk/Reward of this infantry unit.
    HOUSEF::MULTI1.bits()|
	HOUSEF::MULTI2.bits()|
	HOUSEF::MULTI3.bits()|
	HOUSEF::MULTI4.bits()|
	HOUSEF::MULTI5.bits()|
	HOUSEF::MULTI6.bits()|
	HOUSEF::JP.bits()|
//	HOUSEF::GOOD.bits()|
	HOUSEF::BAD.bits(), // Who can own this infantry unit.
    Some(WEAPON_CHEMSPRAY),
    None,
    MPH_SLOW,
);

// Engineer

const EngineerDos: [DoInfoStruct; DoType::COUNT] = [
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_READY
    DoInfoStruct::new(8, 1, 1),    // DO_STAND_GUARD
    DoInfoStruct::new(82, 1, 4),   // DO_PRONE
    DoInfoStruct::new(16, 6, 6),   // DO_WALK
    DoInfoStruct::new(0, 0, 0),    // DO_FIRE_WEAPON
    DoInfoStruct::new(67, 2, 2),   // DO_LIE_DOWN
    DoInfoStruct::new(82, 4, 4),   // DO_CRAWL
    DoInfoStruct::new(114, 2, 2),  // DO_GET_UP
    DoInfoStruct::new(0, 0, 0),    // DO_FIRE_PRONE
    DoInfoStruct::new(130, 16, 0), // DO_IDLE1
    DoInfoStruct::new(0, 0, 0),    // DO_IDLE2
    DoInfoStruct::new(0, 0, 0),    // DO_ON_GUARD
    DoInfoStruct::new(0, 0, 0),    // DO_FIGHT_READY
    DoInfoStruct::new(0, 0, 0),    // DO_PUNCH
    DoInfoStruct::new(0, 0, 0),    // DO_KICK
    DoInfoStruct::new(0, 0, 0),    // DO_PUNCH_HIT1
    DoInfoStruct::new(0, 0, 0),    // DO_PUNCH_HIT2
    DoInfoStruct::new(0, 0, 0),    // DO_PUNCH_DEATH
    DoInfoStruct::new(0, 0, 0),    // DO_KICK_HIT1
    DoInfoStruct::new(0, 0, 0),    // DO_KICK_HIT2
    DoInfoStruct::new(0, 0, 0),    // DO_KICK_DEATH
    DoInfoStruct::new(0, 0, 0),    // DO_READY_WEAPON
    DoInfoStruct::new(146, 8, 0),  // DO_GUN_DEATH
    DoInfoStruct::new(154, 8, 0),  // DO_EXPLOSION_DEATH
    DoInfoStruct::new(162, 8, 0),  // DO_EXPLOSION2_DEATH
    DoInfoStruct::new(162, 12, 0), // DO_GRENADE_DEATH
    DoInfoStruct::new(182, 18, 0), // DO_FIRE_DEATH
    DoInfoStruct::new(200, 3, 3),  // DO_GESTURE1
    DoInfoStruct::new(224, 3, 3),  // DO_SALUTE1
    DoInfoStruct::new(200, 3, 3),  // DO_GESTURE2
    DoInfoStruct::new(224, 3, 3),  // DO_SALUTE2
    DoInfoStruct::new(0, 1, 1),    // DO_PULL_GUN			// N/A
    DoInfoStruct::new(0, 1, 1),    //	DO_PLEAD				// N/A
    DoInfoStruct::new(0, 1, 1),    //	DO_PLEAD_DEATH		// N/A
];

const E7: InfantryTypeClass = InfantryTypeClass::new(
    INFANTRY_E7,          // Infantry type number.
    TXT_E7,               // Translate name number for infantry type.
    "E6",                 // INI name for infantry.
    3,                    // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Is this a female type?
    false,                // Is a leader type?
    false,                // Has crawling animation frames?
    false,                // Is this a civlian?
    false,                // Always use the given name for the infantry?
    false,                // Is this a "fraidycat" run-away type infantry?
    true,                 // Can this infantry type capture a building?
    false,                // Theater specific graphic image?
    -1,                   // Number of shots it has (default).
    EngineerDos,          // ptr to DO table
    3,                    // Frame of projectile launch.
    3,                    // Frame of projectile launch while prone.
    25,                   // Strength of infantry (in damage points).
    2,                    // Sight range.
    500,                  // Cost of infantry (in credits).
    2,                    // Scenario when they first appear.
    80,
    75, // Risk/Reward of this infantry unit.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::GOOD.bits(), // Who can own this infantry unit.
    None,
    None,
    MPH_SLOW, // Maximum speed of infantry.
);

// Commandos

const CommandoDos: [DoInfoStruct; DoType::COUNT] = [
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_READY
    DoInfoStruct::new(8, 1, 1),    // DO_STAND_GUARD
    DoInfoStruct::new(160, 1, 4),  // DO_PRONE
    DoInfoStruct::new(16, 6, 6),   // DO_WALK
    DoInfoStruct::new(64, 4, 4),   // DO_FIRE_WEAPON
    DoInfoStruct::new(96, 2, 2),   // DO_LIE_DOWN
    DoInfoStruct::new(112, 4, 4),  // DO_CRAWL
    DoInfoStruct::new(144, 2, 2),  // DO_GET_UP
    DoInfoStruct::new(160, 4, 4),  // DO_FIRE_PRONE
    DoInfoStruct::new(192, 16, 0), // DO_IDLE1
    DoInfoStruct::new(208, 16, 0), // DO_IDLE2
    DoInfoStruct::new(224, 13, 0), // DO_ON_GUARD
    DoInfoStruct::new(228, 9, 0),  // DO_FIGHT_READY
    DoInfoStruct::new(237, 2, 0),  // DO_PUNCH
    DoInfoStruct::new(239, 6, 0),  // DO_KICK
    DoInfoStruct::new(245, 2, 0),  // DO_PUNCH_HIT1
    DoInfoStruct::new(247, 4, 0),  // DO_PUNCH_HIT2
    DoInfoStruct::new(251, 4, 0),  // DO_PUNCH_DEATH
    DoInfoStruct::new(255, 2, 0),  // DO_KICK_HIT1
    DoInfoStruct::new(257, 4, 0),  // DO_KICK_HIT2
    DoInfoStruct::new(261, 5, 0),  // DO_KICK_DEATH
    DoInfoStruct::new(266, 5, 0),  // DO_READY_WEAPON
    DoInfoStruct::new(318, 8, 0),  // DO_GUN_DEATH
    DoInfoStruct::new(334, 8, 0),  // DO_EXPLOSION_DEATH
    DoInfoStruct::new(334, 8, 0),  // DO_EXPLOSION2_DEATH
    DoInfoStruct::new(342, 12, 0), // DO_GRENADE_DEATH
    DoInfoStruct::new(354, 18, 0), // DO_FIRE_DEATH
    DoInfoStruct::new(372, 3, 3),  // DO_GESTURE1
    DoInfoStruct::new(396, 3, 3),  // DO_SALUTE1
    DoInfoStruct::new(420, 3, 3),  // DO_GESTURE2
    DoInfoStruct::new(444, 3, 3),  // DO_SALUTE2
    DoInfoStruct::new(0, 1, 1),    // DO_PULL_GUN			// N/A
    DoInfoStruct::new(0, 1, 1),    //	DO_PLEAD				// N/A
    DoInfoStruct::new(0, 1, 1),    //	DO_PLEAD_DEATH		// N/A
];
const Commando: InfantryTypeClass = InfantryTypeClass::new(
    INFANTRY_RAMBO,      // Infantry type number.
    TXT_RAMBO,           // Translate name number for infantry type.
    "RMBO",              // INI name for infantry.
    7,                   // Build level.
    STRUCTF::EYE.bits(), // Building prerequisite.
    false,               // Is this a female type?
    true,                // Is a leader type?
    true,                // Has crawling animation frames?
    false,               // Is this a civlian?
    false,               // Always use the given name for the infantry?
    false,               // Is this a "fraidycat" run-away type infantry?
    true,                // Can this infantry type capture a building?
    false,               // Theater specific graphic image?
    -1,                  // Number of shots it has (default).
    CommandoDos,         // ptr to DO table
    2,                   // Frame of projectile launch.
    2,                   // Frame of projectile launch while prone.
    80,                  // Strength of infantry (in damage points).
    5,                   // Sight range.
    1000,                // Cost of infantry (in credits).
    98,                  // Scenario when they first appear.
    80,
    75, // Risk/Reward of this infantry unit.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::BAD.bits(), // Who can own this infantry unit.
    Some(WEAPON_RIFLE),
    None,
    MPH_SLOW_ISH, // Maximum speed of infantry.
);

// Civilians

const CivilianDos1: [DoInfoStruct; DoType::COUNT] = [
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_READY
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_GUARD
    DoInfoStruct::new(0, 1, 1),    // DO_PRONE				// N/A
    DoInfoStruct::new(56, 6, 6),   // DO_WALK
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_WEAPON
    DoInfoStruct::new(0, 1, 1),    // DO_LIE_DOWN			// N/A
    DoInfoStruct::new(8, 6, 6),    // DO_CRAWL
    DoInfoStruct::new(0, 1, 1),    // DO_GET_UP			// N/A
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_PRONE
    DoInfoStruct::new(189, 10, 0), // DO_IDLE1
    DoInfoStruct::new(199, 6, 0),  // DO_IDLE2
    DoInfoStruct::new(104, 3, 0),  // DO_ON_GUARD
    DoInfoStruct::new(107, 7, 0),  // DO_FIGHT_READY
    DoInfoStruct::new(114, 2, 0),  // DO_PUNCH
    DoInfoStruct::new(116, 6, 0),  // DO_KICK
    DoInfoStruct::new(122, 2, 0),  // DO_PUNCH_HIT1
    DoInfoStruct::new(124, 4, 0),  // DO_PUNCH_HIT2
    DoInfoStruct::new(128, 4, 0),  // DO_PUNCH_DEATH
    DoInfoStruct::new(133, 2, 0),  // DO_KICK_HIT1
    DoInfoStruct::new(135, 4, 0),  // DO_KICK_HIT2
    DoInfoStruct::new(139, 5, 0),  // DO_KICK_DEATH
    DoInfoStruct::new(144, 3, 0),  // DO_READY_WEAPON
    DoInfoStruct::new(329, 8, 0),  // DO_GUN_DEATH
    DoInfoStruct::new(337, 8, 0),  // DO_EXPLOSION_DEATH
    DoInfoStruct::new(337, 8, 0),  // DO_EXPLOSION2_DEATH
    DoInfoStruct::new(345, 12, 0), // DO_GRENADE_DEATH
    DoInfoStruct::new(357, 18, 0), // DO_FIRE_DEATH
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE2			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE2			// N/A
    DoInfoStruct::new(199, 6, 0),  // DO_PULL_GUN
    DoInfoStruct::new(237, 40, 0), //	DO_PLEAD
    DoInfoStruct::new(277, 6, 0),  //	DO_PLEAD_DEATH
];

const C1: InfantryTypeClass = InfantryTypeClass::new(
    INFANTRY_C1,          // Infantry type number.
    TXT_C1,               // Translate name number for infantry type.
    "C1",                 // INI name for infantry.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Is this a female type?
    true,                 // Is a leader type?
    false,                // Has crawling animation frames?
    true,                 // Is this a civlian?
    true,                 // Always use the given name for the infantry?
    true,                 // Is this a "fraidycat" run-away type infantry?
    false,                // Can this infantry type capture a building?
    false,                // Theater specific graphic image?
    10,                   // Number of shots it has (default).
    CivilianDos1,         // ptr to DO table
    2,                    // Frame of projectile launch.
    0,                    // Frame of projectile launch while prone.
    25,                   // Strength of infantry (in damage points).
    0,                    // Sight range.
    10,                   // Cost of infantry (in credits).
    99,                   // Scenario when they first appear.
    0,
    1, // Risk/Reward of this infantry unit.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::NEUTRAL.bits(), // Who can own this infantry unit.
    Some(WEAPON_PISTOL),
    None,
    MPH_SLOW_ISH,
);

const CivilianDos2: [DoInfoStruct; DoType::COUNT] = [
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_READY
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_GUARD
    DoInfoStruct::new(0, 1, 1),    // DO_PRONE				// N/A
    DoInfoStruct::new(56, 6, 6),   // DO_WALK
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_WEAPON
    DoInfoStruct::new(0, 1, 1),    // DO_LIE_DOWN			// N/A
    DoInfoStruct::new(8, 6, 6),    // DO_CRAWL
    DoInfoStruct::new(0, 1, 1),    // DO_GET_UP			// N/A
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_PRONE
    DoInfoStruct::new(189, 10, 0), // DO_IDLE1
    DoInfoStruct::new(199, 6, 0),  // DO_IDLE2
    DoInfoStruct::new(104, 3, 0),  // DO_ON_GUARD
    DoInfoStruct::new(107, 7, 0),  // DO_FIGHT_READY
    DoInfoStruct::new(114, 2, 0),  // DO_PUNCH
    DoInfoStruct::new(116, 6, 0),  // DO_KICK
    DoInfoStruct::new(122, 2, 0),  // DO_PUNCH_HIT1
    DoInfoStruct::new(124, 4, 0),  // DO_PUNCH_HIT2
    DoInfoStruct::new(128, 4, 0),  // DO_PUNCH_DEATH
    DoInfoStruct::new(133, 2, 0),  // DO_KICK_HIT1
    DoInfoStruct::new(135, 4, 0),  // DO_KICK_HIT2
    DoInfoStruct::new(139, 5, 0),  // DO_KICK_DEATH
    DoInfoStruct::new(144, 3, 0),  // DO_READY_WEAPON
    DoInfoStruct::new(329, 8, 0),  // DO_GUN_DEATH
    DoInfoStruct::new(337, 8, 0),  // DO_EXPLOSION_DEATH
    DoInfoStruct::new(337, 8, 0),  // DO_EXPLOSION2_DEATH
    DoInfoStruct::new(345, 12, 0), // DO_GRENADE_DEATH
    DoInfoStruct::new(357, 18, 0), // DO_FIRE_DEATH
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE2			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE2			// N/A
    DoInfoStruct::new(199, 6, 0),  // DO_PULL_GUN
    DoInfoStruct::new(237, 40, 0), //	DO_PLEAD
    DoInfoStruct::new(277, 6, 0),  //	DO_PLEAD_DEATH
];

const C2: InfantryTypeClass = InfantryTypeClass::new(
    INFANTRY_C2,          // Infantry type number.
    TXT_C2,               // Translate name number for infantry type.
    "C2",                 // INI name for infantry.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Is this a female type?
    false,                // Is a leader type?
    false,                // Has crawling animation frames?
    true,                 // Is this a civlian?
    true,                 // Always use the given name for the infantry?
    true,                 // Is this a "fraidycat" run-away type infantry?
    false,                // Can this infantry type capture a building?
    false,                // Theater specific graphic image?
    0,                    // Number of shots it has (default).
    CivilianDos2,         // ptr to DO table
    2,                    // Frame of projectile launch.
    0,                    // Frame of projectile launch while prone.
    5,                    // Strength of infantry (in damage points).
    0,                    // Sight range.
    10,                   // Cost of infantry (in credits).
    99,                   // Scenario when they first appear.
    0,
    1, // Risk/Reward of this infantry unit.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::NEUTRAL.bits(), // Who can own this infantry unit.
    None,
    None,
    MPH_SLOW_ISH,
);

const CivilianDos3: [DoInfoStruct; DoType::COUNT] = [
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_READY
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_GUARD
    DoInfoStruct::new(0, 1, 1),    // DO_PRONE				// N/A
    DoInfoStruct::new(56, 6, 6),   // DO_WALK
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_WEAPON
    DoInfoStruct::new(0, 1, 1),    // DO_LIE_DOWN			// N/A
    DoInfoStruct::new(8, 6, 6),    // DO_CRAWL
    DoInfoStruct::new(0, 1, 1),    // DO_GET_UP			// N/A
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_PRONE
    DoInfoStruct::new(189, 10, 0), // DO_IDLE1
    DoInfoStruct::new(199, 6, 0),  // DO_IDLE2
    DoInfoStruct::new(104, 3, 0),  // DO_ON_GUARD
    DoInfoStruct::new(107, 7, 0),  // DO_FIGHT_READY
    DoInfoStruct::new(114, 2, 0),  // DO_PUNCH
    DoInfoStruct::new(116, 6, 0),  // DO_KICK
    DoInfoStruct::new(122, 2, 0),  // DO_PUNCH_HIT1
    DoInfoStruct::new(124, 4, 0),  // DO_PUNCH_HIT2
    DoInfoStruct::new(128, 4, 0),  // DO_PUNCH_DEATH
    DoInfoStruct::new(133, 2, 0),  // DO_KICK_HIT1
    DoInfoStruct::new(135, 4, 0),  // DO_KICK_HIT2
    DoInfoStruct::new(139, 5, 0),  // DO_KICK_DEATH
    DoInfoStruct::new(144, 3, 0),  // DO_READY_WEAPON
    DoInfoStruct::new(329, 8, 0),  // DO_GUN_DEATH
    DoInfoStruct::new(337, 8, 0),  // DO_EXPLOSION_DEATH
    DoInfoStruct::new(337, 8, 0),  // DO_EXPLOSION2_DEATH
    DoInfoStruct::new(345, 12, 0), // DO_GRENADE_DEATH
    DoInfoStruct::new(357, 18, 0), // DO_FIRE_DEATH
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE2			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE2			// N/A
    DoInfoStruct::new(199, 6, 0),  // DO_PULL_GUN
    DoInfoStruct::new(237, 40, 0), //	DO_PLEAD
    DoInfoStruct::new(277, 6, 0),  //	DO_PLEAD_DEATH
];

const C3: InfantryTypeClass = InfantryTypeClass::new(
    INFANTRY_C3,          // Infantry type number.
    TXT_C3,               // Translate name number for infantry type.
    "C3",                 // INI name for infantry.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    true,                 // Is this a female type?
    false,                // Is a leader type?
    false,                // Has crawling animation frames?
    true,                 // Is this a civlian?
    true,                 // Always use the given name for the infantry?
    true,                 // Is this a "fraidycat" run-away type infantry?
    false,                // Can this infantry type capture a building?
    false,                // Theater specific graphic image?
    0,                    // Number of shots it has (default).
    CivilianDos3,         // ptr to DO table
    2,                    // Frame of projectile launch.
    0,                    // Frame of projectile launch while prone.
    5,                    // Strength of infantry (in damage points).
    0,                    // Sight range.
    10,                   // Cost of infantry (in credits).
    99,                   // Scenario when they first appear.
    0,
    1, // Risk/Reward of this infantry unit.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::NEUTRAL.bits(), // Who can own this infantry unit.
    None,
    None,
    MPH_SLOW_ISH,
);

const CivilianDos4: [DoInfoStruct; DoType::COUNT] = [
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_READY
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_GUARD
    DoInfoStruct::new(0, 1, 1),    // DO_PRONE				// N/A
    DoInfoStruct::new(56, 6, 6),   // DO_WALK
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_WEAPON
    DoInfoStruct::new(0, 1, 1),    // DO_LIE_DOWN			// N/A
    DoInfoStruct::new(8, 6, 6),    // DO_CRAWL
    DoInfoStruct::new(0, 1, 1),    // DO_GET_UP			// N/A
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_PRONE
    DoInfoStruct::new(189, 10, 0), // DO_IDLE1
    DoInfoStruct::new(199, 6, 0),  // DO_IDLE2
    DoInfoStruct::new(104, 3, 0),  // DO_ON_GUARD
    DoInfoStruct::new(107, 7, 0),  // DO_FIGHT_READY
    DoInfoStruct::new(114, 2, 0),  // DO_PUNCH
    DoInfoStruct::new(116, 6, 0),  // DO_KICK
    DoInfoStruct::new(122, 2, 0),  // DO_PUNCH_HIT1
    DoInfoStruct::new(124, 4, 0),  // DO_PUNCH_HIT2
    DoInfoStruct::new(128, 4, 0),  // DO_PUNCH_DEATH
    DoInfoStruct::new(133, 2, 0),  // DO_KICK_HIT1
    DoInfoStruct::new(135, 4, 0),  // DO_KICK_HIT2
    DoInfoStruct::new(139, 5, 0),  // DO_KICK_DEATH
    DoInfoStruct::new(144, 3, 0),  // DO_READY_WEAPON
    DoInfoStruct::new(329, 8, 0),  // DO_GUN_DEATH
    DoInfoStruct::new(337, 8, 0),  // DO_EXPLOSION_DEATH
    DoInfoStruct::new(337, 8, 0),  // DO_EXPLOSION2_DEATH
    DoInfoStruct::new(345, 12, 0), // DO_GRENADE_DEATH
    DoInfoStruct::new(357, 18, 0), // DO_FIRE_DEATH
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE2			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE2			// N/A
    DoInfoStruct::new(199, 6, 0),  // DO_PULL_GUN
    DoInfoStruct::new(237, 40, 0), //	DO_PLEAD
    DoInfoStruct::new(277, 6, 0),  //	DO_PLEAD_DEATH
];

const C4: InfantryTypeClass = InfantryTypeClass::new(
    INFANTRY_C4,          // Infantry type number.
    TXT_C4,               // Translate name number for infantry type.
    "C4",                 // INI name for infantry.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    true,                 // Is this a female type?
    false,                // Is a leader type?
    false,                // Has crawling animation frames?
    true,                 // Is this a civlian?
    true,                 // Always use the given name for the infantry?
    true,                 // Is this a "fraidycat" run-away type infantry?
    false,                // Can this infantry type capture a building?
    false,                // Theater specific graphic image?
    0,                    // Number of shots it has (default).
    CivilianDos4,         // ptr to DO table
    2,                    // Frame of projectile launch.
    0,                    // Frame of projectile launch while prone.
    5,                    // Strength of infantry (in damage points).
    0,                    // Sight range.
    10,                   // Cost of infantry (in credits).
    99,                   // Scenario when they first appear.
    0,
    1, // Risk/Reward of this infantry unit.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::NEUTRAL.bits(), // Who can own this infantry unit.
    None,
    None,
    MPH_SLOW_ISH,
);

const CivilianDos5: [DoInfoStruct; DoType::COUNT] = [
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_READY
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_GUARD
    DoInfoStruct::new(0, 1, 1),    // DO_PRONE				// N/A
    DoInfoStruct::new(56, 6, 6),   // DO_WALK
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_WEAPON
    DoInfoStruct::new(0, 1, 1),    // DO_LIE_DOWN			// N/A
    DoInfoStruct::new(8, 6, 6),    // DO_CRAWL
    DoInfoStruct::new(0, 1, 1),    // DO_GET_UP			// N/A
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_PRONE
    DoInfoStruct::new(189, 10, 0), // DO_IDLE1
    DoInfoStruct::new(199, 6, 0),  // DO_IDLE2
    DoInfoStruct::new(104, 3, 0),  // DO_ON_GUARD
    DoInfoStruct::new(107, 7, 0),  // DO_FIGHT_READY
    DoInfoStruct::new(114, 2, 0),  // DO_PUNCH
    DoInfoStruct::new(116, 6, 0),  // DO_KICK
    DoInfoStruct::new(122, 2, 0),  // DO_PUNCH_HIT1
    DoInfoStruct::new(124, 4, 0),  // DO_PUNCH_HIT2
    DoInfoStruct::new(128, 4, 0),  // DO_PUNCH_DEATH
    DoInfoStruct::new(133, 2, 0),  // DO_KICK_HIT1
    DoInfoStruct::new(135, 4, 0),  // DO_KICK_HIT2
    DoInfoStruct::new(139, 5, 0),  // DO_KICK_DEATH
    DoInfoStruct::new(144, 3, 0),  // DO_READY_WEAPON
    DoInfoStruct::new(329, 8, 0),  // DO_GUN_DEATH
    DoInfoStruct::new(337, 8, 0),  // DO_EXPLOSION_DEATH
    DoInfoStruct::new(337, 8, 0),  // DO_EXPLOSION2_DEATH
    DoInfoStruct::new(345, 12, 0), // DO_GRENADE_DEATH
    DoInfoStruct::new(357, 18, 0), // DO_FIRE_DEATH
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE2			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE2			// N/A
    DoInfoStruct::new(199, 6, 0),  // DO_PULL_GUN
    DoInfoStruct::new(237, 40, 0), //	DO_PLEAD
    DoInfoStruct::new(277, 6, 0),  //	DO_PLEAD_DEATH
];

const C5: InfantryTypeClass = InfantryTypeClass::new(
    INFANTRY_C5,          // Infantry type number.
    TXT_C5,               // Translate name number for infantry type.
    "C5",                 // INI name for infantry.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Is this a female type?
    false,                // Is a leader type?
    false,                // Has crawling animation frames?
    true,                 // Is this a civlian?
    true,                 // Always use the given name for the infantry?
    true,                 // Is this a "fraidycat" run-away type infantry?
    false,                // Can this infantry type capture a building?
    false,                // Theater specific graphic image?
    0,                    // Number of shots it has (default).
    CivilianDos5,         // ptr to DO table
    2,                    // Frame of projectile launch.
    0,                    // Frame of projectile launch while prone.
    5,                    // Strength of infantry (in damage points).
    0,                    // Sight range.
    10,                   // Cost of infantry (in credits).
    99,                   // Scenario when they first appear.
    0,
    1, // Risk/Reward of this infantry unit.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::NEUTRAL.bits(), // Who can own this infantry unit.
    None,
    None,
    MPH_SLOW_ISH,
);

const CivilianDos6: [DoInfoStruct; DoType::COUNT] = [
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_READY
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_GUARD
    DoInfoStruct::new(0, 1, 1),    // DO_PRONE				// N/A
    DoInfoStruct::new(56, 6, 6),   // DO_WALK
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_WEAPON
    DoInfoStruct::new(0, 1, 1),    // DO_LIE_DOWN			// N/A
    DoInfoStruct::new(8, 6, 6),    // DO_CRAWL
    DoInfoStruct::new(0, 1, 1),    // DO_GET_UP			// N/A
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_PRONE
    DoInfoStruct::new(189, 10, 0), // DO_IDLE1
    DoInfoStruct::new(199, 6, 0),  // DO_IDLE2
    DoInfoStruct::new(104, 3, 0),  // DO_ON_GUARD
    DoInfoStruct::new(107, 7, 0),  // DO_FIGHT_READY
    DoInfoStruct::new(114, 2, 0),  // DO_PUNCH
    DoInfoStruct::new(116, 6, 0),  // DO_KICK
    DoInfoStruct::new(122, 2, 0),  // DO_PUNCH_HIT1
    DoInfoStruct::new(124, 4, 0),  // DO_PUNCH_HIT2
    DoInfoStruct::new(128, 4, 0),  // DO_PUNCH_DEATH
    DoInfoStruct::new(133, 2, 0),  // DO_KICK_HIT1
    DoInfoStruct::new(135, 4, 0),  // DO_KICK_HIT2
    DoInfoStruct::new(139, 5, 0),  // DO_KICK_DEATH
    DoInfoStruct::new(144, 3, 0),  // DO_READY_WEAPON
    DoInfoStruct::new(329, 8, 0),  // DO_GUN_DEATH
    DoInfoStruct::new(337, 8, 0),  // DO_EXPLOSION_DEATH
    DoInfoStruct::new(337, 8, 0),  // DO_EXPLOSION2_DEATH
    DoInfoStruct::new(345, 12, 0), // DO_GRENADE_DEATH
    DoInfoStruct::new(357, 18, 0), // DO_FIRE_DEATH
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE2			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE2			// N/A
    DoInfoStruct::new(199, 6, 0),  // DO_PULL_GUN
    DoInfoStruct::new(237, 40, 0), //	DO_PLEAD
    DoInfoStruct::new(277, 6, 0),  //	DO_PLEAD_DEATH
];

const C6: InfantryTypeClass = InfantryTypeClass::new(
    INFANTRY_C6,          // Infantry type number.
    TXT_C6,               // Translate name number for infantry type.
    "C6",                 // INI name for infantry.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Is this a female type?
    false,                // Is a leader type?
    false,                // Has crawling animation frames?
    true,                 // Is this a civlian?
    true,                 // Always use the given name for the infantry?
    true,                 // Is this a "fraidycat" run-away type infantry?
    false,                // Can this infantry type capture a building?
    false,                // Theater specific graphic image?
    0,                    // Number of shots it has (default).
    CivilianDos6,         // ptr to DO table
    2,                    // Frame of projectile launch.
    0,                    // Frame of projectile launch while prone.
    5,                    // Strength of infantry (in damage points).
    0,                    // Sight range.
    10,                   // Cost of infantry (in credits).
    99,                   // Scenario when they first appear.
    0,
    1, // Risk/Reward of this infantry unit.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::NEUTRAL.bits(), // Who can own this infantry unit.
    None,
    None,
    MPH_SLOW_ISH,
);

const CivilianDos7: [DoInfoStruct; DoType::COUNT] = [
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_READY
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_GUARD
    DoInfoStruct::new(0, 1, 1),    // DO_PRONE				// N/A
    DoInfoStruct::new(56, 6, 6),   // DO_WALK
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_WEAPON
    DoInfoStruct::new(0, 1, 1),    // DO_LIE_DOWN			// N/A
    DoInfoStruct::new(8, 6, 6),    // DO_CRAWL
    DoInfoStruct::new(0, 1, 1),    // DO_GET_UP			// N/A
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_PRONE
    DoInfoStruct::new(189, 10, 0), // DO_IDLE1
    DoInfoStruct::new(199, 6, 0),  // DO_IDLE2
    DoInfoStruct::new(104, 3, 0),  // DO_ON_GUARD
    DoInfoStruct::new(107, 7, 0),  // DO_FIGHT_READY
    DoInfoStruct::new(114, 2, 0),  // DO_PUNCH
    DoInfoStruct::new(116, 6, 0),  // DO_KICK
    DoInfoStruct::new(122, 2, 0),  // DO_PUNCH_HIT1
    DoInfoStruct::new(124, 4, 0),  // DO_PUNCH_HIT2
    DoInfoStruct::new(128, 4, 0),  // DO_PUNCH_DEATH
    DoInfoStruct::new(133, 2, 0),  // DO_KICK_HIT1
    DoInfoStruct::new(135, 4, 0),  // DO_KICK_HIT2
    DoInfoStruct::new(139, 5, 0),  // DO_KICK_DEATH
    DoInfoStruct::new(144, 3, 0),  // DO_READY_WEAPON
    DoInfoStruct::new(329, 8, 0),  // DO_GUN_DEATH
    DoInfoStruct::new(337, 8, 0),  // DO_EXPLOSION_DEATH
    DoInfoStruct::new(337, 8, 0),  // DO_EXPLOSION2_DEATH
    DoInfoStruct::new(345, 12, 0), // DO_GRENADE_DEATH
    DoInfoStruct::new(357, 18, 0), // DO_FIRE_DEATH
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE2			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE2			// N/A
    DoInfoStruct::new(199, 6, 0),  // DO_PULL_GUN
    DoInfoStruct::new(237, 40, 0), //	DO_PLEAD
    DoInfoStruct::new(277, 6, 0),  //	DO_PLEAD_DEATH
];

const C7: InfantryTypeClass = InfantryTypeClass::new(
    INFANTRY_C7,          // Infantry type number.
    TXT_C7,               // Translate name number for infantry type.
    "C7",                 // INI name for infantry.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Is this a female type?
    true,                 // Is a leader type?
    false,                // Has crawling animation frames?
    true,                 // Is this a civlian?
    true,                 // Always use the given name for the infantry?
    true,                 // Is this a "fraidycat" run-away type infantry?
    false,                // Can this infantry type capture a building?
    false,                // Theater specific graphic image?
    10,                   // Number of shots it has (default).
    CivilianDos7,         // ptr to DO table
    2,                    // Frame of projectile launch.
    0,                    // Frame of projectile launch while prone.
    5,                    // Strength of infantry (in damage points).
    0,                    // Sight range.
    10,                   // Cost of infantry (in credits).
    99,                   // Scenario when they first appear.
    0,
    1, // Risk/Reward of this infantry unit.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::NEUTRAL.bits(), // Who can own this infantry unit.
    Some(WEAPON_PISTOL),
    None,
    MPH_SLOW_ISH,
);

const CivilianDos8: [DoInfoStruct; DoType::COUNT] = [
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_READY
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_GUARD
    DoInfoStruct::new(0, 1, 1),    // DO_PRONE				// N/A
    DoInfoStruct::new(56, 6, 6),   // DO_WALK
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_WEAPON
    DoInfoStruct::new(0, 1, 1),    // DO_LIE_DOWN			// N/A
    DoInfoStruct::new(8, 6, 6),    // DO_CRAWL
    DoInfoStruct::new(0, 1, 1),    // DO_GET_UP			// N/A
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_PRONE
    DoInfoStruct::new(189, 10, 0), // DO_IDLE1
    DoInfoStruct::new(199, 6, 0),  // DO_IDLE2
    DoInfoStruct::new(104, 3, 0),  // DO_ON_GUARD
    DoInfoStruct::new(107, 7, 0),  // DO_FIGHT_READY
    DoInfoStruct::new(114, 2, 0),  // DO_PUNCH
    DoInfoStruct::new(116, 6, 0),  // DO_KICK
    DoInfoStruct::new(122, 2, 0),  // DO_PUNCH_HIT1
    DoInfoStruct::new(124, 4, 0),  // DO_PUNCH_HIT2
    DoInfoStruct::new(128, 4, 0),  // DO_PUNCH_DEATH
    DoInfoStruct::new(133, 2, 0),  // DO_KICK_HIT1
    DoInfoStruct::new(135, 4, 0),  // DO_KICK_HIT2
    DoInfoStruct::new(139, 5, 0),  // DO_KICK_DEATH
    DoInfoStruct::new(144, 3, 0),  // DO_READY_WEAPON
    DoInfoStruct::new(329, 8, 0),  // DO_GUN_DEATH
    DoInfoStruct::new(337, 8, 0),  // DO_EXPLOSION_DEATH
    DoInfoStruct::new(337, 8, 0),  // DO_EXPLOSION2_DEATH
    DoInfoStruct::new(345, 12, 0), // DO_GRENADE_DEATH
    DoInfoStruct::new(357, 18, 0), // DO_FIRE_DEATH
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE2			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE2			// N/A
    DoInfoStruct::new(199, 6, 0),  // DO_PULL_GUN
    DoInfoStruct::new(237, 40, 0), //	DO_PLEAD
    DoInfoStruct::new(277, 6, 0),  //	DO_PLEAD_DEATH
];

const C8: InfantryTypeClass = InfantryTypeClass::new(
    INFANTRY_C8,          // Infantry type number.
    TXT_C8,               // Translate name number for infantry type.
    "C8",                 // INI name for infantry.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Is this a female type?
    false,                // Is a leader type?
    false,                // Has crawling animation frames?
    true,                 // Is this a civlian?
    true,                 // Always use the given name for the infantry?
    true,                 // Is this a "fraidycat" run-away type infantry?
    false,                // Can this infantry type capture a building?
    false,                // Theater specific graphic image?
    0,                    // Number of shots it has (default).
    CivilianDos8,         // ptr to DO table
    2,                    // Frame of projectile launch.
    0,                    // Frame of projectile launch while prone.
    5,                    // Strength of infantry (in damage points).
    0,                    // Sight range.
    10,                   // Cost of infantry (in credits).
    99,                   // Scenario when they first appear.
    0,
    1, // Risk/Reward of this infantry unit.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::NEUTRAL.bits(), // Who can own this infantry unit.
    None,
    None,
    MPH_SLOW_ISH,
);

const CivilianDos9: [DoInfoStruct; DoType::COUNT] = [
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_READY
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_GUARD
    DoInfoStruct::new(0, 1, 1),    // DO_PRONE				// N/A
    DoInfoStruct::new(56, 6, 6),   // DO_WALK
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_WEAPON
    DoInfoStruct::new(0, 1, 1),    // DO_LIE_DOWN			// N/A
    DoInfoStruct::new(8, 6, 6),    // DO_CRAWL
    DoInfoStruct::new(0, 1, 1),    // DO_GET_UP			// N/A
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_PRONE
    DoInfoStruct::new(189, 10, 0), // DO_IDLE1
    DoInfoStruct::new(199, 6, 0),  // DO_IDLE2
    DoInfoStruct::new(104, 3, 0),  // DO_ON_GUARD
    DoInfoStruct::new(107, 7, 0),  // DO_FIGHT_READY
    DoInfoStruct::new(114, 2, 0),  // DO_PUNCH
    DoInfoStruct::new(116, 6, 0),  // DO_KICK
    DoInfoStruct::new(122, 2, 0),  // DO_PUNCH_HIT1
    DoInfoStruct::new(124, 4, 0),  // DO_PUNCH_HIT2
    DoInfoStruct::new(128, 4, 0),  // DO_PUNCH_DEATH
    DoInfoStruct::new(133, 2, 0),  // DO_KICK_HIT1
    DoInfoStruct::new(135, 4, 0),  // DO_KICK_HIT2
    DoInfoStruct::new(139, 5, 0),  // DO_KICK_DEATH
    DoInfoStruct::new(144, 3, 0),  // DO_READY_WEAPON
    DoInfoStruct::new(329, 8, 0),  // DO_GUN_DEATH
    DoInfoStruct::new(337, 8, 0),  // DO_EXPLOSION_DEATH
    DoInfoStruct::new(337, 8, 0),  // DO_EXPLOSION2_DEATH
    DoInfoStruct::new(345, 12, 0), // DO_GRENADE_DEATH
    DoInfoStruct::new(357, 18, 0), // DO_FIRE_DEATH
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE2			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE2			// N/A
    DoInfoStruct::new(199, 6, 0),  // DO_PULL_GUN
    DoInfoStruct::new(237, 40, 0), //	DO_PLEAD
    DoInfoStruct::new(277, 6, 0),  //	DO_PLEAD_DEATH
];

const C9: InfantryTypeClass = InfantryTypeClass::new(
    INFANTRY_C9,          // Infantry type number.
    TXT_C9,               // Translate name number for infantry type.
    "C9",                 // INI name for infantry.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Is this a female type?
    false,                // Is a leader type?
    false,                // Has crawling animation frames?
    true,                 // Is this a civlian?
    true,                 // Always use the given name for the infantry?
    true,                 // Is this a "fraidycat" run-away type infantry?
    false,                // Can this infantry type capture a building?
    false,                // Theater specific graphic image?
    0,                    // Number of shots it has (default).
    CivilianDos9,         // ptr to DO table
    2,                    // Frame of projectile launch.
    0,                    // Frame of projectile launch while prone.
    5,                    // Strength of infantry (in damage points).
    0,                    // Sight range.
    10,                   // Cost of infantry (in credits).
    99,                   // Scenario when they first appear.
    0,
    1, // Risk/Reward of this infantry unit.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::NEUTRAL.bits(), // Who can own this infantry unit.
    None,
    None,
    MPH_SLOW_ISH,
);

const NikoombaDos: [DoInfoStruct; DoType::COUNT] = [
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_READY
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_GUARD
    DoInfoStruct::new(0, 1, 1),    // DO_PRONE				// N/A
    DoInfoStruct::new(56, 6, 6),   // DO_WALK
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_WEAPON
    DoInfoStruct::new(0, 1, 1),    // DO_LIE_DOWN			// N/A
    DoInfoStruct::new(8, 6, 6),    // DO_CRAWL
    DoInfoStruct::new(0, 1, 1),    // DO_GET_UP			// N/A
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_PRONE
    DoInfoStruct::new(189, 10, 0), // DO_IDLE1
    DoInfoStruct::new(199, 6, 0),  // DO_IDLE2
    DoInfoStruct::new(104, 3, 0),  // DO_ON_GUARD
    DoInfoStruct::new(107, 7, 0),  // DO_FIGHT_READY
    DoInfoStruct::new(114, 2, 0),  // DO_PUNCH
    DoInfoStruct::new(116, 6, 0),  // DO_KICK
    DoInfoStruct::new(122, 2, 0),  // DO_PUNCH_HIT1
    DoInfoStruct::new(124, 4, 0),  // DO_PUNCH_HIT2
    DoInfoStruct::new(128, 4, 0),  // DO_PUNCH_DEATH
    DoInfoStruct::new(133, 2, 0),  // DO_KICK_HIT1
    DoInfoStruct::new(135, 4, 0),  // DO_KICK_HIT2
    DoInfoStruct::new(139, 5, 0),  // DO_KICK_DEATH
    DoInfoStruct::new(144, 3, 0),  // DO_READY_WEAPON
    DoInfoStruct::new(329, 8, 0),  // DO_GUN_DEATH
    DoInfoStruct::new(337, 8, 0),  // DO_EXPLOSION_DEATH
    DoInfoStruct::new(337, 8, 0),  // DO_EXPLOSION2_DEATH
    DoInfoStruct::new(345, 12, 0), // DO_GRENADE_DEATH
    DoInfoStruct::new(357, 18, 0), // DO_FIRE_DEATH
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE2			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE2			// N/A
    DoInfoStruct::new(199, 6, 0),  // DO_PULL_GUN
    DoInfoStruct::new(237, 40, 0), //	DO_PLEAD
    DoInfoStruct::new(277, 6, 0),  //	DO_PLEAD_DEATH
];

// Nikoomba
const C10: InfantryTypeClass = InfantryTypeClass::new(
    INFANTRY_C10,         // Infantry type number.
    TXT_C10,              // Translate name number for infantry type.
    "C10",                // INI name for infantry.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Is this a female type?
    false,                // Is a leader type?
    false,                // Has crawling animation frames?
    true,                 // Is this a civlian?
    true,                 // Always use the given name for the infantry?
    true,                 // Is this a "fraidycat" run-away type infantry?
    false,                // Can this infantry type capture a building?
    false,                // Theater specific graphic image?
    0,                    // Number of shots it has (default).
    NikoombaDos,          // ptr to DO table
    2,                    // Frame of projectile launch.
    0,                    // Frame of projectile launch while prone.
    50,                   // Strength of infantry (in damage points).
    0,                    // Sight range.
    10,                   // Cost of infantry (in credits).
    99,                   // Scenario when they first appear.
    0,
    1, // Risk/Reward of this infantry unit.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::NEUTRAL.bits(), // Who can own this infantry unit.
    None,
    None,
    MPH_SLOW_ISH,
);

const MoebiusDos: [DoInfoStruct; DoType::COUNT] = [
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_READY
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_GUARD
    DoInfoStruct::new(0, 1, 1),    // DO_PRONE				// N/A
    DoInfoStruct::new(56, 6, 6),   // DO_WALK
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_WEAPON
    DoInfoStruct::new(0, 1, 1),    // DO_LIE_DOWN			// N/A
    DoInfoStruct::new(8, 6, 6),    // DO_CRAWL
    DoInfoStruct::new(0, 1, 1),    // DO_GET_UP			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_FIRE_PRONE
    DoInfoStruct::new(104, 16, 0), // DO_IDLE1
    DoInfoStruct::new(120, 20, 0), // DO_IDLE2
    DoInfoStruct::new(0, 0, 0),    // DO_ON_GUARD
    DoInfoStruct::new(0, 0, 0),    // DO_FIGHT_READY
    DoInfoStruct::new(0, 0, 0),    // DO_PUNCH
    DoInfoStruct::new(0, 0, 0),    // DO_KICK
    DoInfoStruct::new(0, 0, 0),    // DO_PUNCH_HIT1
    DoInfoStruct::new(0, 0, 0),    // DO_PUNCH_HIT2
    DoInfoStruct::new(0, 0, 0),    // DO_PUNCH_DEATH
    DoInfoStruct::new(0, 0, 0),    // DO_KICK_HIT1
    DoInfoStruct::new(0, 0, 0),    // DO_KICK_HIT2
    DoInfoStruct::new(0, 0, 0),    // DO_KICK_DEATH
    DoInfoStruct::new(0, 0, 0),    // DO_READY_WEAPON
    DoInfoStruct::new(212, 8, 0),  // DO_GUN_DEATH
    DoInfoStruct::new(220, 8, 0),  // DO_EXPLOSION_DEATH
    DoInfoStruct::new(228, 12, 0), // DO_EXPLOSION2_DEATH
    DoInfoStruct::new(228, 12, 0), // DO_GRENADE_DEATH
    DoInfoStruct::new(240, 18, 0), // DO_FIRE_DEATH
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE2			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE2			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_PULL_GUN
    DoInfoStruct::new(120, 31, 0), //	DO_PLEAD
    DoInfoStruct::new(151, 14, 0), //	DO_PLEAD_DEATH
];

const Moebius: InfantryTypeClass = InfantryTypeClass::new(
    INFANTRY_MOEBIUS,     // Infantry type number.
    TXT_MOEBIUS,          // Translate name number for infantry type.
    "MOEBIUS",            // INI name for infantry.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Is this a female type?
    false,                // Is a leader type?
    false,                // Has crawling animation frames?
    true,                 // Is this a civlian?
    true,                 // Always use the given name for the infantry?
    true,                 // Is this a "fraidycat" run-away type infantry?
    false,                // Can this infantry type capture a building?
    false,                // Theater specific graphic image?
    0,                    // Number of shots it has (default).
    MoebiusDos,           // ptr to DO table
    0,                    // Frame of projectile launch.
    0,                    // Frame of projectile launch while prone.
    50,                   // Strength of infantry (in damage points).
    0,                    // Sight range.
    10,                   // Cost of infantry (in credits).
    99,                   // Scenario when they first appear.
    0,
    10, // Risk/Reward of this infantry unit.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::BAD.bits()
        | HOUSEF::NEUTRAL.bits(), // Who can own this infantry unit.
    None,
    None,
    MPH_SLOW_ISH,
);

const DelphiDos: [DoInfoStruct; DoType::COUNT] = [
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_READY
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_GUARD
    DoInfoStruct::new(0, 1, 1),    // DO_PRONE				// N/A
    DoInfoStruct::new(56, 6, 6),   // DO_WALK
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_WEAPON
    DoInfoStruct::new(0, 1, 1),    // DO_LIE_DOWN			// N/A
    DoInfoStruct::new(8, 6, 6),    // DO_CRAWL
    DoInfoStruct::new(0, 1, 1),    // DO_GET_UP			// N/A
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_PRONE
    DoInfoStruct::new(189, 10, 0), // DO_IDLE1
    DoInfoStruct::new(199, 6, 0),  // DO_IDLE2
    DoInfoStruct::new(104, 3, 0),  // DO_ON_GUARD
    DoInfoStruct::new(107, 7, 0),  // DO_FIGHT_READY
    DoInfoStruct::new(114, 2, 0),  // DO_PUNCH
    DoInfoStruct::new(116, 6, 0),  // DO_KICK
    DoInfoStruct::new(122, 2, 0),  // DO_PUNCH_HIT1
    DoInfoStruct::new(124, 4, 0),  // DO_PUNCH_HIT2
    DoInfoStruct::new(128, 4, 0),  // DO_PUNCH_DEATH
    DoInfoStruct::new(133, 2, 0),  // DO_KICK_HIT1
    DoInfoStruct::new(135, 4, 0),  // DO_KICK_HIT2
    DoInfoStruct::new(139, 5, 0),  // DO_KICK_DEATH
    DoInfoStruct::new(144, 3, 0),  // DO_READY_WEAPON
    DoInfoStruct::new(329, 8, 0),  // DO_GUN_DEATH
    DoInfoStruct::new(337, 8, 0),  // DO_EXPLOSION_DEATH
    DoInfoStruct::new(337, 8, 0),  // DO_EXPLOSION2_DEATH
    DoInfoStruct::new(345, 12, 0), // DO_GRENADE_DEATH
    DoInfoStruct::new(357, 18, 0), // DO_FIRE_DEATH
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE2			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE2			// N/A
    DoInfoStruct::new(199, 6, 0),  // DO_PULL_GUN
    DoInfoStruct::new(237, 40, 0), //	DO_PLEAD
    DoInfoStruct::new(277, 6, 0),  //	DO_PLEAD_DEATH
];

const Delphi: InfantryTypeClass = InfantryTypeClass::new(
    INFANTRY_DELPHI,      // Infantry type number.
    TXT_DELPHI,           // Translate name number for infantry type.
    "DELPHI",             // INI name for infantry.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Is this a female type?
    false,                // Is a leader type?
    false,                // Has crawling animation frames?
    true,                 // Is this a civlian?
    true,                 // Always use the given name for the infantry?
    true,                 // Is this a "fraidycat" run-away type infantry?
    false,                // Can this infantry type capture a building?
    false,                // Theater specific graphic image?
    10,                   // Number of shots it has (default).
    DelphiDos,            // ptr to DO table
    2,                    // Frame of projectile launch.
    0,                    // Frame of projectile launch while prone.
    25,                   // Strength of infantry (in damage points).
    0,                    // Sight range.
    10,                   // Cost of infantry (in credits).
    99,                   // Scenario when they first appear.
    0,
    0, // Risk/Reward of this infantry unit.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::NEUTRAL.bits(), // Who can own this infantry unit.
    Some(WEAPON_PISTOL),
    None,
    MPH_SLOW_ISH,
);

const DrChanDos: [DoInfoStruct; DoType::COUNT] = [
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_READY
    DoInfoStruct::new(0, 1, 1),    // DO_STAND_GUARD
    DoInfoStruct::new(0, 1, 1),    // DO_PRONE				// N/A
    DoInfoStruct::new(56, 6, 6),   // DO_WALK
    DoInfoStruct::new(205, 4, 4),  // DO_FIRE_WEAPON
    DoInfoStruct::new(0, 1, 1),    // DO_LIE_DOWN			// N/A
    DoInfoStruct::new(8, 6, 6),    // DO_CRAWL
    DoInfoStruct::new(0, 1, 1),    // DO_GET_UP			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_FIRE_PRONE
    DoInfoStruct::new(104, 16, 0), // DO_IDLE1
    DoInfoStruct::new(120, 20, 0), // DO_IDLE2
    DoInfoStruct::new(0, 0, 0),    // DO_ON_GUARD
    DoInfoStruct::new(0, 0, 0),    // DO_FIGHT_READY
    DoInfoStruct::new(0, 0, 0),    // DO_PUNCH
    DoInfoStruct::new(0, 0, 0),    // DO_KICK
    DoInfoStruct::new(0, 0, 0),    // DO_PUNCH_HIT1
    DoInfoStruct::new(0, 0, 0),    // DO_PUNCH_HIT2
    DoInfoStruct::new(0, 0, 0),    // DO_PUNCH_DEATH
    DoInfoStruct::new(0, 0, 0),    // DO_KICK_HIT1
    DoInfoStruct::new(0, 0, 0),    // DO_KICK_HIT2
    DoInfoStruct::new(0, 0, 0),    // DO_KICK_DEATH
    DoInfoStruct::new(0, 0, 0),    // DO_READY_WEAPON
    DoInfoStruct::new(212, 8, 0),  // DO_GUN_DEATH
    DoInfoStruct::new(220, 8, 0),  // DO_EXPLOSION_DEATH
    DoInfoStruct::new(228, 12, 0), // DO_EXPLOSION2_DEATH
    DoInfoStruct::new(228, 12, 0), // DO_GRENADE_DEATH
    DoInfoStruct::new(240, 18, 0), // DO_FIRE_DEATH
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE1			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_GESTURE2			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_SALUTE2			// N/A
    DoInfoStruct::new(0, 0, 0),    // DO_PULL_GUN
    DoInfoStruct::new(120, 31, 0), //	DO_PLEAD
    DoInfoStruct::new(151, 14, 0), //	DO_PLEAD_DEATH
];

const DrChan: InfantryTypeClass = InfantryTypeClass::new(
    INFANTRY_CHAN,        // Infantry type number.
    TXT_CHAN,             // Translate name number for infantry type.
    "CHAN",               // INI name for infantry.
    99,                   // Build level.
    STRUCTF::NONE.bits(), // Building prerequisite.
    false,                // Is this a female type?
    false,                // Is a leader type?
    false,                // Has crawling animation frames?
    true,                 // Is this a civlian?
    true,                 // Always use the given name for the infantry?
    true,                 // Is this a "fraidycat" run-away type infantry?
    false,                // Can this infantry type capture a building?
    false,                // Theater specific graphic image?
    10,                   // Number of shots it has (default).
    DrChanDos,            // ptr to DO table
    2,                    // Frame of projectile launch.
    0,                    // Frame of projectile launch while prone.
    25,                   // Strength of infantry (in damage points).
    0,                    // Sight range.
    10,                   // Cost of infantry (in credits).
    99,                   // Scenario when they first appear.
    0,
    1, // Risk/Reward of this infantry unit.
    HOUSEF::MULTI1.bits()
        | HOUSEF::MULTI2.bits()
        | HOUSEF::MULTI3.bits()
        | HOUSEF::MULTI4.bits()
        | HOUSEF::MULTI5.bits()
        | HOUSEF::MULTI6.bits()
        | HOUSEF::JP.bits()
        | HOUSEF::GOOD.bits()
        | HOUSEF::NEUTRAL.bits(), // Who can own this infantry unit.
    None,
    None,
    MPH_SLOW_ISH,
);

/// This is the array of classes to the static data associated with each
///	infantry type.
const BORROWS: [&InfantryTypeClass; InfantryType::COUNT] = [
    &E1, &E2, &E3, &E4, &E5, //	&E6,
    &E7, &Commando, &C1, &C2, &C3, &C4, &C5, &C6, &C7, &C8, &C9, &C10, &Moebius, &Delphi, &DrChan,
];

impl LookupByInternalControlName for InfantryTypeClass {
    type TypeEnum = InfantryType;

    /// Converts an ASCII name into an infantry type enum variant/number.
    /// This routine is used to convert the infantry ASCII name as specified into an infantry
    /// type number. This is called from the INI reader routine in the process if creating the
    /// infantry objects needed for the scenario.
    ///
    /// Returns with the infantry type number that corresponds to the infantry ASCII name
    /// specified. If no match could be found, then None is returned.
    ///
    /// Was From_Name in C++ code.
    fn lookup_type_enum_variant_by_internal_control_name(name: &str) -> Option<Self::TypeEnum> {
        for classid in Self::TypeEnum::iter() {
            if BORROWS[classid as usize].matches_internal_control_name(name) {
                return Some(classid);
            }
        }
        None
    }
}

impl InfantryTypeClass {
    pub const fn As_Reference(infantry_type: InfantryType) -> &'static Self {
        BORROWS[infantry_type as usize]
    }
}
