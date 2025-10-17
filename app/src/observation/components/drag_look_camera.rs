use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

#[derive(Component)]
pub struct DragLookCamera {
    pub sensitivity: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub smoothness: f32,
    target_yaw: f32,
    target_pitch: f32,
}

impl Default for DragLookCamera {
    fn default() -> Self {
        Self {
            sensitivity: 0.003,
            yaw: 0.0,
            pitch: 0.0,
            smoothness: 0.5,
            target_yaw: 0.0,
            target_pitch: 0.0,
        }
    }
}

pub fn drag_look_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut DragLookCamera)>,
    time: Res<Time>,
) {
    let mut delta = Vec2::ZERO;

    if mouse_button.pressed(MouseButton::Left) {
        for motion in mouse_motion.read() {
            delta += motion.delta;
        }
    } else {
        mouse_motion.clear();
    }

    for (mut transform, mut camera) in query.iter_mut() {
        if delta.length_squared() > 0.0 {
            camera.target_yaw -= delta.x * camera.sensitivity;
            camera.target_pitch -= delta.y * camera.sensitivity;
            camera.target_pitch = camera
                .target_pitch
                .clamp(-89_f32.to_radians(), 89_f32.to_radians());
        }

        let lerp_factor = 1.0 - camera.smoothness.powf(time.delta_secs() * 60.0);
        camera.yaw += (camera.target_yaw - camera.yaw) * lerp_factor;
        camera.pitch += (camera.target_pitch - camera.pitch) * lerp_factor;
        transform.rotation = Quat::from_euler(EulerRot::YXZ, camera.yaw, camera.pitch, 0.0);
    }
}
