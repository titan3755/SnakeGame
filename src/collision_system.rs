use bevy::prelude::*;
use rand::prelude::*;

use crate::components::{Player, Score, Direction, PlayerSize, SnakePart};
use crate::{DIMENSIONS, PLAYER_DIMENSIONS, BOUNDARY_WIDTH, SnakeParts};
use crate::tail_spawner::tail_spawner;

pub fn collision_system(mut player_query: Query<(&mut Direction, &mut Score, &mut PlayerSize, &mut Transform), With<Player>>, tail_query: Query<Entity, With<SnakePart>>, mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>, mut segments: ResMut<SnakeParts>) {
    for (mut direction, mut score, mut p_size, mut transform) in player_query.iter_mut() {
        if transform.translation.x < -(DIMENSIONS / 2.0 - BOUNDARY_WIDTH) || transform.translation.x > (DIMENSIONS / 2.0 - BOUNDARY_WIDTH) || transform.translation.y < -(DIMENSIONS / 2.0 - BOUNDARY_WIDTH) || transform.translation.y > (DIMENSIONS / 2.0 - BOUNDARY_WIDTH) {
            *direction = Direction::Right;
            score.value = 0;
            p_size.value = 0;
            transform.translation.x = thread_rng().gen_range((-(DIMENSIONS / 2.0) + (PLAYER_DIMENSIONS + BOUNDARY_WIDTH)) as f32 .. ((DIMENSIONS / 2.0) - (PLAYER_DIMENSIONS + BOUNDARY_WIDTH)) as f32);
            transform.translation.y = thread_rng().gen_range((-(DIMENSIONS / 2.0) + (PLAYER_DIMENSIONS + BOUNDARY_WIDTH)) as f32 .. ((DIMENSIONS / 2.0) - (PLAYER_DIMENSIONS + BOUNDARY_WIDTH)) as f32);
            let segment_clone: Entity = segments.0[0];
            segments.0.retain(|&_x| false);
            segments.0.push(segment_clone);
            segments.0.push(tail_spawner(&mut commands));
            let mut peekaboo = tail_query.iter().peekable();
            if peekaboo.peek().is_some() {
                for entity in tail_query.iter() {
                    commands.entity(entity).despawn();
                }
            }
            audio.play(asset_server.load("collision.ogg"));
        }
    }
}

//  || ((((tail_transform.translation.x - transform.translation.x) as f32).powf(2.0) + ((tail_transform.translation.y - transform.translation.y) as f32).powf(2.0)) as f32).sqrt() < (PLAYER_DIMENSIONS / 2.0 + TAIL_DIMENSIONS / 2.0)