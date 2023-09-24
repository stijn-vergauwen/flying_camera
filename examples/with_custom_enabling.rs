//! Demonstrates enabling / disabling the camera with our own code.

use bevy::prelude::*;
use flying_camera::{FlyingCamera, FlyingCameraBundle, FlyingCameraPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FlyingCameraPlugin))
        .add_systems(Startup, (spawn_scene_objects, spawn_flying_camera))
        .add_systems(Update, toggle_enabled_on_key_press)
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
                // Turn off the default behaviour of holding a mouse button to enable the camera.
                enable_on_button_hold: false,

                // Notice that the cursor will still get hidden when we enable the camera.
                hide_cursor_when_enabled: true,

                ..default()
            },
            ..default()
        },
    ));
}

// This system enables / disables all FlyingCamera's when the F key is pressed.
fn toggle_enabled_on_key_press(mut cameras: Query<&mut FlyingCamera>, input: Res<Input<KeyCode>>) {
    let toggle_key = KeyCode::F;

    if input.just_pressed(toggle_key) {
        for mut camera in cameras.iter_mut() {
            camera.enabled = !camera.enabled;
        }
    }
}

// Spawn objects so we have something to look at.
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
        material: materials.add(Color::BISQUE.into()),
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
