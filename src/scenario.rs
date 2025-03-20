#![allow(dead_code, non_snake_case, non_upper_case_globals, unused_variables)]

use crate::{ini::Ini, team::TeamTypeClass};

pub struct Scenario {
    movies: Movies,
    /// This is the list of BuildingTypes that define the AI's base.
    base: BaseClass,
    theme: ThemeClass,
    /// Used to be ScenarioCRC in C++ code
    //CRC: crc::Crc<u64>,

    Units: TFixedIHeapClass<UnitClass>,
    Factories: TFixedIHeapClass<FactoryClass>,
    Terrains: TFixedIHeapClass<TerrainClass>,
    Templates: TFixedIHeapClass<TemplateClass>,
    Smudges: TFixedIHeapClass<SmudgeClass>,
    Overlays: TFixedIHeapClass<OverlayClass>,
    Infantry: TFixedIHeapClass<InfantryClass>,
    Bullets: TFixedIHeapClass<BulletClass>,
    Buildings: TFixedIHeapClass<BuildingClass>,
    Anims: TFixedIHeapClass<AnimClass>,
    Aircraft: TFixedIHeapClass<AircraftClass>,
    Triggers: TFixedIHeapClass<TriggerClass>,
    TeamTypes: TFixedIHeapClass<TeamTypeClass>,
    Teams: TFixedIHeapClass<TeamClass>,
    Houses: TFixedIHeapClass<HouseClass>,
}

impl Scenario {
    /// Read in the scenario INI file. This routine only sets the game
    /// globals with that data that is explicitly defined in the INI file.
    /// The remaining necessary interpolated data is generated elsewhere.
    /// Was Read_Scenario_Ini in C++
    fn try_from(root: Ini, fresh: bool) -> Self {
        
    //     char *buffer;				// Scenario.ini staging buffer pointer.
    //     char fname[_MAX_FNAME+_MAX_EXT];			// full INI filename
    //     char buf[128];				// Working string staging buffer.
    //     int rndmax;
    //     int rndmin;
    //     int len;
    //     unsigned char val;
    
    //     ScenarioInit++;
    
    //     /*
    //     **	Fetch working pointer to the INI staging buffer. Make sure that the buffer
    //     **	is cleared out before proceeding.  (Don't use the HidPage for this, since
    //     **	the HidPage may be needed for various uncompressions during the INI
    //     **	parsing.)
    //     */
    // let buffer = [0; 1024];
    //     buffer = (char *)_ShapeBuffer;
    //     memset(buffer, '\0', _ShapeBufferSize);
    
    //     if (fresh) {
    //         Clear_Scenario();
    //     }
    
    //     /*
    //     ** If we are not dealing with scenario 1, or a multi player scenario
    //     ** then make sure the correct disk is in the drive.
    //     */
    //     if (RequiredCD != -2) {
    //         if (Scenario >= 20 && Scenario <60 && GameToPlay == GAME_NORMAL) {
    //             RequiredCD = 2;
    //         } else {
    //             if (Scenario != 1) {
    //                 if (Scenario >=60){
    //                     RequiredCD = -1;
    //                 }else{
    //                     switch (ScenPlayer) {
    //                         case SCEN_PLAYER_GDI:
    //                             RequiredCD = 0;
    //                             break;
    //                         case SCEN_PLAYER_NOD:
    //                             RequiredCD = 1;
    //                             break;
    //                         default:
    //                             RequiredCD = -1;
    //                             break;
    //                     }
    //                 }
    //             } else {
    //                 RequiredCD = -1;
    //             }
    //         }
    //     }
    //     if (!Force_CD_Available(RequiredCD)) {
    //         Prog_End();
    //         exit(EXIT_FAILURE);
    //     }
    
    //     /*
    //     **	Create scenario filename and read the file.
    //     */
    
    //     sprintf(fname,"%s.INI",root);
    let ini = IniProfile::read_from_file(fname);
    //     CCFileClass file(fname);
    //     if (!file.Is_Available()) {
    //         return(false);
    //     } else {
    //         file.Read(buffer, _ShapeBufferSize-1);
    //     }
    
    //     /*
    //     ** Init the Scenario CRC value
    //     */
    //     ScenarioCRC = 0;
    //     len = strlen(buffer);
    //     for (int i = 0; i < len; i++) {
    //         val = (unsigned char)buffer[i];
    // #ifndef DEMO
    //         Add_CRC(&ScenarioCRC, (unsigned long)val);
    // #endif
    //     }
        Self{
            /*
            **	Fetch the appropriate movie names from the INI file.
             */
            movies: Movies::try_from(ini)?,
        }
    
    //     /*
    //     **	For single-player scenarios, 'BuildLevel' is the scenario number.
    //     **	This must be set before any buildings are created (if a factory is created,
    //     **	it needs to know the BuildLevel for the sidebar.)
    //     */
    //     if (GameToPlay == GAME_NORMAL) {
    // #ifdef NEWMENU
    //         if (Scenario <= 15) {
    //             BuildLevel = Scenario;
    //         } else {
    //             BuildLevel = WWGetPrivateProfileInt("Basic", "BuildLevel", Scenario, buffer);
    //         }
    // #else
    //         BuildLevel = Scenario;
    // #endif
    //     }
    
    //     /*
    //     **	Jurassic scenarios are allowed to build the full multiplayer set
    //     **	of objects.
    //     */
    //     if (Special.IsJurassic && AreThingiesEnabled) {
    //         BuildLevel = 98;
    //     }
    
        /*
        **	Fetch the transition theme for this scenario.
        */
        // TransitTheme = THEME_NONE;
        // WWGetPrivateProfileString("Basic", "Theme", "No Theme", buf, sizeof(buf), buffer);
        // TransitTheme = Theme.From_Name(buf);
    
        /*
        **	Read in the team-type data. The team types must be created before any
        **	triggers can be created.
        */
        TeamTypeClass::try_from(ini)?;
    
        /*
        **	Read in the specific information for each of the house types.  This creates
        **	the houses of different types.
        */
        HouseClass::try_from(ini)?;
    
    //     /*
    //     **	Assign PlayerPtr by reading the player's house from the INI;
    //     **	Must be done before any TechnoClass objects are created.
    //     */
    // //	if (GameToPlay == GAME_NORMAL && (ScenPlayer == SCEN_PLAYER_GDI || ScenPlayer == SCEN_PLAYER_NOD)) {
    //     if (GameToPlay == GAME_NORMAL) {
    //         WWGetPrivateProfileString("Basic", "Player", "GoodGuy", buf, 127, buffer);
    //         CarryOverPercent = WWGetPrivateProfileInt("Basic", "CarryOverMoney", 100, buffer);
    //         CarryOverPercent = Cardinal_To_Fixed(100, CarryOverPercent);
    //         CarryOverCap = WWGetPrivateProfileInt("Basic", "CarryOverCap", -1, buffer);
    
    //         PlayerPtr = HouseClass::As_Pointer(HouseTypeClass::From_Name(buf));
    //         PlayerPtr->IsHuman = true;
    //         int carryover;
    //         if (CarryOverCap != -1) {
    //             carryover = MIN(Fixed_To_Cardinal(CarryOverMoney, CarryOverPercent), CarryOverCap);
    //         } else {
    //             carryover = Fixed_To_Cardinal(CarryOverMoney, CarryOverPercent);
    //         }
    //         PlayerPtr->Credits += carryover;
    //         PlayerPtr->InitialCredits += carryover;
    
    //         if (Special.IsJurassic) {
    //             PlayerPtr->ActLike = Whom;
    //         }
    //     } else {
    
    // #ifdef OBSOLETE
    //         if (GameToPlay==GAME_NORMAL && ScenPlayer==SCEN_PLAYER_JP) {
    //             PlayerPtr = HouseClass::As_Pointer(HOUSE_MULTI4);
    //             PlayerPtr->IsHuman = true;
    //             PlayerPtr->Credits += CarryOverMoney;
    //             PlayerPtr->InitialCredits += CarryOverMoney;
    //             PlayerPtr->ActLike = Whom;
    //         } else {
    //             Assign_Houses();
    //         }
    // #endif
    //         Assign_Houses();
    //     }
    
    //     /*
    //     **	Read in the trigger data. The triggers must be created before any other
    //     **	objects can be initialized.
    //     */
    TriggerClass::try_from(ini)?;
    
        /*
        **	Read in the map control values. This includes dimensions
        **	as well as theater information.
        */
        Map::try_from(ini)?;
    
    //     /*
    //     **	Attempt to read the map's binary image file; if fails, read the
    //     **	template data from the INI, for backward compatibility
    //     */
    //     if (fresh) {
    //         if (!Map.Read_Binary(root, &ScenarioCRC)) {
    //             TemplateClass::Read_INI(buffer);
    //         }
    //     }
    //     Call_Back();
    
        /*
        **	Read in and place the 3D terrain objects.
        */
        TerrainClass::try_from(ini);
    
        /*
        **	Read in and place the units (all sides).
        */
        UnitClass::try_from(ini);
    
        /*
        **	Read in and place the infantry units (all sides).
        */
        InfantryClass::try_from(ini);
    
        /*
        **	Read in and place all the buildings on the map.
        */
        BuildingClass::try_from(ini);
    
        /*
        **	Read in the AI's base information.
        */
        base = BaseClass::try_from(ini);
    
        /*
        **	Read in any normal overlay objects.
        */
        OverlayClass::try_from(ini);
    
        //
        SmudgeClass::try_from(ini);


	    // Read in any briefing text.
        Briefing::try_from(ini);

        // Fetch the transition theme for this scenario.
        TransitTheme: {
            let def = String::from("No Theme");
            let name = section.get("Theme").unwrap_or(def.as_str());
            ThemeClass::From_Name(name)
        }

    //     /*
    //     **	Perform a final overpass of the map. This handles smoothing of certain
    //     **	types of terrain (tiberium).
    //     */
    //     Map.Overpass();
    //     Call_Back();
    
    //     /*
    //     **	Multi-player last-minute fixups:
    //     **	- If computer players are disabled, remove all computer-owned houses
    //     ** - Otherwise, set MPlayerBlitz to 0 or 1, randomly
    //     **	- If bases are disabled, create the scenario dynamically
    //     **	- Remove any flag spot overlays lying around
    //     **	- If capture-the-flag is enabled, assign flags to cells.
    //     */
    //     if (GameToPlay != GAME_NORMAL || ScenPlayer == SCEN_PLAYER_2PLAYER ||
    //         ScenPlayer == SCEN_PLAYER_MPLAYER) {
    
    //         /*
    //         **	If Ghosts are disabled and we're not editing, remove computer players
    //         **	(Must be done after all objects are read in from the INI)
    //         */
    //         if (!MPlayerGhosts && !Debug_Map) {
    //             Remove_AI_Players();
    //         } else {
    
    //             /*
    //             ** If Ghosts are on, set up their houses for blitzing the humans
    //             */
    //             MPlayerBlitz = IRandom (0,1);					// 1 = computer will blitz
    //             if (MPlayerBlitz) {
    //                 if (MPlayerBases) {
    //                     rndmax = 14000;
    //                     rndmin = 10000;
    //                 } else {
    //                     rndmax = 8000;
    //                     rndmin = 4000;
    //                 }
    
    //                 for (i = 0; i < MPlayerMax; i++) {
    //                     HousesType house = (HousesType)(i + (int)HOUSE_MULTI1);
    //                     HouseClass *housep = HouseClass::As_Pointer (house);
    //                     housep->BlitzTime = IRandom (rndmin,rndmax);
    //                 }
    
    //             }
    //         }
    
    //         /*
    //         **	Units must be created for each house.  If bases are ON, this routine
    //         **	will create an MCV along with the units; otherwise, it will just create
    //         **	a whole bunch of units.  MPlayerUnitCount is the total # of units
    //         **	to create.
    //         */
    //         if (!Debug_Map) {
    //             int save_init = ScenarioInit;			// turn ScenarioInit off
    //             ScenarioInit = 0;
    //             Create_Units();
    //             ScenarioInit = save_init;				// turn ScenarioInit back on
    //         }
    
    //         /*
    //         **	Place crates if MPlayerGoodies is on.
    //         */
    //         if (MPlayerGoodies) {
    //             for (int index = 0; index < MPlayerCount; index++) {
    //                 Map.Place_Random_Crate();
    //             }
    //         }
    
    //         /*
    //         **	Compute my starting location as the average Coord of all my stuff.
    //         */
    //         Map.Compute_Start_Pos();
    //     }
    
    //     /*
    //     **	Return with flag saying that the scenario file was read.
    //     */
    //     ScenarioInit--;
    //     return(true);
        Self{}
   
    }
}
