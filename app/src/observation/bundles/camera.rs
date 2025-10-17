use crate::observation::components::drag_look_camera::DragLookCamera;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct CustomCamera {
    camera3d: Camera3d,
    camera: Camera,
    drag_look_camera: DragLookCamera,
    transform: Transform,
    tonemapping: Tonemapping,
    bloom: Bloom,
}

impl Default for CustomCamera {
    fn default() -> Self {
        Self {
            camera3d: Camera3d::default(),
            camera: Camera::default(),
            drag_look_camera: DragLookCamera::default(),
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
            tonemapping: Tonemapping::TonyMcMapface,
            bloom: Bloom::NATURAL,
        }
    }
}
