use bevy::prelude::*;

// TODO make a proper 2d direction utility
#[derive(Copy, Clone)]
pub enum Dir {
    North,
    East,
    South,
    West,
}
