use bevy::prelude::*;

use crate::{
    components::{DespawnedTetrominoPiece, Position, TetrominoPiece},
    events::{DespawnTetrominoEvent, GameOverEvent},
    resources::{DespawnedTetrominoPieces, TetrominoSegment},
    systems::spawning::spawn_tetromino,
    SpawnArea,
};

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
