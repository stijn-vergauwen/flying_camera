use bevy::prelude::*;

use crate::{input::MovementInput, FlyingCamera};

pub struct FlyingCameraMovementPlugin;

impl Plugin for FlyingCameraMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_camera);
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
