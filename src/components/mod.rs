use bevy::prelude::*;

/**
 * Postiion and Scale
 **/
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

/**
 *  Entities
 **/
#[derive(Component)]
pub struct Tetromino {
    movement_timer: Timer,
}

impl Default for Tetromino {
    fn default() -> Self {
        Self {
            movement_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}
