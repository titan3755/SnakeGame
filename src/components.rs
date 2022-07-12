use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct SnakePart(pub usize);

#[derive(Component)]
pub struct Food(pub String);

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct Score {
    pub value: usize
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32
}

#[derive(Component)]
pub enum Direction {
   Up, Down, Left, Right, NoDir
}

#[derive(Component)]
pub struct PlayerSize {
    pub value: u32
}

#[derive(Component)]
pub struct BoundaryWall(pub usize);

#[derive(Default)]
pub struct GameState {
    pub winning_score: usize
}

#[derive(Default)]
pub struct CoordList(pub Vec<(f32, f32)>);