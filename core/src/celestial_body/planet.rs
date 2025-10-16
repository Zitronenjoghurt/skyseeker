use crate::celestial_body::position_from_ecliptic_coords;
use crate::position::observer::Observer;
use crate::position::time::Time;
use crate::position::Position;
use bincode::{Decode, Encode};

#[derive(Debug, Copy, Clone, Encode, Decode)]
pub enum Planet {
    Mercury,
    Venus,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

impl Planet {
    pub fn id(&self) -> &str {
        match self {
            Planet::Mercury => "Mercury",
            Planet::Venus => "Venus",
            Planet::Mars => "Mars",
            Planet::Jupiter => "Jupiter",
            Planet::Saturn => "Saturn",
            Planet::Uranus => "Uranus",
            Planet::Neptune => "Neptune",
        }
    }

    pub fn get_astro_planet(&self) -> astro::planet::Planet {
        match self {
            Self::Mercury => astro::planet::Planet::Mercury,
            Self::Venus => astro::planet::Planet::Venus,
            Self::Mars => astro::planet::Planet::Mars,
            Self::Jupiter => astro::planet::Planet::Jupiter,
            Self::Saturn => astro::planet::Planet::Saturn,
            Self::Uranus => astro::planet::Planet::Uranus,
            Self::Neptune => astro::planet::Planet::Neptune,
        }
    }

    pub fn position(&self, observer: &Observer, time: &Time) -> Position {
        let (ecliptic_coords, _) = astro::planet::geocent_apprnt_ecl_coords(
            &self.get_astro_planet(),
            time.get_astro_julian_day(),
        );
        position_from_ecliptic_coords(ecliptic_coords, observer, time)
    }
}
