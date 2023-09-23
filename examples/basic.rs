//! Demonstrates the default behaviour.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_scene_objects)
        .run();
}

fn spawn_scene_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn((PbrBundle {
        mesh: meshes.add(shape::Box::new(200.0, 0.2, 200.0).into()),
        material: materials.add(StandardMaterial {
            base_color: Color::GRAY,
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
            rotation: Quat::from_rotation_x((-45.0 as f32).to_degrees()),
            ..default()
        },
        ..default()
    });
}
