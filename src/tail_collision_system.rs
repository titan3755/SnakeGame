use bevy::prelude::*;
use rand::prelude::*;

use crate::{SnakeParts, components::{SnakePart, Direction}, DIMENSIONS, PLAYER_DIMENSIONS, BOUNDARY_WIDTH};
use crate::components::{Food, Player, Score};
use crate::tail_spawner::tail_spawner;


pub fn tail_collision_system(mut heads: Query<(Entity, &Player)>, mut tail: Query<&Transform, With<SnakePart>>, mut player: Query<&mut Transform, Without<SnakePart>>, mut player_data: Query<(&mut Direction, &mut Score), With<Player>>, tail_query: Query<Entity, With<SnakePart>>, mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>, mut segments: ResMut<SnakeParts>) {
    if let Some((_head_entity, _head)) = heads.iter_mut().next() {
        for mut player_transform in player.iter_mut() {
            for tail_transform in tail.iter_mut() {
                for (mut direction, mut score) in player_data.iter_mut() {
                    if (tail_transform.translation.x, tail_transform.translation.y) == (player_transform.translation.x, player_transform.translation.y) {
                        *direction = Direction::Right;
                        score.value = 0;
                        player_transform.translation.x = thread_rng().gen_range((-(DIMENSIONS / 2.0) + (PLAYER_DIMENSIONS + BOUNDARY_WIDTH)) as f32 .. ((DIMENSIONS / 2.0) - (PLAYER_DIMENSIONS + BOUNDARY_WIDTH)) as f32);
                        player_transform.translation.y = thread_rng().gen_range((-(DIMENSIONS / 2.0) + (PLAYER_DIMENSIONS + BOUNDARY_WIDTH)) as f32 .. ((DIMENSIONS / 2.0) - (PLAYER_DIMENSIONS + BOUNDARY_WIDTH)) as f32);
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
        }
    }
}

// Respawn functionality -->

// *direction = Direction::Right;
// score.value = 0;
// player_transform.translation.x = thread_rng().gen_range((-(DIMENSIONS / 2.0) + (PLAYER_DIMENSIONS + BOUNDARY_WIDTH)) as f32 .. ((DIMENSIONS / 2.0) - (PLAYER_DIMENSIONS + BOUNDARY_WIDTH)) as f32);
// player_transform.translation.y = thread_rng().gen_range((-(DIMENSIONS / 2.0) + (PLAYER_DIMENSIONS + BOUNDARY_WIDTH)) as f32 .. ((DIMENSIONS / 2.0) - (PLAYER_DIMENSIONS + BOUNDARY_WIDTH)) as f32);
// let segment_clone: Entity = segments.0[0];
// segments.0.retain(|&_x| false);
// segments.0.push(segment_clone);
// segments.0.push(tail_spawner(&mut commands));
// let mut peekaboo = tail_query.iter().peekable();
// if peekaboo.peek().is_some() {
//     for entity in tail_query.iter() {
//         commands.entity(entity).despawn();
//     }
// }
// audio.play(asset_server.load("collision.ogg"));

// References -->

// let mut player_transform = *transform_query.get_mut(segments.0[0]).unwrap();
// let segment_parts = segments.0.iter().map(|e| *transform_query.get_mut(*e).unwrap()).collect::<Vec<Transform>>();