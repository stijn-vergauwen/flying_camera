mod input;
mod interaction;
mod movement;
mod rotation;

use bevy::prelude::*;

pub struct FlyingCameraPlugin;

impl Plugin for FlyingCameraPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, ());
    }
}

#[derive(Component)]
pub struct FlyingCamera;
