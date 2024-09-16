use bevy::prelude::*;

use crate::{
    components::{DespawnedTetrominoPiece, Position, TetrominoPiece},
    constants::{ANGLE, ARENA_WIDTH},
    events::{DespawnTetrominoEvent, NearbyPieceEvent},
};

pub fn check_movement_collision(
    mut tetromino_pieces: Query<Entity, With<TetrominoPiece>>,
    despawned_pieces: Query<Entity, With<DespawnedTetrominoPiece>>,
    mut positions: Query<&mut Position>,
    mut nearby_piece_writer: EventWriter<NearbyPieceEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if !keyboard_input.any_just_pressed([
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
        KeyCode::ArrowLeft,
        KeyCode::ArrowRight,
    ]) {
        return;
    }

    let mut has_collision = false;

    let despawned_pieces_positions = despawned_pieces
        .iter()
        .map(|e| *positions.get_mut(e).unwrap())
        .collect::<Vec<Position>>();

    let axis_point = {
        let ent = tetromino_pieces.iter_mut().nth(1).unwrap().clone();

        *positions.get_mut(ent).unwrap()
    };

    for tetromino_piece in tetromino_pieces.iter_mut() {
        let piece = positions.get_mut(tetromino_piece).unwrap();

        match keyboard_input.get_pressed().next() {
            Some(KeyCode::ArrowUp) => {
                let mut next_rotated_piece = piece.clone();
                let translated_x = next_rotated_piece.x - axis_point.x;
                let translated_y = next_rotated_piece.y - axis_point.y;

                next_rotated_piece.x = translated_x * (ANGLE.cos() as i32)
                    - translated_y * (ANGLE.sin() as i32)
                    + axis_point.x;
                next_rotated_piece.y = translated_x * (ANGLE.sin() as i32)
                    + translated_y * (ANGLE.cos() as i32)
                    + axis_point.y;

                if despawned_pieces_positions.contains(&next_rotated_piece)
                    || next_rotated_piece.x == -1
                    || next_rotated_piece.x == ARENA_WIDTH as i32
                {
                    has_collision = true;
                    break;
                }
            }
            Some(KeyCode::ArrowLeft) => {
                let mut next_left_movement = piece.clone();
                next_left_movement.x -= 1;

                if despawned_pieces_positions.contains(&next_left_movement)
                    || next_left_movement.x == -1
                {
                    has_collision = true;
                    break;
                }
            }
            Some(KeyCode::ArrowRight) => {
                let mut next_right_movement = piece.clone();
                next_right_movement.x += 1;

                if despawned_pieces_positions.contains(&next_right_movement)
                    || next_right_movement.x == (ARENA_WIDTH as i32)
                {
                    has_collision = true;
                    break;
                }
            }
            Some(KeyCode::ArrowDown) => {
                let mut down_movement_pos = piece.clone();
                down_movement_pos.y -= 1;

                if despawned_pieces_positions.contains(&down_movement_pos) {
                    has_collision = true;
                    break;
                }
            }
            _ => return,
        }
    }

    if has_collision {
        nearby_piece_writer.send(NearbyPieceEvent);
    }
}

pub fn check_descend_collision(
    mut tetromino_pieces: Query<Entity, With<TetrominoPiece>>,
    despawned_pieces: Query<Entity, With<DespawnedTetrominoPiece>>,
    mut positions: Query<&mut Position>,
    mut despawn_writer: EventWriter<DespawnTetrominoEvent>,
) {
    let despawned_pieces_positions = despawned_pieces
        .iter()
        .map(|e| *positions.get_mut(e).unwrap())
        .collect::<Vec<Position>>();

    for tetromino_piece in tetromino_pieces.iter_mut() {
        let piece = positions.get_mut(tetromino_piece).unwrap();

        let mut next_down_pos = piece.clone();
        next_down_pos.y -= 1;

        if piece.y == 0 || despawned_pieces_positions.contains(&next_down_pos) {
            despawn_writer.send(DespawnTetrominoEvent);
            return;
        }
    }
}
