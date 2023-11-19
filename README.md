# Flying camera plugin
A simple, quick to use free flying camera for when you just want to move around in your scene.  
Mainly built for prototyping.

### Default controls:
- Hold right click -> Enable movement.
- Wasd -> Movement.
- Space & Left ctrl -> Move up & down.
- Move mouse -> Rotate camera.

## Quick Start

Add the plugin:

```rust ignore
.add_plugins(FlyingCameraPlugin)
```

Add `FlyingCameraBundle` to a camera:

```rust ignore
commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 3.0, 6.0)),
            ..default()
        },
        FlyingCameraBundle::default(),
    ));
```

You can check out the examples for more info.
