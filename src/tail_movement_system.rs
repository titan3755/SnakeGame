use bevy::prelude::*;

use crate::components::{Player, SnakeParts};

pub fn tail_movement_system(mut transform_query: Query<&mut Transform>, segments: ResMut<SnakeParts>, mut heads: Query<(Entity, &Player)>) {
    if let Some((_head_entity, _head)) = heads.iter_mut().next() {
        segments.0.iter().map(|e| *transform_query.get_mut(*e).unwrap()).collect::<Vec<Transform>>().iter().zip(segments.0.iter().skip(1)).for_each(|(tr, seg)| {*transform_query.get_mut(*seg).unwrap() = *tr});
    }
}








// Unused code / References -->

// coords.0.retain(|&x| index_list.contains(&coords.0.iter().position(|&y| y == x).unwrap()));

// match *direction {
// Direction::Up => {
// tail_transform.translation.x = transform.translation.x;
// tail_transform.translation.y = transform.translation.y - distance - TAIL_DIMENSIONS;
// },
// Direction::Down => {
// tail_transform.translation.x = transform.translation.x;
// tail_transform.translation.y = transform.translation.y + distance + TAIL_DIMENSIONS;
// },
// Direction::Left => {
// tail_transform.translation.x = transform.translation.x + distance + TAIL_DIMENSIONS;
// tail_transform.translation.y = transform.translation.y;
// },
// Direction::Right => {
// tail_transform.translation.x = transform.translation.x - distance - TAIL_DIMENSIONS;
// tail_transform.translation.y = transform.translation.y;
// },
// Direction::NoDir => ()
// }
// distance += TAIL_DIMENSIONS + 20.0;

// for (transform, direction) in player_query.iter_mut() {
//     for (mut tail_transform, mut tail_direction) in tail_query.iter_mut() {
//
//         let mut tuple_list: Vec<(f32, f32)> = Vec::new();
//         for coordinate_tuple in coords.0.iter() {
//             if (tail_transform.translation.x, tail_transform.translation.y) == *coordinate_tuple {
//
//                 if matches!(direction, Direction::Up) {
//                     *tail_direction = Direction::Up;
//                 }
//
//                 if matches!(direction, Direction::Down) {
//                     *tail_direction = Direction::Down;
//                 }
//
//                 if matches!(direction, Direction::Left) {
//                     *tail_direction = Direction::Left;
//                 }
//
//                 if matches!(direction, Direction::Right) {
//                     *tail_direction = Direction::Right;
//                 }
//                 tuple_list.push(*coordinate_tuple)
//             }
//         }
//         match *tail_direction {
//             Direction::Up => {
//                 tail_transform.translation.x = transform.translation.x;
//                 tail_transform.translation.y = transform.translation.y - TAIL_DIMENSIONS;
//             },
//             Direction::Down => {
//                 tail_transform.translation.x = transform.translation.x;
//                 tail_transform.translation.y = transform.translation.y + TAIL_DIMENSIONS;
//             },
//             Direction::Left => {
//                 tail_transform.translation.x = transform.translation.x + TAIL_DIMENSIONS;
//                 tail_transform.translation.y = transform.translation.y;
//             },
//             Direction::Right => {
//                 tail_transform.translation.x = transform.translation.x - TAIL_DIMENSIONS;
//                 tail_transform.translation.y = transform.translation.y;
//             }
//         }
//         coords.0.retain(|&x| !tuple_list.contains(&x));
//     }
// }