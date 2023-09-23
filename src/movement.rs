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
        .filter(|(_, camera, input)| camera.enabled && input.move_direction != Vec3::ZERO)
    {
        let rotated_input = transform.rotation * input.move_direction;

        transform.translation += rotated_input * camera.move_speed * time.delta_seconds();
    }
}

fn rotate_camera(mut cameras: Query<(&mut Transform, &mut FlyingCamera, &MovementInput)>) {
    for (mut transform, mut camera, input) in cameras
        .iter_mut()
        .filter(|(_, camera, input)| camera.enabled && input.rotate_direction != Vec3::ZERO)
    {
        let mut new_rotation = camera.current_eulers + input.rotate_direction * camera.rotate_speed;
        new_rotation.x = new_rotation.x.clamp(
            -camera.max_pitch_degrees.to_radians(),
            camera.max_pitch_degrees.to_radians(),
        );

        camera.current_eulers = new_rotation;

        transform.rotation = Quat::from_euler(
            EulerRot::YXZ,
            new_rotation.y,
            new_rotation.x,
            new_rotation.z,
        );
    }
}
