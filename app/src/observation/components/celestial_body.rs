use bevy::prelude::Component;

#[derive(Debug, Component)]
pub struct CelestialBody(skyseeker_core::celestial_body::CelestialBody);

impl CelestialBody {
    pub fn new(body: skyseeker_core::celestial_body::CelestialBody) -> Self {
        Self(body)
    }

    pub fn get(&self) -> &skyseeker_core::celestial_body::CelestialBody {
        &self.0
    }
}
