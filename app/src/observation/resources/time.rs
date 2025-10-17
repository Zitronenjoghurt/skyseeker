use bevy::prelude::Resource;
use chrono::{DateTime, Utc};

#[derive(Debug, Resource)]
pub struct SimulationTime {
    pub live: bool,
    pub custom: DateTime<Utc>,
}

impl Default for SimulationTime {
    fn default() -> Self {
        Self {
            live: true,
            custom: Utc::now(),
        }
    }
}

impl SimulationTime {
    pub fn get_skyseeker_time(&self) -> skyseeker_core::position::time::Time {
        if self.live {
            skyseeker_core::position::time::Time::now()
        } else {
            skyseeker_core::position::time::Time::from_datetime(self.custom)
        }
    }
}
