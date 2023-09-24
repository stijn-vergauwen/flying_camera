//! Demonstrates the default behaviour.

use bevy::prelude::*;
use flying_camera::{FlyingCameraBundle, FlyingCameraPlugin};

fn main() {
    App::new()
        // 1. Add FlyingCameraPlugin to plugins.
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
        // 2. Add FlyingCameraBundle to a camera.
        FlyingCameraBundle::default(),
    ));
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
        material: materials.add(Color::BEIGE.into()),
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
