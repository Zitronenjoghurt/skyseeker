use crate::celestial_body::planet::Planet;
use crate::celestial_body::star::Star;
use crate::error::CoreResult;
use crate::position::earth_orientation::EarthOrientation;
use crate::position::observer::Observer;
use crate::position::time::Time;
use crate::position::Position;
use astro::coords::EclPoint;
use bincode::{Decode, Encode};
use std::sync::Arc;

mod moon;
mod planet;
pub mod star;
mod sun;

#[derive(Debug, Clone, Encode, Decode)]
pub enum CelestialBody {
    Star(Arc<Star>),
    Planet(Planet),
    Moon,
    Sun,
}

impl CelestialBody {
    pub fn standard_bodies() -> Vec<Self> {
        vec![
            Self::Planet(Planet::Mercury),
            Self::Planet(Planet::Venus),
            Self::Planet(Planet::Mars),
            Self::Planet(Planet::Jupiter),
            Self::Planet(Planet::Saturn),
            Self::Planet(Planet::Uranus),
            Self::Planet(Planet::Neptune),
            Self::Moon,
            Self::Sun,
        ]
    }

    pub fn id(&self) -> &str {
        match self {
            CelestialBody::Star(star) => &star.id,
            CelestialBody::Planet(planet) => planet.id(),
            CelestialBody::Moon => "Moon",
            CelestialBody::Sun => "Sun",
        }
    }

    pub fn position(
        &self,
        observer: &Observer,
        time: &Time,
        earth_orientation: &EarthOrientation,
    ) -> CoreResult<Position> {
        match self {
            CelestialBody::Star(star) => star.position(observer, time, earth_orientation),
            CelestialBody::Planet(planet) => Ok(planet.position(observer, time)),
            CelestialBody::Moon => Ok(moon::moon_position(observer, time)),
            CelestialBody::Sun => Ok(sun::sun_position(observer, time)),
        }
    }

    pub fn visual_magnitude(&self) -> f64 {
        match self {
            Self::Star(star) => star.visual_magnitude,
            Self::Moon => -3.0,
            Self::Sun => -14.0,
            _ => 0.0,
        }
    }

    pub fn constellation(&self) -> Option<&str> {
        match self {
            Self::Star(star) => star.constellation.as_deref(),
            _ => None,
        }
    }

    pub fn is_star(&self) -> bool {
        matches!(self, Self::Star(_))
    }

    pub fn is_planet(&self) -> bool {
        matches!(self, Self::Planet(_))
    }

    pub fn is_moon(&self) -> bool {
        matches!(self, Self::Moon)
    }

    pub fn is_sun(&self) -> bool {
        matches!(self, Self::Sun)
    }
}

pub(crate) fn position_from_ecliptic_coords(
    ecliptic_coords: EclPoint,
    observer: &Observer,
    time: &Time,
) -> Position {
    let date = time.get_astro_date();
    let julian_day = astro::time::julian_day(&date);

    let (nut_in_long, nut_in_oblq) = astro::nutation::nutation(julian_day);
    let mean_obliquity = astro::ecliptic::mn_oblq_IAU(julian_day);
    let true_obliquity = mean_obliquity + nut_in_oblq;

    let right_ascension =
        astro::coords::asc_frm_ecl(ecliptic_coords.long, ecliptic_coords.lat, true_obliquity);
    let declination =
        astro::coords::dec_frm_ecl(ecliptic_coords.long, ecliptic_coords.lat, true_obliquity);

    let mean_sidereal = astro::time::mn_sidr(julian_day);
    let greenwich_sidereal = astro::time::apprnt_sidr(mean_sidereal, nut_in_long, true_obliquity);
    let local_sidereal = greenwich_sidereal + observer.longitude;
    let hour_angle = local_sidereal - right_ascension;

    let azimuth = astro::coords::az_frm_eq(hour_angle, declination, observer.latitude);
    let altitude = astro::coords::alt_frm_eq(hour_angle, declination, observer.latitude);

    let compass_azimuth = (azimuth.to_degrees() + 180.0) % 360.0;

    Position {
        azimuth: compass_azimuth,
        altitude: altitude.to_degrees(),
    }
}
