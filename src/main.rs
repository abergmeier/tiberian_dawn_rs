mod aadata;
mod abstract_;
mod adata;
mod aircraft;
mod animation;
mod armor;
mod audio;
mod base;
mod bbdata;
mod bdata;
mod briefing;
mod building;
mod bullet;
mod cdata;
mod color;
mod coords;
mod direction;
mod display;
mod hdata;
mod heap;
mod house;
mod idata;
mod infantry;
mod ini;
mod map;
mod mdata;
mod mission;
mod movie;
mod object;
mod odata;
mod overlay;
mod player;
mod rtti;
mod scenario;
mod sdata;
mod smudge;
mod speed;
mod tdata;
mod team;
mod techno;
mod template;
mod terrain;
mod text;
mod thdata;
mod theater;
mod theme;
mod tmdata;
mod trigger;
mod udata;
mod unit;
mod weapon;

mod mdata_test;
mod team_test;
mod tmdata_test;

const ADVANCED: bool = false;
const PATCH: bool = false;
pub const REFRESH_EOL: u16 = 32767; // This number ends a refresh/occupy offset list.

fn main() {
    println!("Hello, world!");
}
