pub mod input;
mod interaction;
mod movement;

use bevy::prelude::*;
use input::{MovementInput, MovementInputPlugin};
use movement::FlyingCameraMovementPlugin;

pub struct FlyingCameraPlugin;

impl Plugin for FlyingCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MovementInputPlugin, FlyingCameraMovementPlugin))
            .add_systems(Update, update_camera_enabled);
    }
}

#[derive(Component)]
pub struct FlyingCamera {
    pub enabled: bool,
    pub move_speed: f32,
    pub speed_up_multiplier: f32,
    pub rotate_speed: f32,
    pub max_pitch_degrees: f32,
    pub button_to_enable: MouseButton,
    /// Current rotation in eulers, used for updating the transform's rotation.
    current_eulers: Vec3,
}

impl FlyingCamera {
    fn get_movement_speed(&self, speed_up: bool) -> f32 {
        if speed_up {
            self.move_speed * self.speed_up_multiplier
        } else {
            self.move_speed
        }
    }
}

impl Default for FlyingCamera {
    fn default() -> Self {
        Self {
            enabled: true,
            move_speed: 3.0,
            speed_up_multiplier: 3.0,
            rotate_speed: 0.1,
            max_pitch_degrees: 90.0,
            button_to_enable: MouseButton::Right,
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

fn update_camera_enabled(mut cameras: Query<&mut FlyingCamera>, input: Res<Input<MouseButton>>) {
    for mut camera in cameras.iter_mut() {
        camera.enabled = input.pressed(camera.button_to_enable);
    }
}
