use bevy::prelude::*;
use rand::prelude::*;

use crate::components::{Player, Score, Direction, PlayerSize, SnakePart};
use crate::{DIMENSIONS, PLAYER_DIMENSIONS, BOUNDARY_WIDTH};

pub fn boundary_collision_system(mut player_query: Query<(&mut Direction, &mut Score, &mut PlayerSize, &mut Transform), With<Player>>, tail_query: Query<Entity, With<SnakePart>>, mut commands: Commands) {
    for (mut direction, mut score, mut p_size, mut transform) in player_query.iter_mut() {
        if transform.translation.x < -(DIMENSIONS / 2.0 - BOUNDARY_WIDTH) || transform.translation.x > (DIMENSIONS / 2.0 - BOUNDARY_WIDTH) || transform.translation.y < -(DIMENSIONS / 2.0 - BOUNDARY_WIDTH) || transform.translation.y > (DIMENSIONS / 2.0 - BOUNDARY_WIDTH) {
            *direction = Direction::NoDir;
            score.value = 0;
            p_size.value = 0;
            transform.translation.x = thread_rng().gen_range((-(DIMENSIONS / 2.0) + (PLAYER_DIMENSIONS + BOUNDARY_WIDTH)) as f32 .. ((DIMENSIONS / 2.0) - (PLAYER_DIMENSIONS + BOUNDARY_WIDTH)) as f32);
            transform.translation.y = thread_rng().gen_range((-(DIMENSIONS / 2.0) + (PLAYER_DIMENSIONS + BOUNDARY_WIDTH)) as f32 .. ((DIMENSIONS / 2.0) - (PLAYER_DIMENSIONS + BOUNDARY_WIDTH)) as f32);
            let mut peekaboo = tail_query.iter().peekable();
            if peekaboo.peek().is_some() {
                for entity in tail_query.iter() {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}