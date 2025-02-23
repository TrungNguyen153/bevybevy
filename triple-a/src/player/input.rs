use bevy::prelude::*;

use crate::components::movement::{Facing, Velocity};

use super::Player;

const PLAYER_SPEED: f32 = 200.;

pub fn update_player_direction(
    keys: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<(&mut Facing, &mut Velocity), With<Player>>,
) {
    for (mut facing, mut velo) in &mut q_player {
        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::KeyJ) || keys.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if keys.pressed(KeyCode::KeyK) || keys.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keys.pressed(KeyCode::KeyF) || keys.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keys.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if direction != Vec3::ZERO {
            **facing = direction.normalize_or_zero();
        }

        velo.change_direction_speed(direction, PLAYER_SPEED);
    }
}
