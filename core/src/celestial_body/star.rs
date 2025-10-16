use crate::error::{CoreError, CoreResult};
use crate::position::earth_orientation::EarthOrientation;
use crate::position::observer::Observer;
use crate::position::time::Time;
use crate::position::Position;
use bincode::{Decode, Encode};
use sofars::astro::atco13;

/// Data sources:
/// - https://simbad.cds.unistra.fr/simbad/sim-id?Ident=Betelgeuse&NbIdent=1&Radius=2&Radius.unit=arcmin&submit=submit+id
#[derive(Debug, Encode, Decode)]
pub struct Star {
    pub id: String,
    /// Harvard Revised Number = Bright Star Number
    pub hr: Option<u16>,
    pub name: Option<String>,
    pub common_name: Option<String>,
    pub bayer: Option<String>,
    pub bayer_full: Option<String>,
    pub constellation: Option<String>,
    pub notes: Vec<(String, String)>,
    /// Right ascension at J2000.0 epoch in radians
    /// => Celestial longitude, measured eastward from the vernal equinox (0 to 2π)
    /// => Like longitude on Earth's surface, but on the celestial sphere
    pub right_ascension: f64,
    /// Declination at J2000.0 epoch in radians
    /// => Celestial latitude, angular distance north (+) or south (-) of celestial equator
    /// => Range: -π/2 (south pole) to +π/2 (north pole)
    pub declination: f64,
    /// Proper motion in right ascension in radians/year
    /// => Accounts for the star's actual motion through space perpendicular to our line of sight
    pub proper_motion_right_ascension: f64,
    /// Proper motion in declination in radians/year
    /// => Together with proper_motion_ra, describes the star's apparent motion across the sky
    pub proper_motion_declination: f64,
    /// Parallax in arcseconds
    /// => Distance indicator: parallax = 1/distance_in_parsecs
    /// => The apparent shift in position as Earth orbits the Sun
    /// => Smaller parallax = more distant star (e.g., 0.005" ≈ 200 parsecs ≈ 650 light-years)
    pub parallax: f64,
    /// Heliocentric radial velocity in km/s (positive = receding from Sun, negative = approaching Sun)
    /// => Movement along the line of sight relative to the Sun
    pub radial_velocity: f64,
    /// Visual magnitude in mag
    /// => How bright the star appears in the night sky
    pub visual_magnitude: f64,
    /// B-V color in the UBV system
    pub b_v_color: Option<f64>,
}

impl Star {
    pub fn position(
        &self,
        observer: &Observer,
        time: &Time,
        earth_orientation: &EarthOrientation,
    ) -> CoreResult<Position> {
        let (utc1, utc2) = time.get_double_julian()?;

        match atco13(
            self.right_ascension,
            self.declination,
            self.proper_motion_right_ascension,
            self.proper_motion_declination,
            self.parallax,
            self.radial_velocity,
            utc1,
            utc2,
            earth_orientation.dut1,
            observer.longitude,
            observer.latitude,
            observer.height(),
            earth_orientation.polar_motion_x(),
            earth_orientation.polar_motion_y(),
            observer.pressure(),
            observer.temperature(),
            observer.humidity(),
            observer.wavelength(),
        ) {
            Ok((azimuth, zenith_dist, _, _, _, _)) => {
                let azimuth_deg = azimuth.to_degrees();
                let zenith_dist_deg = zenith_dist.to_degrees();
                let altitude_deg = 90.0 - zenith_dist_deg;
                Ok(Position {
                    azimuth: azimuth_deg,
                    altitude: altitude_deg,
                })
            }
            Err(_) => Err(CoreError::StarPositionDate),
        }
    }
}
