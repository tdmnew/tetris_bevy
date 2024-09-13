use bevy::prelude::*;

use crate::{
    components::{DespawnedTetrominoPiece, Position, TetrominoPiece},
    constants::ARENA_WIDTH,
    events::{DespawnTetrominoEvent, NearbyPieceEvent},
    resources::MovementTimer,
};

pub fn movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut nearby_piece_reader: EventReader<NearbyPieceEvent>,
    mut tetromino_pieces: Query<(&TetrominoPiece, &mut Position)>,
) {
    let key_pressed = keyboard_input.get_just_pressed().next();

    match key_pressed {
        // Some(KeyCode::ArrowUp) => {
        //     let collision_detected = nearby_piece_reader.read().next().is_some();

        //     if !collision_detected {
        //         for (_, mut pos) in tetromino_pieces.iter_mut() {
        //             let mut new_pos = pos.clone();

        //         }
        //     }
        // }
        Some(KeyCode::ArrowLeft) => {
            let collision_detected = nearby_piece_reader.read().next().is_some();

            if !collision_detected {
                for (_, mut pos) in tetromino_pieces.iter_mut() {
                    pos.x -= 1
                }
            }
        }
        Some(KeyCode::ArrowRight) => {
            let collision_detected = nearby_piece_reader.read().next().is_some();

            if !collision_detected {
                for (_, mut pos) in tetromino_pieces.iter_mut() {
                    pos.x += 1
                }
            }
        }
        Some(KeyCode::ArrowDown) => {
            let collision_detected = nearby_piece_reader.read().next().is_some();

            if !collision_detected {
                for (_, mut pos) in tetromino_pieces.iter_mut() {
                    pos.y -= 1
                }
            }
        }
        _ => return,
    };
}

pub fn descend(
    time: Res<Time>,
    mut movement_timer: ResMut<MovementTimer>,
    mut tetromino_pieces: Query<Entity, With<TetrominoPiece>>,
    mut positions: Query<&mut Position>,
) {
    movement_timer.0.tick(time.delta());
    if !movement_timer.0.finished() {
        return;
    }

    for tetromino_piece in tetromino_pieces.iter_mut() {
        let mut piece = positions.get_mut(tetromino_piece).unwrap();

        piece.y -= 1
    }
}

pub fn check_collision(
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

pub fn check_movement_collision(
    mut tetromino_pieces: Query<Entity, With<TetrominoPiece>>,
    despawned_pieces: Query<Entity, With<DespawnedTetrominoPiece>>,
    mut positions: Query<&mut Position>,
    mut nearby_piece_writer: EventWriter<NearbyPieceEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let key_pressed = keyboard_input.get_just_pressed().next();

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
                let mut down_acc_pos = piece.clone();
                let mut down_movement_pos = piece.clone();
                down_movement_pos.y -= 1;
                down_acc_pos.y -= 2;

                if despawned_pieces_positions.contains(&down_movement_pos)
                    || despawned_pieces_positions.contains(&down_acc_pos)
                    || down_movement_pos.y == 0
                {
                    nearby_piece_writer.send(NearbyPieceEvent);
                }
            }
            _ => return,
        }
    }
}
