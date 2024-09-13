use bevy::prelude::*;

use crate::components::Position;

/*
 *
 * Resources
 *
 */

#[derive(Resource, Default, Deref, DerefMut, Debug)]
pub struct TetrominoSegment(pub Vec<Entity>);

// Not needed?
#[derive(Resource, Deref, Debug, DerefMut)]
pub struct DespawnedTetrominoPieces(pub Vec<Entity>);

impl Default for DespawnedTetrominoPieces {
    fn default() -> Self {
        DespawnedTetrominoPieces(vec![])
    }
}

#[derive(Resource)]
pub struct MovementTimer(pub Timer);
impl Default for MovementTimer {
    fn default() -> Self  {
        Self(Timer::from_seconds(0.25, TimerMode::Repeating))
    }
}

#[derive(Resource, Deref, Debug, Default, DerefMut)]
pub struct SpawnArea(pub Vec<Position>);
