use bevy::prelude::*;

/**
 *
 * Events
 *
 */
#[derive(Event)]
pub struct GameOverEvent;

#[derive(Event)]
pub struct DespawnTetrominoEvent;

#[derive(Event)]
pub struct NearbyPieceEvent;
