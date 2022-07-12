use bevy::prelude::*;

use crate::components::Food;

pub fn food_despawner_system(mut query: Query<Entity, With<Food>>, mut commands: Commands) {
    let mut peekaboo = query.iter_mut().peekable();
    if peekaboo.peek().is_some() {
        commands.entity(peekaboo.next().unwrap()).despawn();
    }
}