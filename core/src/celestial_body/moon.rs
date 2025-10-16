use crate::celestial_body::position_from_ecliptic_coords;
use crate::position::observer::Observer;
use crate::position::time::Time;
use crate::position::Position;

pub fn moon_position(observer: &Observer, time: &Time) -> Position {
    let (ecliptic_coords, _) = astro::lunar::geocent_ecl_pos(time.get_astro_julian_day());
    position_from_ecliptic_coords(ecliptic_coords, observer, time)
}
