use crate::text::IDs;
use crate::thdata::_themes;
use strum::IntoEnumIterator;
use strum_macros::{EnumCount, EnumIter};

/// These are the themes that the game can play. They must be in exact
/// same order as specified in the CONQUER.TXT file as well as the filename
/// list located in the ThemeClass.
#[derive(Clone, Copy, EnumCount, EnumIter)]
#[repr(i8)]
pub enum ThemeType {
    THEME_PICK_ANOTHER = -2,
    //THEME_NONE=-1,
    THEME_AIRSTRIKE = 0,
    THEME_80MX,
    THEME_CHRG,
    THEME_CREP,
    THEME_DRIL,
    THEME_DRON,
    THEME_FIST,
    THEME_RECON,
    THEME_VOICE,
    THEME_HEAVYG,
    THEME_J1,
    THEME_JDI_V2,
    THEME_RADIO,
    THEME_RAIN,
    THEME_AOI,      // Act On Instinct
    THEME_CCTHANG,  //	C&C Thang
    THEME_DIE,      //	Die!!
    THEME_FWP,      //	Fight, Win, Prevail
    THEME_IND,      //	Industrial
    THEME_IND2,     //	Industrial2
    THEME_JUSTDOIT, //	Just Do It!
    THEME_LINEFIRE, //	In The Line Of Fire
    THEME_MARCH,    //	March To Your Doom
    THEME_MECHMAN,  // Mechanical Man
    THEME_NOMERCY,  //	No Mercy
    THEME_OTP,      //	On The Prowl
    THEME_PRP,      //	Prepare For Battle
    THEME_ROUT,     //	Reaching Out
    THEME_HEART,    //
    THEME_STOPTHEM, //	Stop Them
    THEME_TROUBLE,  //	Looks Like Trouble
    THEME_WARFARE,  //	Warfare
    THEME_BFEARED,  //	Enemies To Be Feared
    THEME_IAM,      // I Am
    THEME_WIN1,     //	Great Shot!
    THEME_MAP1,     // Map subliminal techno "theme".
    THEME_VALKYRIE, // Ride of the valkyries.

                    // THEME_COUNT,
                    // THEME_LAST=THEME_BFEARED,
                    // THEME_FIRST=0
}

pub struct ThemeControl {
    Name: &'static str, // Filename of score.
    Fullname: IDs,      // Text number for full score name.
    Scenario: u8,       // Scenario when it first becomes available.
    Duration: u32,      // Duration of theme in seconds.
    Normal: bool,       // Allowed in normal game play?
    Variation: bool,    // Is there a variation to the score?
    Repeat: bool,       // Always repeat this score?
    Available: bool,    // Is the score available?
}

impl ThemeControl {
    pub const fn new(
        name: &'static str,
        fullname: IDs,
        scenario: u8,
        duration: u32,
        normal: bool,
        variation: bool,
        repeat: bool,
        available: bool,
    ) -> Self {
        Self {
            Name: name,
            Fullname: fullname,
            Scenario: scenario,
            Duration: duration,
            Normal: normal,
            Variation: variation,
            Repeat: repeat,
            Available: available,
        }
    }
}

pub type TransitTheme = Option<ThemeType>;

pub struct ThemeClass {}

impl ThemeClass {
    /// Determines theme number from specified name.
    /// Use this routine to convert a name (either the base filename of the theme, or a partial
    /// substring of the full name) into the matching ThemeType value. Typical use of this is
    /// when parsing the INI file for theme control values.
    /// WARNINGS: If a filename is specified the comparison may be case insensitive. When scanning
    /// the full theme name, the comparison is case sensitive.
    pub fn From_Name(name: &str) -> Option<ThemeType> {
        if name.is_empty() {
            return None;
        }

        // First search for an exact name match with the filename
        // of the theme. This is guaranteed to be unique.
        for theme in ThemeType::iter() {
            if _themes[theme].Name.eq_ignore_ascii_case(name) {
                return Some(theme);
            }
        }

        // If the filename scan failed to find a match, then scan for
        // a substring within the full name of the score. This might
        // yeild a match, but is not guaranteed to be unique.
        for theme in ThemeType::iter() {
            unimplemented!("Finish this code");
            // if (strstr(Text_String(_themes[theme].Fullname), name) != NULL) {
            //     return (theme);
            // }
        }

        return None;
    }
}
