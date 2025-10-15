use crate::error::{CoreError, CoreResult};
use sofars::astro::atco13;

pub mod codec;
pub mod earth_orientation;
pub mod error;
pub mod math;
pub mod observer;
pub mod position;
pub mod star;
pub mod time;

pub fn position_star(
    star: &star::Star,
    observer: &observer::Observer,
    time: &time::Time,
    earth_orientation: &earth_orientation::EarthOrientation,
) -> CoreResult<position::Position> {
    match atco13(
        star.right_ascension,
        star.declination,
        star.proper_motion_right_ascension,
        star.proper_motion_declination,
        star.parallax,
        star.radial_velocity,
        time.utc1,
        time.utc2,
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
            Ok(position::Position {
                azimuth: azimuth_deg,
                altitude: altitude_deg,
            })
        }
        Err(_) => Err(CoreError::StarPositionDate),
    }
}
