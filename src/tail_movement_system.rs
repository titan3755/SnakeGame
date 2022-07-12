use bevy::prelude::*;

use crate::components::{SnakePart, Direction};
use crate::TAIL_DIMENSIONS;

pub fn tail_movement_system(mut tail_query: Query<&mut Transform, With<SnakePart>>, mut player_query: Query<(&Transform, &Direction), Without<SnakePart>>) {
    let mut distance: f32 = 0.0;
    for (transform, direction) in player_query.iter_mut() {
        for mut tail_transform in tail_query.iter_mut() {
            match *direction {
                Direction::Up => {
                    tail_transform.translation.x = transform.translation.x;
                    tail_transform.translation.y = transform.translation.y - distance;
                },
                Direction::Down => {
                    tail_transform.translation.x = transform.translation.x;
                    tail_transform.translation.y = transform.translation.y + distance;
                },
                Direction::Left => {
                    tail_transform.translation.x = transform.translation.x + distance;
                    tail_transform.translation.y = transform.translation.y;
                },
                Direction::Right => {
                    tail_transform.translation.x = transform.translation.x - distance;
                    tail_transform.translation.y = transform.translation.y;
                },
                Direction::NoDir => ()
            }
            distance += TAIL_DIMENSIONS + 20.0;
        }
    }
}