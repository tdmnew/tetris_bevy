use bevy::prelude::*;

use crate::{
    components::{DespawnedTetrominoPiece, Position, TetrominoPiece},
    constants::ARENA_WIDTH,
    events::{DespawnTetrominoEvent, NearbyPieceEvent},
};

pub fn check_movement_collision(
    mut tetromino_pieces: Query<Entity, With<TetrominoPiece>>,
    despawned_pieces: Query<Entity, With<DespawnedTetrominoPiece>>,
    mut positions: Query<&mut Position>,
    mut nearby_piece_writer: EventWriter<NearbyPieceEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let key_pressed = keyboard_input.get_pressed().next();

    let despawned_pieces_positions = despawned_pieces
        .iter()
        .map(|e| *positions.get_mut(e).unwrap())
        .collect::<Vec<Position>>();

    for tetromino_piece in tetromino_pieces.iter_mut() {
        let piece = positions.get_mut(tetromino_piece).unwrap();

        match key_pressed {
            Some(KeyCode::ArrowLeft) => {
                let mut next_left_movement = piece.clone();
                next_left_movement.x -= 1;

                if despawned_pieces_positions.contains(&next_left_movement)
                    || next_left_movement.x == -1
                    || next_left_movement.y == 0
                {
                    nearby_piece_writer.send(NearbyPieceEvent);
                }
            }
            Some(KeyCode::ArrowRight) => {
                let mut next_right_movement = piece.clone();
                next_right_movement.x += 1;

                if despawned_pieces_positions.contains(&next_right_movement)
                    || next_right_movement.x == (ARENA_WIDTH as i32)
                    || next_right_movement.y == 0
                {
                    nearby_piece_writer.send(NearbyPieceEvent);
                }
            }
            Some(KeyCode::ArrowDown) => {
                let mut down_movement_pos = piece.clone();
                down_movement_pos.y -= 1;

                if despawned_pieces_positions.contains(&down_movement_pos)
                    || down_movement_pos.y == 0
                {
                    nearby_piece_writer.send(NearbyPieceEvent);
                }
            }
            _ => return,
        }
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
