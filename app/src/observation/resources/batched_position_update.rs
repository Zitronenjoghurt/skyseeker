use bevy::prelude::Resource;

#[derive(Debug, Resource)]
pub struct BatchedPositionUpdate {
    pub current_index: usize,
    pub batch_size: usize,
}

impl Default for BatchedPositionUpdate {
    fn default() -> Self {
        Self {
            current_index: 0,
            batch_size: 50,
        }
    }
}
