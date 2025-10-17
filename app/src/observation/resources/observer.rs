use bevy::prelude::Resource;

#[derive(Debug, Default, Resource)]
pub struct Observer(skyseeker_core::position::observer::Observer);

impl Observer {
    pub fn get(&self) -> &skyseeker_core::position::observer::Observer {
        &self.0
    }
}
