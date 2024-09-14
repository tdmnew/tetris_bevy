use bevy::prelude::*;

use crate::{
    events::{DespawnTetrominoEvent, NearbyPieceEvent, GameOverEvent},
    resources::*,
    systems::{events::*, movement::*, spawning::*, translation_scale::*},
};

mod components;
mod constants;
mod events;
mod resources;
mod systems;

// 
// Screen Resolutions
// 400, 600
// 500, 800


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetris Clone".to_string(),
                resolution: Vec2::new(500.0, 800.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(
            Startup,
            (
                |mut commands: Commands| {
                    commands.spawn(Camera2dBundle::default());
                },
                spawn_tetromino,
            ),
        )
        .add_systems(
            Update,
            (
                descend,
                movement,
                check_movement_collision.before(movement),
                check_descend_collision.before(descend),
                spawn_new_tetromino.after(movement),
                game_over.after(spawn_new_tetromino),
                remove_row.after(spawn_new_tetromino),
            ),
        )
        .add_systems(FixedUpdate, (size_scaling, pos_translation))
        .insert_resource(TetrominoSegment::default())
        .insert_resource(DespawnedTetrominoPieces::default())
        .insert_resource(MovementTimer::default())
        .insert_resource(SpawnArea::default())
        .add_event::<DespawnTetrominoEvent>()
        .add_event::<NearbyPieceEvent>()
        .add_event::<GameOverEvent>()
        .run();
}
