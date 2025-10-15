use skyseeker_core::earth_orientation::EarthOrientation;
use skyseeker_core::math::{
    angle_format_to_radians, arc_seconds_to_radians, time_format_to_radians,
};
use skyseeker_core::observer::Observer;
use skyseeker_core::position_star;
use skyseeker_core::star::Star;
use skyseeker_core::time::Time;

fn main() {
    let earth_orientation = EarthOrientation::default();

    let ra = time_format_to_radians(' ', 5, 55, 10.30536).unwrap();
    let dec = angle_format_to_radians('+', 7, 24, 25.4304).unwrap();
    let star = Star {
        hr: "2061".to_string(),
        name: Some("Betelgeuse".to_string()),
        common_name: None,
        bayer: None,
        bayer_full: None,
        constellation: None,
        notes: vec![],
        right_ascension: ra,
        declination: dec,
        proper_motion_right_ascension: arc_seconds_to_radians(27.54e-3) / dec.cos(),
        proper_motion_declination: arc_seconds_to_radians(11.3e-3),
        parallax: 6.55e-3,
        radial_velocity: 21.91,
        visual_magnitude: 0.0,
        b_v_color: None,
    };

    let observer = Observer {
        longitude: angle_format_to_radians('+', 14, 0, 0.0).unwrap(),
        latitude: angle_format_to_radians('+', 51, 0, 0.0).unwrap(),
        height: Some(300.0),
        pressure: None,
        temperature: None,
        humidity: None,
        wavelength: None,
    };

    let time = Time::from_utc(2025, 10, 14, 1, 30, 0.0).unwrap();

    let position = position_star(&star, &observer, &time, &earth_orientation).unwrap();
    println!("{:?}", position);
}
