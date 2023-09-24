pub mod input;
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
    /// A modifier for the regular movement speed of this camera.
    pub move_speed: f32,

    /// The amount that `move_speed` gets multiplied by when holding the speed-up key.
    pub speed_up_multiplier: f32,

    /// A modifier for the speed at which this camera rotates.
    pub rotate_speed: f32,

    /// The maximum degrees this camera can rotate up & down.
    pub max_pitch_degrees: f32,

    /// If this camera is currently enabled, disabled camera's don't move or rotate.
    pub enabled: bool,

    /// If this camera should get enabled by holding the button set in `button_to_enable`.
    pub enable_on_button_hold: bool,

    /// The mouse button used to enable this camera, if `enable_on_button_hold` is set to true.
    pub button_to_enable: MouseButton,

    /// If the cursor should be hidden when this camera is enabled.
    pub hide_cursor_when_enabled: bool,

    /// Current rotation in eulers.
    /// 
    /// Is set automatically, used for updating the transform's rotation.
    pub current_eulers: Vec3,
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
            move_speed: 3.0,
            speed_up_multiplier: 3.0,
            rotate_speed: 0.04,
            max_pitch_degrees: 90.0,
            enabled: false,
            enable_on_button_hold: true,
            button_to_enable: MouseButton::Right,
            hide_cursor_when_enabled: true,
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
    for mut camera in cameras
        .iter_mut()
        .filter(|camera| camera.enable_on_button_hold)
    {
        camera.enabled = input.pressed(camera.button_to_enable);
    }
}

fn update_cursor_visibility(
    cameras: Query<&FlyingCamera, Changed<FlyingCamera>>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = window.get_single_mut() {
        for camera in cameras
            .iter()
            .filter(|camera| camera.hide_cursor_when_enabled)
        {
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
