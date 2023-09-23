pub mod input;
mod interaction;
mod movement;
mod rotation;

use bevy::prelude::*;
use input::{MovementInput, MovementInputPlugin};

pub struct FlyingCameraPlugin;

impl Plugin for FlyingCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MovementInputPlugin);
    }
}

#[derive(Component)]
pub struct FlyingCamera;

#[derive(Bundle)]
pub struct FlyingCameraBundle {
    pub flying_camera: FlyingCamera,
    pub movement_input: MovementInput,
}

impl Default for FlyingCameraBundle {
    fn default() -> Self {
        Self {
            flying_camera: FlyingCamera,
            movement_input: MovementInput::default(),
        }
    }
}
