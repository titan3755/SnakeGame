use bevy::math::vec2;
use bevy::prelude::*;
use rand::prelude::*;
use uuid::*;

use crate::components::*;
use crate::components::Direction;
use crate::{BOUNDARY_WIDTH, DIMENSIONS, FOOD_COLOR, FOOD_DIMENSIONS, PLAYER_DIMENSIONS, SCORE_COLOR, SCORE_TEXT_COLOR, SNAKE_COLOR};
use crate::tail_spawner::tail_spawner;

pub fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>, mut tail_parts: ResMut<SnakeParts>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    *tail_parts = SnakeParts(vec![
        commands.spawn_bundle(SpriteBundle{
            sprite: Sprite {
                color: SNAKE_COLOR,
                custom_size: Some(vec2(PLAYER_DIMENSIONS, PLAYER_DIMENSIONS)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(thread_rng().gen_range((-(DIMENSIONS / 2.0) + (PLAYER_DIMENSIONS + BOUNDARY_WIDTH + 50.0)) as f32 .. ((DIMENSIONS / 2.0) - (PLAYER_DIMENSIONS + BOUNDARY_WIDTH + 50.0)) as f32), thread_rng().gen_range((-(DIMENSIONS / 2.0) + (PLAYER_DIMENSIONS + BOUNDARY_WIDTH + 50.0)) as f32 .. ((DIMENSIONS / 2.0) - (PLAYER_DIMENSIONS + BOUNDARY_WIDTH + 50.0)) as f32), 0.0),
                ..default()
            },
            ..default()
        }).insert(Player).insert(Direction::Right).insert(Velocity{x: PLAYER_DIMENSIONS, y: PLAYER_DIMENSIONS}).insert(Score{value: 0}).insert(PlayerSize{value: 1}).id(),
        tail_spawner(&mut commands),
    ]);
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: FOOD_COLOR,
            custom_size: Some(vec2(10.0, 10.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(thread_rng().gen_range((-(DIMENSIONS / 2.0) + (FOOD_DIMENSIONS + BOUNDARY_WIDTH)) as f32 .. ((DIMENSIONS / 2.0) - (FOOD_DIMENSIONS + BOUNDARY_WIDTH)) as f32), thread_rng().gen_range((-(DIMENSIONS / 2.0) + (FOOD_DIMENSIONS + BOUNDARY_WIDTH)) as f32 .. ((DIMENSIONS / 2.0) - (FOOD_DIMENSIONS + BOUNDARY_WIDTH)) as f32), 0.0),
            ..default()
        },
        ..default()
    }).insert(Food(Uuid::new_v4().to_string()));
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Score: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("FiraCode-Bold.ttf"),
                            font_size: 20.0,
                            color: SCORE_TEXT_COLOR,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("FiraCode-Medium.ttf"),
                            font_size: 20.0,
                            color: SCORE_COLOR,
                            ..Default::default()
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(ScoreText);
}
