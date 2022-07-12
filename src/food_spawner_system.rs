use bevy::math::vec2;
use bevy::prelude::*;
use rand::prelude::*;
use uuid::*;

use crate::{BOUNDARY_WIDTH, DIMENSIONS, FOOD_DIMENSIONS};
use crate::components::Food;

pub fn food_spawner_system(mut commands: Commands) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(vec2(10.0, 10.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(thread_rng().gen_range((-(DIMENSIONS / 2.0) + (FOOD_DIMENSIONS + BOUNDARY_WIDTH)) as f32 .. ((DIMENSIONS / 2.0) - (FOOD_DIMENSIONS + BOUNDARY_WIDTH)) as f32), thread_rng().gen_range((-(DIMENSIONS / 2.0) + (FOOD_DIMENSIONS + BOUNDARY_WIDTH)) as f32 .. ((DIMENSIONS / 2.0) - (FOOD_DIMENSIONS + BOUNDARY_WIDTH)) as f32), 0.0),
            ..default()
        },
        ..default()
    }).insert(Food(Uuid::new_v4().to_string()));
}