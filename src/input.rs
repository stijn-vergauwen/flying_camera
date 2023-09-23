use bevy::{input::mouse::MouseMotion, prelude::*};

pub struct MovementInputPlugin;

impl Plugin for MovementInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_movement_input, update_rotation_input));
    }
}

#[derive(Component)]
pub struct MovementInput {
    pub move_direction: Vec3,
    pub rotate_direction: Vec3,
    pub keybinds: MovementKeybinds,
}

impl Default for MovementInput {
    fn default() -> Self {
        Self {
            move_direction: Vec3::ZERO,
            rotate_direction: Vec3::ZERO,
            keybinds: MovementKeybinds::default(),
        }
    }
}

pub struct MovementKeybinds {
    pub forward: KeyCode,
    pub back: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub up: KeyCode,
    pub down: KeyCode,
    pub speed_up: KeyCode,
}

impl Default for MovementKeybinds {
    fn default() -> Self {
        Self {
            forward: KeyCode::W,
            back: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
            up: KeyCode::Space,
            down: KeyCode::ControlLeft,
            speed_up: KeyCode::ShiftLeft,
        }
    }
}

fn update_movement_input(
    mut movement_inputs: Query<&mut MovementInput>,
    input: Res<Input<KeyCode>>,
) {
    for mut movement_input in movement_inputs.iter_mut() {
        let direction = get_movement_direction(&movement_input.keybinds, &input);

        movement_input.move_direction = direction;
    }
}

fn update_rotation_input(
    mut movement_inputs: Query<&mut MovementInput>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    for mut movement_input in movement_inputs.iter_mut() {
        let total_delta = mouse_motion
            .iter()
            .fold(Vec2::ZERO, |sum, motion| sum + motion.delta);

        movement_input.rotate_direction = mouse_movement_to_euler(total_delta);
    }
}

fn get_movement_direction(keybinds: &MovementKeybinds, input: &Input<KeyCode>) -> Vec3 {
    let mut direction = Vec3::ZERO;

    if input.pressed(keybinds.forward) {
        direction.z -= 1.0;
    }

    if input.pressed(keybinds.back) {
        direction.z += 1.0;
    }

    if input.pressed(keybinds.left) {
        direction.x -= 1.0;
    }

    if input.pressed(keybinds.right) {
        direction.x += 1.0;
    }

    direction.normalize_or_zero()
}

/// Takes the mouse_motion.delta value and turns it into an euler rotation with an YXZ rotation order.
fn mouse_movement_to_euler(movement: Vec2) -> Vec3 {
    Vec3::new(-movement.y, -movement.x, 0.0)
}
