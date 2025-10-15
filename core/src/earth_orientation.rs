/// Source: https://datacenter.iers.org/data/latestVersion/bulletinA.txt
#[derive(Debug, Default)]
pub struct EarthOrientation {
    /// Polar motion (x & y) in radians (NOT radians/year!)
    /// => Coordinates of the Celestial Intermediate Pole relative to ITRS
    /// => Measured along meridians 0° and 90° west respectively
    /// => Can be set to (0.0, 0.0) for many applications
    pub polar_motion: (f64, f64),
    /// UT1-UTC difference in seconds
    /// => Keeps UT1-UTC within ±0.9s through leap seconds
    /// => Increases by exactly 1 second at each positive UTC leap second
    pub dut1: f64,
}

impl EarthOrientation {
    pub fn polar_motion_x(&self) -> f64 {
        self.polar_motion.0
    }

    pub fn polar_motion_y(&self) -> f64 {
        self.polar_motion.1
    }
}
