use bevy::prelude::*;
use std::f64::consts::PI;

use crate::{
    components::{Position, TetrominoPiece},
    events::NearbyPieceEvent,
    resources::DescendTimer,
};

const ANGLE: f64 = 90.0 * (PI / 180.0);

pub fn movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut nearby_piece_reader: EventReader<NearbyPieceEvent>,
    mut tetromino_pieces: Query<Entity, With<TetrominoPiece>>,
    mut positions: Query<&mut Position>,
) {
    let key_just_pressed = keyboard_input.get_just_pressed().next();

    let collision_detected = nearby_piece_reader.read().next().is_some();
    if collision_detected {
        return;
    }

    match key_just_pressed {
        Some(&KeyCode::ArrowUp) => {
            let axis_point = {
                let entity = tetromino_pieces.iter().nth(1).unwrap().clone();

                *positions.get_mut(entity).unwrap()
            };

            for entity in tetromino_pieces.iter_mut() {
                let mut pos = positions.get_mut(entity).unwrap();

                let translated_x = pos.x - axis_point.x;
                let translated_y = pos.y - axis_point.y;

                pos.x = translated_x * (ANGLE.cos() as i32) - translated_y * (ANGLE.sin() as i32)
                    + axis_point.x;
                pos.y = translated_x * (ANGLE.sin() as i32)
                    + translated_y * (ANGLE.cos() as i32)
                    + axis_point.y;
            }
        }
        Some(KeyCode::ArrowLeft) => {
            for entity in tetromino_pieces.iter_mut() {
                let mut pos = positions.get_mut(entity).unwrap();
                pos.x -= 1
            }
        }
        Some(KeyCode::ArrowRight) => {
            for entity in tetromino_pieces.iter_mut() {
                let mut pos = positions.get_mut(entity).unwrap();
                pos.x += 1
            }
        }
        Some(KeyCode::ArrowDown) => {
            for entity in tetromino_pieces.iter_mut() {
                let mut pos = positions.get_mut(entity).unwrap();
                pos.y -= 1
            }
        }
        _ => return,
    }
}

pub fn descend(
    time: Res<Time>,
    mut descend_timer: ResMut<DescendTimer>,
    mut tetromino_pieces: Query<Entity, With<TetrominoPiece>>,
    mut positions: Query<&mut Position>,
) {
    descend_timer.0.tick(time.delta());
    if !descend_timer.0.finished() {
        return;
    }

    for tetromino_piece in tetromino_pieces.iter_mut() {
        let mut piece = positions.get_mut(tetromino_piece).unwrap();

        piece.y -= 1
    }
}
