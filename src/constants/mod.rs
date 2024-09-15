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

//
// The vec! will keep its index position, we're just shifting the positions around not their
// position
//
// therefore the first adjacent bottom or right piece will still be the axis
//
// second item in the array should always be the axis?
//

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

pub const TETROMINO_COLORS: [Color; 5] = [
    Color::rgb(0.255, 0.165, 0.1),
    Color::rgb(0.1, 0.128, 0.1),
    Color::rgb(0.255, 0.255, 0.1),
    Color::rgb(0.255, 0.1, 0.1),
    Color::rgb(0.128, 0.1, 0.128),
];
