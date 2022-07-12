use bevy::prelude::*;

use crate::components::{Direction, Velocity, Player};
use crate::CoordList;

pub fn snake_movement_system(kbd: Res<Input<KeyCode>>, mut coords: ResMut<CoordList>, mut query: Query<(&mut Direction, &Velocity, &mut Transform), With<Player>>) {
    for (mut sprite, velocity, mut transform) in query.iter_mut() {
        match *sprite {
            Direction::Up    => transform.translation.y += velocity.y,
            Direction::Down  => transform.translation.y -= velocity.y,
            Direction::Left  => transform.translation.x -= velocity.x,
            Direction::Right => transform.translation.x += velocity.x,
            Direction::NoDir => ()
        }

        if kbd.pressed(KeyCode::W) && !matches!(*sprite, Direction::Down) {
            *sprite = Direction::Up;
            coords.0.push((transform.translation.x, transform.translation.y));
        }

        if kbd.pressed(KeyCode::S) && !matches!(*sprite, Direction::Up) {
            *sprite = Direction::Down;
            coords.0.push((transform.translation.x, transform.translation.y));
        }

        if kbd.pressed(KeyCode::A) && !matches!(*sprite, Direction::Right) {
            *sprite = Direction::Left;
            coords.0.push((transform.translation.x, transform.translation.y));
        }

        if kbd.pressed(KeyCode::D) && !matches!(*sprite, Direction::Left) {
            *sprite = Direction::Right;
            coords.0.push((transform.translation.x, transform.translation.y));
        }
    }
}