#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused_variables)]
use crate::smudge::SmudgeTypeClass;
use crate::smudge::SmudgeType::*;
use crate::text::IDs::*;

 
const Crater1: SmudgeTypeClass = SmudgeTypeClass::new (
 
     SMUDGE_CRATER1,
     "CR1",
     TXT_CRATER,
     1,1,							// Width and height of smudge (in icons).
     false,						// Is this a building bib?
     true							// Is this a crater smudge?
 );
const Crater2: SmudgeTypeClass = SmudgeTypeClass::new (
     SMUDGE_CRATER2,
     "CR2",
     TXT_CRATER,
     1,1,							// Width and height of smudge (in icons).
     false,						// Is this a building bib?
     true							// Is this a crater smudge?
 );
const Crater3: SmudgeTypeClass = SmudgeTypeClass::new (
     SMUDGE_CRATER3,
     "CR3",
     TXT_CRATER,
     1,1,							// Width and height of smudge (in icons).
     false,						// Is this a building bib?
     true							// Is this a crater smudge?
 );
const Crater4: SmudgeTypeClass = SmudgeTypeClass::new (
     SMUDGE_CRATER4,
     "CR4",
     TXT_CRATER,
     1,1,							// Width and height of smudge (in icons).
     false,						// Is this a building bib?
     true							// Is this a crater smudge?
 );
const Crater5: SmudgeTypeClass = SmudgeTypeClass::new (
     SMUDGE_CRATER5,
     "CR5",
     TXT_CRATER,
     1,1,							// Width and height of smudge (in icons).
     false,						// Is this a building bib?
     true							// Is this a crater smudge?
 );
const Crater6: SmudgeTypeClass = SmudgeTypeClass::new (
     SMUDGE_CRATER6,
     "CR6",
     TXT_CRATER,
     1,1,							// Width and height of smudge (in icons).
     false,						// Is this a building bib?
     true							// Is this a crater smudge?
 );
const Scorch1: SmudgeTypeClass = SmudgeTypeClass::new (
     SMUDGE_SCORCH1,
     "SC1",
     TXT_SCORCH,
     1,1,							// Width and height of smudge (in icons).
     false,						// Is this a building bib?
     false							// Is this a crater smudge?
 );
const Scorch2: SmudgeTypeClass = SmudgeTypeClass::new (
     SMUDGE_SCORCH2,
     "SC2",
     TXT_SCORCH,
     1,1,							// Width and height of smudge (in icons).
     false,						// Is this a building bib?
     false							// Is this a crater smudge?
 );
const Scorch3: SmudgeTypeClass = SmudgeTypeClass::new (
     SMUDGE_SCORCH3,
     "SC3",
     TXT_SCORCH,
     1,1,							// Width and height of smudge (in icons).
     false,						// Is this a building bib?
     false							// Is this a crater smudge?
 );
const Scorch4: SmudgeTypeClass = SmudgeTypeClass::new (
     SMUDGE_SCORCH4,
     "SC4",
     TXT_SCORCH,
     1,1,							// Width and height of smudge (in icons).
     false,						// Is this a building bib?
     false							// Is this a crater smudge?
 );
const Scorch5: SmudgeTypeClass = SmudgeTypeClass::new (
     SMUDGE_SCORCH5,
     "SC5",
     TXT_SCORCH,
     1,1,							// Width and height of smudge (in icons).
     false,						// Is this a building bib?
     false							// Is this a crater smudge?
 );
const Scorch6: SmudgeTypeClass = SmudgeTypeClass::new (
     SMUDGE_SCORCH6,
     "SC6",
     TXT_SCORCH,
     1,1,							// Width and height of smudge (in icons).
     false,						// Is this a building bib?
     false							// Is this a crater smudge?
 );
 
const Bibx1: SmudgeTypeClass = SmudgeTypeClass::new (
     SMUDGE_BIB1,
     "BIB1",
     TXT_BIB,
     4,2,							// Width and height of smudge (in icons).
     true,							// Is this a building bib?
     false							// Is this a crater smudge?
 );
const Bibx2: SmudgeTypeClass = SmudgeTypeClass::new (
     SMUDGE_BIB2,
     "BIB2",
     TXT_BIB,
     3,2,							// Width and height of smudge (in icons).
     true,							// Is this a building bib?
     false							// Is this a crater smudge?
 );
 
 /*
 ** The watcom code optimiser screws up the last constructor call. Making it 'volatile' reduces the
 ** level of optimisation enough for the problem not to manifest.
 */
 const Bibx3: SmudgeTypeClass = SmudgeTypeClass::new (
     SMUDGE_BIB3,
     "BIB3",
     TXT_BIB,
     2,2,							// Width and height of smudge (in icons).
     true,							// Is this a building bib?
     false							// Is this a crater smudge?
 );
 