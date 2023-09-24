use bevy::prelude::*;

use crate::{input::MovementInput, FlyingCamera};

pub struct FlyingCameraMovementPlugin;

impl Plugin for FlyingCameraMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_camera, rotate_camera));
    }
}

fn move_camera(
    mut cameras: Query<(&mut Transform, &FlyingCamera, &MovementInput)>,
    time: Res<Time>,
) {
    for (mut transform, camera, input) in cameras
        .iter_mut()
        .filter(|(_, camera, input)| camera.enabled && input.has_movement())
    {
        let rotated_input = transform.rotation * input.move_direction;
        let scaled_input = rotated_input * camera.get_movement_speed(input.speed_up);

        transform.translation += scaled_input * time.delta_seconds();
    }
}

fn rotate_camera(mut cameras: Query<(&mut Transform, &mut FlyingCamera, &MovementInput)>) {
    for (mut transform, mut camera, input) in cameras
        .iter_mut()
        .filter(|(_, camera, input)| camera.enabled && input.has_rotation())
    {
        let new_rotation = camera.current_eulers + input.rotate_direction * camera.rotate_speed;

        let clamped_rotation = clamp_pitch(new_rotation, camera.max_pitch_degrees.to_radians());

        camera.current_eulers = clamped_rotation;
        transform.rotation = Quat::from_euler(
            EulerRot::YXZ,
            clamped_rotation.y,
            clamped_rotation.x,
            clamped_rotation.z,
        );
    }
}

fn clamp_pitch(mut eulers: Vec3, max_pitch_radians: f32) -> Vec3 {
    eulers.x = eulers.x.clamp(-max_pitch_radians, max_pitch_radians);

    eulers
}
