use crate::error::{CoreError, CoreResult};
use crate::position::Position;
use position::{earth_orientation, observer, time};
use std::collections::HashMap;

pub mod celestial_body;
pub mod codec;
pub mod error;
pub mod math;
pub mod position;

#[derive(Debug, Default)]
pub struct Skyseeker {
    pub bodies_by_id: HashMap<String, celestial_body::CelestialBody>,
}

impl Skyseeker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_body(&mut self, body: celestial_body::CelestialBody) {
        let id = body.id().to_string();
        self.bodies_by_id.insert(id, body);
    }

    pub fn load_bodies(&mut self, new_bodies: Vec<celestial_body::CelestialBody>) {
        new_bodies.into_iter().for_each(|new_body| {
            self.load_body(new_body);
        });
    }

    pub fn load_standard_bodies(&mut self) {
        self.load_bodies(celestial_body::CelestialBody::standard_bodies());
    }

    pub fn get_body(&self, body_id: impl AsRef<str>) -> Option<&celestial_body::CelestialBody> {
        self.bodies_by_id.get(body_id.as_ref())
    }

    pub fn position(
        &self,
        body_id: impl AsRef<str>,
        observer: &observer::Observer,
        time: &time::Time,
        earth_orientation: &earth_orientation::EarthOrientation,
    ) -> CoreResult<Position> {
        let Some(body) = self.get_body(&body_id) else {
            return Err(CoreError::BodyNotFound(body_id.as_ref().to_string()));
        };
        body.position(observer, time, earth_orientation)
    }
}
