use bevy::prelude::*;

use crate::components::{Position, Size};
use crate::constants::{ARENA_HEIGHT, ARENA_WIDTH};

pub fn pos_translation(mut window: Query<&mut Window>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, tile_size: f32) -> f32 {
        (pos * tile_size) - (bound_window / 2.0) + (tile_size / 2.0)
    }

    let primary_window = window.single_mut();

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(
                pos.x as f32,
                primary_window.width(),
                primary_window.width() / ARENA_WIDTH as f32,
            ),
            convert(
                pos.y as f32,
                primary_window.height(),
                primary_window.height() / ARENA_HEIGHT as f32,
            ),
            0.0,
        );
    }
}

pub fn size_scaling(mut window: Query<&mut Window>, mut q: Query<(&Size, &mut Transform)>) {
    let primary_window = window.single_mut();

    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * primary_window.width(),
            sprite_size.height / ARENA_HEIGHT as f32 * primary_window.height(),
            1.0,
        );
    }
}
