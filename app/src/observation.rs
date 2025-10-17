use crate::observation::resources::batched_position_update::BatchedPositionUpdate;
use crate::observation::resources::observer::Observer;
use crate::observation::resources::skyseeker::Skyseeker;
use crate::observation::resources::time::SimulationTime;
use bevy::app::App;
use bevy::prelude::*;

mod bundles;
mod components;
mod resources;
mod systems;

pub struct ObservationPlugin;

impl Plugin for ObservationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Skyseeker::setup())
            .insert_resource(Observer::default())
            .insert_resource(SimulationTime::default())
            .insert_resource(BatchedPositionUpdate::default())
            .add_systems(
                Startup,
                (
                    systems::setup::setup_view,
                    systems::setup::spawn_horizon,
                    systems::setup::spawn_objects,
                ),
            )
            .add_systems(
                Update,
                (
                    systems::position::update_positions,
                    components::drag_look_camera::drag_look_system,
                ),
            );

        app.insert_resource(Time::<Fixed>::from_seconds(1.0));
    }
}
