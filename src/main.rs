use bevy::prelude::*;

use crate::resources::MovementTimer;
use crate::systems::{spawning::*, translation_scale::*};

mod components;
mod constants;
mod resources;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetris Clone".to_string(),
                resolution: Vec2::new(400.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
        })
        .add_systems(Update, draw_square)
        .add_systems(PostUpdate, (size_scaling, pos_translation))
        .insert_resource(MovementTimer::default())
        .run();
}
