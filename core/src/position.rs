pub mod earth_orientation;
pub mod observer;
pub mod time;

/// The position of an object in the sky
#[derive(Debug)]
pub struct Position {
    /// Azimuth in degrees
    /// => Compass direction: 0°=N, 90°=E, 180°=S, 270°=W
    pub azimuth: f64,
    /// Altitude in degrees
    /// => Angle above horizon: 0°=horizon, 90°=overhead
    pub altitude: f64,
}
