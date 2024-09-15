use bevy::prelude::*;

use crate::{
    events::{DespawnTetrominoEvent, GameOverEvent, NearbyPieceEvent, UpdateScoreEvent},
    resources::{DescendTimer, Score, SpawnArea, TetrominoSegment},
    systems::{collision::*, events::*, movement::*, setup::*, spawning::*, translation_scale::*},
};

mod components;
mod constants;
mod events;
mod resources;
mod systems;

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
        .add_systems(Startup, (setup, spawn_tetromino))
        .add_systems(
            Update,
            (
                descend,
                spawn_new_tetromino.after(movement),
                game_over.after(spawn_new_tetromino),
                remove_row.after(spawn_new_tetromino),
            ),
        )
        .add_systems(
            PostUpdate,
            (
                movement,
                check_movement_collision.before(movement),
                check_descend_collision.before(descend),
                update_score.after(game_over),
                update_score.after(remove_row),
            ),
        )
        .add_systems(FixedUpdate, (size_scaling, pos_translation))
        .insert_resource(ClearColor(Color::rgb(0.200, 0.200, 0.200)))
        .insert_resource(DescendTimer::default())
        .insert_resource(Score::default())
        .insert_resource(SpawnArea::default())
        .insert_resource(TetrominoSegment::default())
        .add_event::<DespawnTetrominoEvent>()
        .add_event::<GameOverEvent>()
        .add_event::<NearbyPieceEvent>()
        .add_event::<UpdateScoreEvent>()
        .run();
}
