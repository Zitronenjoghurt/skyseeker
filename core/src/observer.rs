#[derive(Debug, Default)]
pub struct Observer {
    /// Observer's longitude in radians (east positive, range: -π to +π)
    /// => Geographic longitude on Earth's surface
    pub longitude: f64,
    /// Observer's geodetic latitude in radians (range: -π/2 to +π/2)
    /// => Geographic latitude, positive north of equator
    pub latitude: f64,
    /// Height above WGS84 ellipsoid in meters
    /// => Geodetic height (not sea level altitude)
    /// => For most purposes, sea level altitude is close enough
    /// => Used to calculate atmospheric refraction and parallax effects
    pub height: Option<f64>,
    /// Atmospheric pressure at observer in hPa (millibars)
    /// => Standard sea level: 1013.25 hPa
    /// => Critical for accurate refraction calculation (refraction is nearly proportional to pressure)
    /// => Lower pressure at high altitude = less atmospheric refraction
    pub pressure: Option<f64>,
    /// Ambient temperature at observer in degrees Celsius
    /// => Affects atmospheric refraction calculation
    /// => Warmer air = less dense = slightly less refraction
    /// => Typical range: -20°C to +40°C for most observations
    pub temperature: Option<f64>,
    /// Relative humidity as a fraction (0.0 = 0% to 1.0 = 100%)
    /// => Water vapor content in the atmosphere
    /// => Affects atmospheric refraction, especially at radio wavelengths
    /// => More important for precise work and longer wavelengths
    /// => 0.5 (50%) is a reasonable default for optical observations
    pub humidity: Option<f64>,
    /// Observing wavelength in micrometers
    /// => Determines optical vs radio refraction model (transition at ~100 μm ≈ 3000 GHz)
    /// => Visible light: 0.4-0.7 μm (violet to red)
    /// => Common choices: 0.55 μm (green, middle of visible spectrum)
    /// => Infrared: 1-25 μm, Radio: >100 μm
    /// => Refraction increases at shorter wavelengths (blue light bends more than red)
    pub wavelength: Option<f64>,
}

impl Observer {
    pub fn height(&self) -> f64 {
        self.height.unwrap_or(0.0)
    }

    pub fn pressure(&self) -> f64 {
        self.pressure.unwrap_or(1013.25)
    }

    pub fn temperature(&self) -> f64 {
        self.temperature.unwrap_or(15.0)
    }

    pub fn humidity(&self) -> f64 {
        self.humidity.unwrap_or(0.5)
    }

    pub fn wavelength(&self) -> f64 {
        self.wavelength.unwrap_or(0.55)
    }
}
