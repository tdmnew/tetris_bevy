use bevy::prelude::*;

use crate::components::Position;

/*
 *
 * Resources
 *
 */

#[derive(Resource, Default, Deref, DerefMut, Debug)]
pub struct TetrominoSegment(pub Vec<Entity>);

#[derive(Resource, Deref, Debug, Default, DerefMut)]
pub struct SpawnArea(pub Vec<Position>);

#[derive(Resource)]
pub struct DescendTimer(pub Timer);
impl Default for DescendTimer {
    fn default() -> Self  {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

#[derive(Resource, Debug)]
pub struct Score(pub i32);
impl Default for Score {
    fn default() -> Self  {
        Self(0)
    }
}
