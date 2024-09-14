use bevy::prelude::*;

use crate::{
    components::{DespawnedTetrominoPiece, Position, TetrominoPiece},
    constants::{ARENA_HEIGHT, ARENA_WIDTH},
    events::{DespawnTetrominoEvent, GameOverEvent},
    resources::{DespawnedTetrominoPieces, TetrominoSegment},
    systems::spawning::spawn_tetromino,
    SpawnArea,
};

pub fn game_over(
    mut commands: Commands,
    mut game_over_reader: EventReader<GameOverEvent>,
    mut tetromino_pieces: Query<Entity, With<TetrominoPiece>>,
    mut despawned_pieces: Query<Entity, With<DespawnedTetrominoPiece>>,
    segments: ResMut<TetrominoSegment>,
    spawn_area: ResMut<SpawnArea>,
) {
    if game_over_reader.read().next().is_some() {
        for ent in tetromino_pieces.iter_mut() {
            commands.entity(ent).despawn();
        }

        for ent in despawned_pieces.iter_mut() {
            commands.entity(ent).despawn();
        }

        spawn_tetromino(commands, segments, spawn_area);
    }
}

pub fn spawn_new_tetromino(
    mut commands: Commands,
    mut game_over_writer: EventWriter<GameOverEvent>,
    mut despawn_reader: EventReader<DespawnTetrominoEvent>,
    tetromino_pieces: Query<Entity, With<TetrominoPiece>>,
    mut segments: ResMut<TetrominoSegment>,
    mut despawned_pieces: ResMut<DespawnedTetrominoPieces>,
    positions: Query<&mut Position>,
    spawn_area: ResMut<SpawnArea>,
) {
    if despawn_reader.read().next().is_some() {
        segments.clear();

        for piece in tetromino_pieces.iter() {
            let in_spwan_area = spawn_area
                .iter()
                .any(|p| p == positions.get(piece).unwrap());

            if in_spwan_area {
                game_over_writer.send(GameOverEvent);
                return;
            }

            commands
                .entity(piece)
                .remove::<TetrominoPiece>()
                .insert(DespawnedTetrominoPiece);

            despawned_pieces.push(piece);
        }

        spawn_tetromino(commands, segments, spawn_area);
    }
}

pub fn remove_row(
    mut commands: Commands,
    despawned_pieces: Query<Entity, With<DespawnedTetrominoPiece>>,
    mut positions: Query<&mut Position>,
) {
    for y in 0..ARENA_HEIGHT {
        let mut y_axis_pieces = despawned_pieces
            .iter()
            .filter(|e| {
                let pos = positions.get_mut(*e).unwrap();
                pos.y == (y as i32)
            })
            .collect::<Vec<Entity>>();

        if y_axis_pieces.len() == (ARENA_WIDTH as usize) {
            for ent in y_axis_pieces.iter_mut() {
                commands.entity(*ent).despawn();
            }

            for row in (y + 1)..ARENA_HEIGHT {
                let row_above = despawned_pieces
                    .iter()
                    .filter(|p| {
                        let pos = positions.get_mut(*p).unwrap();
                        pos.y == (row as i32)
                    })
                    .collect::<Vec<Entity>>();

                for piece in row_above {
                    let mut position = positions.get_mut(piece).unwrap();
                    position.y -= 1
                }
            }
        }
    }
}
