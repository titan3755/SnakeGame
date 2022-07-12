use bevy::prelude::*;
use rand::prelude::*;

use crate::components::{Food, Player, PlayerSize, Score, SnakePart};
use crate::{COORDVEC_PLACEHOLDER, FOOD_DIMENSIONS, PLAYER_DIMENSIONS, TAIL_DIMENSIONS};

pub fn food_eating_system(mut food_query: Query<(Entity, &Transform), With<Food>>, mut player_query: Query<(&mut Score, &mut PlayerSize, &Transform), With<Player>>, mut commands: Commands) {
    for (mut score, mut p_size, p_transform) in player_query.iter_mut() {
        for (entity, f_transform) in food_query.iter_mut() {
            if ((((f_transform.translation.x - p_transform.translation.x) as f32).powf(2.0) + ((f_transform.translation.y - p_transform.translation.y) as f32).powf(2.0)) as f32).sqrt() < (PLAYER_DIMENSIONS / 2.0 + FOOD_DIMENSIONS / 2.0) {
                score.value += 1;
                p_size.value += 1;
                commands.spawn_bundle(SpriteBundle{
                    sprite: Sprite{
                        color: Color::GRAY,
                        custom_size: Some(Vec2::new(TAIL_DIMENSIONS, TAIL_DIMENSIONS)),
                        ..default()
                    },
                    transform: Transform{
                        translation: Vec3::new(COORDVEC_PLACEHOLDER.0, COORDVEC_PLACEHOLDER.1, 0.0),
                        ..default()
                    },
                    ..default()
                }).insert(SnakePart(thread_rng().gen_range(1..10000)));
                commands.entity(entity).despawn();
            }
        }
    }
}