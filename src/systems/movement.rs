use bevy::prelude::*;

use crate::components::{Position, Tetromino};

pub fn move_down(time: Res<Time>, mut tetrominos: Query<Tetromino, With<Position>>) {
    if let Some((t_entity, mut t_pos)) = tetrominos.iter_mut().next() {}
}
