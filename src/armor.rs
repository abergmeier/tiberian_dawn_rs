#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_variables
)]

///	The various armor types are best suited to defend against a limited
///	kind of warheads. The game strategy revolves around proper
///	combination of armor and weaponry. Each vehicle or building has armor
///	rated according to one of the following types.
pub enum ArmorType {
    //ARMOR_NONE,			// Vulnerable to SA and HE.
    ARMOR_WOOD,     // Vulnerable to HE and Fire.
    ARMOR_ALUMINUM, // Vulnerable to AP and SA.
    ARMOR_STEEL,    // Vulnerable to AP.
    ARMOR_CONCRETE, // Vulnerable to HE and AP.

                    //ARMOR_COUNT
}
