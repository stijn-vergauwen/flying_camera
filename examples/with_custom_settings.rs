//! Demonstrates changing of different camera settings.

use bevy::prelude::*;
use flying_camera::{
    input::{MovementInput, MovementKeybinds},
    FlyingCamera, FlyingCameraBundle, FlyingCameraPlugin,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FlyingCameraPlugin))
        .add_systems(Startup, (spawn_scene_objects, spawn_flying_camera))
        .run();
}

fn spawn_flying_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 3.0, 6.0)),
            ..default()
        },
        FlyingCameraBundle {
            flying_camera: FlyingCamera {
                // Make the camera go faster.
                move_speed: 10.0,

                // And even faster when speeding up.
                speed_up_multiplier: 5.0,

                // Enable the camera with middle mouse click instead of right.
                button_to_enable: MouseButton::Middle,

                // Leave the cursor visible when moving.
                hide_cursor_when_enabled: false,

                // Limit up & down rotation to 45 degrees.
                max_pitch_degrees: 45.0,

                // And leave the rest the same.
                ..default()
            },

            // Create a MovementInput struct with custom keybinds.
            movement_input: MovementInput::with_keybinds(MovementKeybinds {
                // Move the camera down with the left alt key instead of ctrl.
                down: KeyCode::AltLeft,

                // And leave the rest the same.
                ..default()
            }),
        },
    ));
}

fn spawn_scene_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn((PbrBundle {
        mesh: meshes.add(shape::Box::new(10.0, 0.2, 10.0).into()),
        material: materials.add(StandardMaterial {
            base_color: Color::GREEN,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    },));

    // Cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::CYAN.into()),
        transform: Transform::from_xyz(0.0, 0.7, 0.0),
        ..default()
    });

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 10_000.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 20.0, 0.0),
            rotation: Quat::from_rotation_x(-45.0f32.to_degrees()),
            ..default()
        },
        ..default()
    });
}
