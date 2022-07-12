use bevy::math::vec2;
use bevy::prelude::*;
use rand::prelude::*;
use uuid::*;

use crate::components::*;
use crate::components::Direction::{NoDir};
use crate::{BOUNDARY_WIDTH, DIMENSIONS, FOOD_DIMENSIONS, PLAYER_DIMENSIONS};

pub fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(SpriteBundle{
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(vec2(PLAYER_DIMENSIONS, PLAYER_DIMENSIONS)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(thread_rng().gen_range((-(DIMENSIONS / 2.0) + (PLAYER_DIMENSIONS + BOUNDARY_WIDTH)) as f32 .. ((DIMENSIONS / 2.0) - (PLAYER_DIMENSIONS + BOUNDARY_WIDTH)) as f32), thread_rng().gen_range((-(DIMENSIONS / 2.0) + (PLAYER_DIMENSIONS + BOUNDARY_WIDTH)) as f32 .. ((DIMENSIONS / 2.0) - (PLAYER_DIMENSIONS + BOUNDARY_WIDTH)) as f32), 0.0),
            ..default()
        },
        ..default()
    }).insert(Player).insert(NoDir).insert(Velocity{x: PLAYER_DIMENSIONS, y: PLAYER_DIMENSIONS}).insert(Score{value: 0}).insert(PlayerSize{value: 1});
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
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("FiraCode-Medium.ttf"),
                            font_size: 20.0,
                            color: Color::GOLD,
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
