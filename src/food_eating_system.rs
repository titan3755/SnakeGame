use bevy::prelude::*;

use crate::components::{Food, Player, PlayerSize, Score};
use crate::{FOOD_DIMENSIONS, PLAYER_DIMENSIONS, SnakeParts};
use crate::tail_spawner::tail_spawner;

pub fn food_eating_system(mut food_query: Query<(Entity, &Transform), With<Food>>, mut player_query: Query<(&mut Score, &mut PlayerSize, &Transform), With<Player>>, mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>, mut segments: ResMut<SnakeParts>) {
    for (mut score, mut p_size, p_transform) in player_query.iter_mut() {
        for (entity, f_transform) in food_query.iter_mut() {
            if ((((f_transform.translation.x - p_transform.translation.x) as f32).powf(2.0) + ((f_transform.translation.y - p_transform.translation.y) as f32).powf(2.0)) as f32).sqrt() < (PLAYER_DIMENSIONS / 2.0 + FOOD_DIMENSIONS / 2.0) {
                score.value += 1;
                p_size.value += 1;
                commands.entity(entity).despawn();
                segments.0.push(tail_spawner(&mut commands));
                audio.play(asset_server.load("eat.ogg"));
            }
        }
    }
}

// commands.spawn_bundle(SpriteBundle{
//     sprite: Sprite{
//         color: Color::GRAY,
//         custom_size: Some(Vec2::new(TAIL_DIMENSIONS, TAIL_DIMENSIONS)),
//         ..default()
//     },
//     transform: Transform{
//         translation: Vec3::new(p_transform.translation.x - TAIL_DIMENSIONS, p_transform.translation.y, 0.0),
//         ..default()
//     },
//     ..default()
// }).insert(SnakePart);