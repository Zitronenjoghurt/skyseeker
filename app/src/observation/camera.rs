use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

#[derive(Bundle)]
pub struct CustomCamera {
    camera3d: Camera3d,
    camera: Camera,
    transform: Transform,
    pan_orbit: PanOrbitCamera,
    tonemapping: Tonemapping,
    bloom: Bloom,
}

impl Default for CustomCamera {
    fn default() -> Self {
        Self {
            camera3d: Camera3d::default(),
            camera: Camera::default(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            pan_orbit: PanOrbitCamera {
                button_orbit: MouseButton::Left,
                pan_sensitivity: 0.0,
                zoom_sensitivity: 0.0,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            bloom: Bloom::NATURAL,
        }
    }
}
