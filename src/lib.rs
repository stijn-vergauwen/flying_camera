pub mod input;
mod interaction;
mod movement;

use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use input::{MovementInput, MovementInputPlugin};
use movement::FlyingCameraMovementPlugin;

pub struct FlyingCameraPlugin;

impl Plugin for FlyingCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MovementInputPlugin, FlyingCameraMovementPlugin))
            .add_systems(Update, (update_camera_enabled, update_cursor_visibility));
    }
}

#[derive(Component)]
pub struct FlyingCamera {
    pub enabled: bool,
    pub move_speed: f32,
    pub speed_up_multiplier: f32,
    pub rotate_speed: f32,
    pub max_pitch_degrees: f32,
    pub enable_on_button_hold: bool,
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
            enabled: false,
            move_speed: 3.0,
            speed_up_multiplier: 3.0,
            rotate_speed: 0.04,
            max_pitch_degrees: 90.0,
            enable_on_button_hold: true,
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

fn update_cursor_visibility(
    cameras: Query<&FlyingCamera, Changed<FlyingCamera>>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = window.get_single_mut() {
        for camera in cameras.iter() {
            if camera.enabled == window.cursor.visible {
                set_cursor_visibility(&mut window, !camera.enabled);
            }
        }
    }
}

fn set_cursor_visibility(window: &mut Window, set_visible: bool) {
    window.cursor.visible = set_visible;

    window.cursor.grab_mode = match set_visible {
        true => CursorGrabMode::None,
        false => CursorGrabMode::Locked,
    }
}
