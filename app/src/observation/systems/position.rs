use crate::observation::components::celestial_body::CelestialBody;
use crate::observation::resources::batched_position_update::BatchedPositionUpdate;
use crate::observation::resources::observer::Observer;
use crate::observation::resources::skyseeker::Skyseeker;
use crate::observation::resources::time::SimulationTime;
use bevy::prelude::{Query, Res, ResMut, Transform, Vec3};

pub fn update_positions(
    mut query: Query<(&CelestialBody, &mut Transform)>,
    mut batched_update: ResMut<BatchedPositionUpdate>,
    res_observer: Res<Observer>,
    simulation_time: Res<SimulationTime>,
    skyseeker: Res<Skyseeker>,
) {
    let observer = res_observer.get();
    let time = simulation_time.get_skyseeker_time();
    let earth_orientation =
        skyseeker_core::position::earth_orientation::EarthOrientation::default();

    let total_bodies = query.iter().count();
    if total_bodies == 0 {
        return;
    }

    if batched_update.current_index >= total_bodies {
        batched_update.current_index = 0;
    }

    let start = batched_update.current_index;
    let end = (start + batched_update.batch_size).min(total_bodies);

    query
        .iter_mut()
        .skip(start)
        .take(end - start)
        .for_each(|(body, mut transform)| {
            let _span = tracing::span!(tracing::Level::TRACE, "calculate_body_position").entered();

            let Ok(sky_position) =
                skyseeker
                    .get()
                    .position(body.get().id(), observer, &time, &earth_orientation)
            else {
                return;
            };

            let space_position = sky_position_to_vec_3(sky_position, 3500.0);
            transform.translation = space_position;
        });

    batched_update.current_index = end;
}

fn sky_position_to_vec_3(sky_position: skyseeker_core::position::Position, radius: f32) -> Vec3 {
    let azimuth = -sky_position.azimuth.to_radians() as f32;
    let altitude = sky_position.altitude.to_radians() as f32;

    let y = radius * altitude.sin();
    let horizontal_distance = radius * altitude.cos();

    let x = horizontal_distance * azimuth.sin();
    let z = horizontal_distance * azimuth.cos();

    Vec3::new(x, y, z)
}
