#![windows_subsystem = "windows"]

// Module attachments

mod setup;
mod components;
mod snake_movement_system;
mod food_eating_system;
mod food_spawner_system;
mod food_despawner_system;
mod score_rendering_system;
mod collision_system;
mod tail_movement_system;
mod tail_spawner;
mod tail_collision_system;

// Library imports

use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use rand::{Rng, thread_rng};

// File imports

use setup::*;
use snake_movement_system::*;
use food_eating_system::*;
use food_spawner_system::*;
use food_despawner_system::*;
use score_rendering_system::*;
use collision_system::*;
use tail_movement_system::*;
use tail_collision_system::*;
use crate::components::SnakeParts;

// Game constants

pub const DIMENSIONS: f32 = 500.0;
pub const PLAYER_DIMENSIONS: f32 = 30.0;
pub const TAIL_DIMENSIONS: f32 = 20.0;
pub const FOOD_DIMENSIONS: f32 = 8.0;
pub const BOUNDARY_WIDTH: f32 = 10.0;
pub const FRAME_TIME: f64 = 0.1;
pub const SNAKE_COLOR: Color = Color::WHITE;
pub const TAIL_COLOR: Color = Color::GRAY;
pub const BG_COLOR: Color = Color::BLACK;
pub const FOOD_COLOR: Color = Color::RED;
pub const SCORE_TEXT_COLOR: Color = Color::WHITE;
pub const SCORE_COLOR: Color = Color::GOLD;
pub const COORDVEC_PLACEHOLDER: (f32, f32) = (-1111.1111, -1111.1111);

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    App::new()
        .insert_resource(SnakeParts::default())
        .insert_resource(ClearColor{0: BG_COLOR})
        .insert_resource(WindowDescriptor{
            title: "Snake".to_string(),
            height: DIMENSIONS,
            width: DIMENSIONS,
            resizable: false,
            cursor_visible: false,
            mode: WindowMode::Windowed,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_plugins_with(DefaultPlugins, |group| {
            group.add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
        })
        .add_startup_system(setup_system)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(FRAME_TIME))
                .with_system(snake_movement_system.label("snake_movement"))
        )
        .add_system(food_eating_system)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(thread_rng().gen_range(1.0..3.0)))
                .with_system(food_spawner_system)
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(10.0))
                .with_system(food_despawner_system)
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(FRAME_TIME))
                .with_system(tail_movement_system.label("tail_movement"))
                .before("snake_movement")
        )
        .add_system(score_rendering_system)
        .add_system(collision_system)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(FRAME_TIME))
                .with_system(tail_collision_system)
                .after("snake_movement")
                .after("tail_movement")
        )
        .run();
}
