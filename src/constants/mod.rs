use std::f64::consts::PI;
use bevy::prelude::*;

/**
 *
 * Constants
 *
 **/


/*
 * Arena
 */
pub const ARENA_HEIGHT: u32 = 13;
pub const ARENA_WIDTH: u32 = 8;


/**
 * Tetrominos
 */
type AreaSize = [[i32; 4]; 3];

pub const ANGLE: f64 = 90.0 * (PI / 180.0);

pub const L_SHAPE: AreaSize = [
    [0, 1, 0, 0], 
    [0, 1, 0, 0], 
    [0, 1, 1, 0]
];

pub const SKEW: AreaSize = [
    [0, 1, 0, 0], 
    [0, 1, 1, 0], 
    [0, 0, 1, 0]
];

pub const SQUARE: AreaSize = [
    [0, 0, 0, 0], 
    [0, 1, 1, 0], 
    [0, 1, 1, 0]
];

pub const STRAIGHT: AreaSize = [
    [0, 0, 0, 0], 
    [1, 1, 1, 1], 
    [0, 0, 0, 0]
];

pub const T_SHAPE: AreaSize = [
    [0, 0, 1, 0], 
    [0, 1, 1, 0], 
    [0, 0, 1, 0]
];

pub const TETROMINO_VARIANTS: [AreaSize; 5] = [
    L_SHAPE, 
    SKEW,
    SQUARE, 
    STRAIGHT, 
    T_SHAPE, 
];

pub const TETROMINO_COLORS: [&str; 5] = [
    "orange",
    "green",
    "yellow",
    "red",
    "purple"
];
