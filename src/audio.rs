#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_variables)]

/// These are the sound effect digitized sample file names.
#[repr(u8)]
pub enum VocType{
	//VOC_NONE=-1,

	VOC_RAMBO_PRESENT,//	"I've got a present for	ya"
	VOC_RAMBO_CMON,	//	"c'mon"
	VOC_RAMBO_UGOTIT,	//	"you got it"
	VOC_RAMBO_COMIN,	//	"keep 'em commin'"
	VOC_RAMBO_LAUGH,	//	"hahaha"
	VOC_RAMBO_LEFTY,	//	"that was left handed"
	VOC_RAMBO_NOPROB,	//	"no problem"
//	VOC_RAMBO_OHSH,	//	"oh shiiiiii...."
	VOC_RAMBO_ONIT,	//	"I'm on it"
	VOC_RAMBO_YELL,	//	"ahhhhhhh"
	VOC_RAMBO_ROCK,	//	"time to rock and roll"
	VOC_RAMBO_TUFF,	//	"real tuff guy"
	VOC_RAMBO_YEA,		//	"yea"
	VOC_RAMBO_YES,		//	"yes"
	VOC_RAMBO_YO,		//	"yo"

	VOC_GIRL_OKAY,		// "okay"
	VOC_GIRL_YEAH,		// "yeah?"
	VOC_GUY_OKAY,		//	"okay"
	VOC_GUY_YEAH,		// "yeah?"

	VOC_2DANGER,		//	"negative, too dangerous"
	VOC_ACKNOWL,		//	"acknowledged"
	VOC_AFFIRM,			//	"affirmative"
	VOC_AWAIT,			//	"awaiting orders"
//	VOC_BACKUP,			//	"send backup"
//	VOC_HELP,			//	"send help"
	VOC_MOVEOUT,		//	"movin' out"
	VOC_NEGATIVE,		//	"negative"
	VOC_NO_PROB,		//	"not a problem"
	VOC_READY,			//	"ready and waiting"
	VOC_REPORT,			//	"reporting"
	VOC_RIGHT_AWAY,	//	"right away sir"
	VOC_ROGER,			//	"roger"
//	VOC_SIR,				//	"sir?"
//	VOC_SQUAD,			//	"squad reporting"
//	VOC_PRACTICE,		//	"target practice"
	VOC_UGOTIT,			//	"you got it"
	VOC_UNIT,			//	"unit reporting"
	VOC_VEHIC,			//	"vehicle reporting"
	VOC_YESSIR,			//	"yes sir"

	VOC_BAZOOKA,		//	Gunfire
	VOC_BLEEP,			//	Clean metal bing
	VOC_BOMB1,			//	Crunchy parachute bomb type explosion
	VOC_BUTTON,			//	Dungeon Master button click
	VOC_RADAR_ON,		//	Elecronic static with beeps
	VOC_CONSTRUCTION,	//	construction sounds
	VOC_CRUMBLE,		//	muffled crumble sound
	VOC_FLAMER1,		//	flame thrower
	VOC_RIFLE,			//	rifle shot
	VOC_M60,				//	machine gun burst -- 6 rounds
	VOC_GUN20,			//	bat hitting heavy metal door
	VOC_M60A,			//	medium machine gun burst
	VOC_MINI,			//	mini gun burst
	VOC_RELOAD,			//	gun clip reload
	VOC_SLAM,			//	metal plates slamming together
	VOC_HVYGUN10,		//	loud sharp cannon
	VOC_ION_CANNON,	//	partical beam
	VOC_MGUN11,			//	alternate tripple burst
	VOC_MGUN2,			//	M-16 tripple burst
	VOC_NUKE_FIRE,		//	long missile sound
	VOC_NUKE_EXPLODE,	//	long but not loud explosion
	VOC_LASER,			//	humming star wars laser beam
	VOC_LASER_POWER,	//	warming up sound of star wars laser beam
	VOC_RADAR_OFF,		//	doom door slide
	VOC_SNIPER,			//	silenced rifle fire
	VOC_ROCKET1,		//	rocket launch variation #1
	VOC_ROCKET2,		//	rocket launch variation #2
	VOC_MOTOR,			//	dentists drill
	VOC_SCOLD,			//	cannot perform action feedback tone
	VOC_SIDEBAR_OPEN,	//	xylophone clink
	VOC_SIDEBAR_CLOSE,//	xylophone clink
	VOC_SQUISH2,		//	crushing infantry
	VOC_TANK1,			//	sharp tank fire with recoil
	VOC_TANK2,			//	sharp tank fire
	VOC_TANK3,			//	sharp tank fire
	VOC_TANK4,			//	big gun tank fire
	VOC_UP,				//	credits counting up
	VOC_DOWN,			//	credits counting down
	VOC_TARGET,			//	target sound
	VOC_SONAR,			//	sonar echo
	VOC_TOSS,			//	air swish
	VOC_CLOAK,			//	stealth tank
	VOC_BURN,			//	burning crackle
	VOC_TURRET,			//	muffled gunfire
	VOC_XPLOBIG4,		//	very long muffled explosion
	VOC_XPLOBIG6,		//	very long muffled explosion
	VOC_XPLOBIG7,		//	very long muffled explosion
	VOC_XPLODE,			//	long soft muffled explosion
	VOC_XPLOS,			//	short crunchy explosion
	VOC_XPLOSML2,		//	muffled mechanical explosion

	VOC_SCREAM1,		//	short infantry scream
	VOC_SCREAM3,		//	short infantry scream
	VOC_SCREAM4,		//	short infantry scream
	VOC_SCREAM5,		//	short infantry scream
	VOC_SCREAM6,		//	short infantry scream
	VOC_SCREAM7,		//	short infantry scream
	VOC_SCREAM10,		//	short infantry scream
	VOC_SCREAM11,		//	short infantry scream
	VOC_SCREAM12,		//	short infantry scream
	VOC_YELL1,			//	long infantry scream

	VOC_YES,				//	"Yes?"
	VOC_COMMANDER,		//	"Commander?"
	VOC_HELLO,			//	"Hello?"
	VOC_HMMM,			//	"Hmmm?"
//	VOC_PROCEED1,		//	"I will proceed, post haste."
//	VOC_PROCEED2,		//	"I will proceed, at once."
//	VOC_PROCEED3,		//	"I will proceed, immediately."
//	VOC_EXCELLENT1,	//	"That is an excellent plan."
//	VOC_EXCELLENT2,	//	"Yes, that is an excellent plan."
	VOC_EXCELLENT3,	//	"A wonderful plan."
//	VOC_EXCELLENT4,	//	"Asounding plan of action commander."
//	VOC_EXCELLENT5,	//	"Remarkable contrivance."
	VOC_OF_COURSE,		//	"Of course."
	VOC_YESYES,			//	"Yes yes yes."
	VOC_QUIP1,			//	"Mind the Tiberium."
//	VOC_QUIP2,			//	"A most remarkable  Metasequoia Glyptostroboides."
	VOC_THANKS,			//	"Thank you."

	VOC_CASHTURN,		// Sound of money being piled up.
	VOC_BLEEPY3,		// Clean computer bleep sound.

	VOC_DINOMOUT,		// Movin' out in dino-speak.
	VOC_DINOYES,		// Yes Sir in dino-speak.
	VOC_DINOATK1,		// Dino attack sound.
	VOC_DINODIE1,		// Dino die sound.

	//VOC_COUNT,
	//VOC_BUILD_SELECT=VOC_TARGET,
	//VOC_FIRST=0
}
