pub mod input;
mod interaction;
mod movement;

use bevy::prelude::*;
use input::{MovementInput, MovementInputPlugin};
use movement::FlyingCameraMovementPlugin;

pub struct FlyingCameraPlugin;

impl Plugin for FlyingCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MovementInputPlugin, FlyingCameraMovementPlugin));
    }
}

#[derive(Component)]
pub struct FlyingCamera {
    pub enabled: bool,
    pub move_speed: f32,
    pub rotate_speed: f32,
    pub max_pitch_degrees: f32,
    /// Current rotation in eulers, used for updating the transform's rotation.
    current_eulers: Vec3,
}

impl Default for FlyingCamera {
    fn default() -> Self {
        Self {
            enabled: true,
            move_speed: 4.0,
            rotate_speed: 0.1,
            max_pitch_degrees: 90.0,
            current_eulers: Vec3::ZERO,
        }
    }
}

#[derive(Bundle)]
pub struct FlyingCameraBundle {
    pub flying_camera: FlyingCamera,
    pub movement_input: MovementInput,
}

impl Default for FlyingCameraBundle {
    fn default() -> Self {
        Self {
            flying_camera: FlyingCamera::default(),
            movement_input: MovementInput::default(),
        }
    }
}
