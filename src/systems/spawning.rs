use bevy::prelude::*;

use crate::components::{Position, Size, Tetromino};

pub fn spawn_tetromino(mut commands: Commands) {}

pub fn draw_square(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 0.1, 0.1),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(Tetromino::default())
        .insert(Position { x: 7,  y: 11 })
        .insert(Size::square(1.0));
}
