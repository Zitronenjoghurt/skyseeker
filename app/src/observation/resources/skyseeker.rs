use bevy::prelude::Resource;
use skyseeker_core::codec::decode;

#[derive(Debug, Resource)]
pub struct Skyseeker(skyseeker_core::Skyseeker);

impl Skyseeker {
    pub fn setup() -> Self {
        let mut skyseeker = skyseeker_core::Skyseeker::new();
        skyseeker.load_standard_bodies();
        skyseeker.load_bodies(decode(include_bytes!("../../../../data/bsc5-stars.bin")).unwrap());
        Self(skyseeker)
    }

    pub fn get(&self) -> &skyseeker_core::Skyseeker {
        &self.0
    }
}
