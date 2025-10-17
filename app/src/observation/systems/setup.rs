use crate::observation::bundles::camera;
use crate::observation::bundles::horizon::Horizon;
use crate::observation::bundles::sky_object::SkyObject;
use crate::observation::components::celestial_body::CelestialBody;
use crate::observation::resources::skyseeker::Skyseeker;
use bevy::camera::ClearColor;
use bevy::color::Color;
use bevy::prelude::*;

pub fn setup_view(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
    commands.spawn(camera::CustomCamera::default());
    clear_color.0 = Color::BLACK;
}

pub fn spawn_horizon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let horizon = Horizon::new(3500.0, &mut meshes, &mut materials);
    commands.spawn(horizon);
}

pub fn spawn_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    skyseeker: Res<Skyseeker>,
) {
    for body in skyseeker.get().iter_bodies() {
        let celestial_body = CelestialBody::new(body.clone());
        let sky_object = SkyObject::new(celestial_body, &mut meshes, &mut materials);
        commands.spawn(sky_object);
    }
}
