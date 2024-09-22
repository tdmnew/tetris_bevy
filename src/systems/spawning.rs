use bevy::prelude::*;
use rand::Rng;

use crate::{
    components::{Position, Size, TetrominoPiece},
    constants::{TETROMINO_COLORS, TETROMINO_VARIANTS},
    resources::{SpawnArea, TetrominoSegment},
};

fn draw_piece(
    commands: &mut Commands,
    color: &str,
    position: Position,
    server: &Res<AssetServer>,
) -> Entity {
    commands
        .spawn(SpriteBundle {
            texture: server.load(format!("{}-brick.png", color)),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(TetrominoPiece)
        .insert(Size::square(1.0))
        .insert(position)
        .id()
}

fn draw_tetromino_pieces(
    commands: &mut Commands,
    spawn_area: &mut SpawnArea,
    server: &Res<AssetServer>,
) -> Vec<Entity> {
    let mut entities: Vec<Entity> = vec![];

    let rand_index = rand::thread_rng().gen_range(0, TETROMINO_VARIANTS.len());

    for (y_index, y_axis) in TETROMINO_VARIANTS[rand_index].iter().enumerate() {
        for (x_index, segment) in y_axis.iter().enumerate() {
            if *segment == 1 {
                let piece_pos = Position {
                    x: (x_index as i32) + 2,
                    y: (y_index as i32) + 10,
                };

                entities.push(draw_piece(
                    commands,
                    TETROMINO_COLORS[rand_index],
                    piece_pos,
                    server,
                ));

                spawn_area.push(piece_pos);
            }
        }
    }

    entities
}

pub fn spawn_tetromino(
    mut commands: Commands,
    mut segments: ResMut<TetrominoSegment>,
    mut spawn_area: ResMut<SpawnArea>,
    server: Res<AssetServer>,
) {
    *segments = TetrominoSegment(draw_tetromino_pieces(
        &mut commands,
        &mut spawn_area,
        &server,
    ));
}
