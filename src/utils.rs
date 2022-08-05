use bevy::prelude::*;

// TODO make a proper 2d direction utility
#[derive(Copy, Clone)]
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
        let clamped = v.clamp(IVec2::ZERO, IVec2::ONE);

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
