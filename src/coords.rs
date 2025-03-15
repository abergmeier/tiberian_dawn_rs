///	These are custom C&C specific types. The CELL is used for map coordinate
///	with cell resolution. The COORD type is used for map coordinates that
///	have a lepton resolution.
pub type COORDINATE = u64;
pub type CELL = i16;

macro_rules! XYP_COORD {
    ($x:expr, $y:expr) => {
        ($x as u64 * ICON_LEPTON_W as u64) / CELL_PIXEL_W as u64 + ((($y as u64 * ICON_LEPTON_H as u64) / CELL_PIXEL_H as u64) << 16)
    };
}

pub(crate) use XYP_COORD;
