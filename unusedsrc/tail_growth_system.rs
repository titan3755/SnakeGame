use bevy::prelude::*;
use uuid::*;

use crate::components::{Player, PlayerSize, SnakePart};
use crate::{TAIL_DIMENSIONS};

pub fn tail_growth_system(mut player_query: Query<(&PlayerSize, &Transform), With<Player>>, mut commands: Commands) {
    // for (p_size, p_transform) in player_query.iter_mut() {
    //     for _ in 0..p_size.value {
    //         commands.spawn_bundle(SpriteBundle{
    //             sprite: Sprite{
    //                 color: Color::GRAY,
    //                 custom_size: Some(Vec2::new(TAIL_DIMENSIONS, TAIL_DIMENSIONS)),
    //                 ..default()
    //             },
    //             transform: Transform{
    //                 translation: Vec3::new()
    //             },
    //             ..default()
    //         }).insert(SnakePart());
    //     }
    // }
}