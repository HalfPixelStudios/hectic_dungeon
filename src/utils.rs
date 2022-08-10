use bevy::prelude::*;

// TODO make a proper 2d direction utility
#[derive(Debug, Copy, Clone)]
pub enum Dir {
    None,

    North,
    East,
    South,
    West,

    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl From<IVec2> for Dir {
    fn from(v: IVec2) -> Self {
        let clamped = v.clamp(-IVec2::ONE, IVec2::ONE);

        // TODO can match directly in bevy 0.8
        match clamped.to_array() {
            [0, 0] => Dir::None,
            [0, 1] => Dir::North,
            [1, 0] => Dir::East,
            [0, -1] => Dir::South,
            [-1, 0] => Dir::West,
            [1, 1] => Dir::NorthEast,
            [-1, 1] => Dir::NorthWest,
            [1, -1] => Dir::SouthEast,
            [-1, -1] => Dir::SouthWest,
            _ => unreachable!(),
        }
    }
}

impl From<Dir> for IVec2 {
    fn from(d: Dir) -> Self {
        match d {
            Dir::None => IVec2::new(0, 0),
            Dir::North => IVec2::new(0, 1),
            Dir::East => IVec2::new(1, 0),
            Dir::South => IVec2::new(0, -1),
            Dir::West => IVec2::new(-1, 0),
            Dir::NorthEast => IVec2::new(1, 1),
            Dir::NorthWest => IVec2::new(-1, 1),
            Dir::SouthEast => IVec2::new(1, -1),
            Dir::SouthWest => IVec2::new(-1, -1),
            _ => unreachable!(),
        }
    }
}

impl From<String> for Dir {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "north" => Dir::North,
            "south" => Dir::South,
            "east" => Dir::East,
            "west" => Dir::West,
            _ => Dir::None,
        }
    }
}

pub fn to_rotation(dir: Dir) -> f32 {
    use std::f32::consts::PI;

    match dir {
        Dir::East => 0.,
        Dir::North => PI / 2.,
        Dir::West => PI,
        Dir::South => 3. * PI / 2.,
        _ => unreachable!(),
    }
}

pub fn cardinal_dirs() -> Vec<Dir> {
    vec![Dir::North, Dir::South, Dir::East, Dir::West]
}

pub fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

macro_rules! unwrap_or_return {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return,
        }
    };
}
pub(crate) use unwrap_or_return;

macro_rules! unwrap_or_continue {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => continue,
        }
    };
}
pub(crate) use unwrap_or_continue;
