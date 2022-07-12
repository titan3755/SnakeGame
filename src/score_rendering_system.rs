use bevy::prelude::*;

use crate::components::{Player, Score, ScoreText};

pub fn score_rendering_system(mut text_query: Query<&mut Text, With<ScoreText>>, mut score_query: Query<&Score, With<Player>>) {
    for mut text in text_query.iter_mut() {
        let score = score_query.iter_mut().next().unwrap().value;
        text.sections[1].value = format!("{:.2}", score);
    }
}