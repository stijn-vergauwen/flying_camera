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
        .filter(|(_, _, input)| input.move_direction != Vec3::ZERO)
    {
        let rotated_input = transform.rotation * input.move_direction;

        transform.translation += rotated_input * time.delta_seconds();
    }
}

fn rotate_camera(mut cameras: Query<(&mut Transform, &FlyingCamera, &MovementInput)>) {
    for (mut transform, camera, input) in cameras
        .iter_mut()
        .filter(|(_, _, input)| input.rotate_direction != Vec3::ZERO)
    {
        let (current_y, current_x, current_z) = transform.rotation.to_euler(EulerRot::YXZ);

        let new_y = current_y + input.rotate_direction.y.to_radians();
        let new_x = current_x + input.rotate_direction.x.to_radians();

        transform.rotation = Quat::from_euler(EulerRot::YXZ, new_y, new_x, current_z);
    }
}
