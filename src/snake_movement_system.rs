use bevy::prelude::*;

use crate::components::{Direction, Velocity, Player};

pub fn snake_movement_system(kbd: Res<Input<KeyCode>>, mut query: Query<(&mut Direction, &Velocity, &mut Transform), With<Player>>) {
    for (mut sprite, velocity, mut transform) in query.iter_mut() {
        match *sprite {
            Direction::Up    => transform.translation.y += velocity.y,
            Direction::Down  => transform.translation.y -= velocity.y,
            Direction::Left  => transform.translation.x -= velocity.x,
            Direction::Right => transform.translation.x += velocity.x
        }

        if kbd.pressed(KeyCode::W) && !matches!(*sprite, Direction::Down) {
            *sprite = Direction::Up;
        }

        if kbd.pressed(KeyCode::S) && !matches!(*sprite, Direction::Up) {
            *sprite = Direction::Down;
        }

        if kbd.pressed(KeyCode::A) && !matches!(*sprite, Direction::Right) {
            *sprite = Direction::Left;
        }

        if kbd.pressed(KeyCode::D) && !matches!(*sprite, Direction::Left) {
            *sprite = Direction::Right;
        }
    }
}