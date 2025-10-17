use crate::observation::components::horizon::HorizonTag;
use bevy::color::Color;
use bevy::mesh::{Mesh, Mesh3d};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{AlphaMode, Assets, Bundle, Circle, Quat, ResMut, Transform};
use bevy::utils::default;

#[derive(Bundle)]
pub struct Horizon {
    tag: HorizonTag,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    transform: Transform,
}

impl Horizon {
    pub fn new(
        radius: f32,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        let mesh = meshes.add(Circle::new(radius));
        let material = materials.add(StandardMaterial {
            base_color: Color::srgba(0.05, 0.05, 0.05, 0.9),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            double_sided: true,
            cull_mode: None,
            ..default()
        });

        Self {
            tag: HorizonTag,
            mesh: Mesh3d(mesh),
            material: MeshMaterial3d(material),
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        }
    }
}
