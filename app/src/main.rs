use skyseeker_core::codec::decode;
use skyseeker_core::math::angle_format_to_radians;
use skyseeker_core::position::earth_orientation::EarthOrientation;
use skyseeker_core::position::observer::Observer;
use skyseeker_core::position::time::Time;

fn main() {
    let mut skyseeker = skyseeker_core::Skyseeker::new();
    skyseeker.load_standard_bodies();
    skyseeker.load_bodies(decode(include_bytes!("../../data/bsc5-stars.bin")).unwrap());

    let observer = Observer {
        longitude: angle_format_to_radians('+', 14, 0, 0.0).unwrap(),
        latitude: angle_format_to_radians('+', 51, 0, 0.0).unwrap(),
        height: Some(300.0),
        pressure: None,
        temperature: None,
        humidity: None,
        wavelength: None,
    };

    let body = skyseeker.get_body("HR 5340").unwrap();
    let position = body
        .position(&observer, &Time::now(), &EarthOrientation::default())
        .unwrap();

    println!("{:?}", position);
}
