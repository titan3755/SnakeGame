use bevy::math::vec2;
use bevy::prelude::*;

use crate::{COORDVEC_PLACEHOLDER, TAIL_COLOR, TAIL_DIMENSIONS};
use crate::components::SnakePart;

pub fn tail_spawner(commands: &mut Commands<'_, '_>) -> Entity {
    commands.spawn_bundle(SpriteBundle{
        sprite: Sprite{
            color: TAIL_COLOR,
            custom_size: Some(vec2(TAIL_DIMENSIONS, TAIL_DIMENSIONS)),
            ..default()
        },
        transform: Transform{
            translation: Vec3::new(COORDVEC_PLACEHOLDER.0, COORDVEC_PLACEHOLDER.1, 0.0),
            ..default()
        },
        ..default()
    }).insert(SnakePart).id()
}