use bevy::prelude::*;

// TODO make a proper 2d direction utility
pub enum Dir {
    North,
    East,
    South,
    West,
}

pub enum AttackPattern {
    StraightOne,
    StraightTwo,
}

impl AttackPattern {
    // default north
    pub fn to_offsets(&self, dir: Dir) -> Vec<IVec2> {
        let north = match self {
            AttackPattern::StraightOne => vec![IVec2::new(0, 1)],
            AttackPattern::StraightTwo => vec![IVec2::new(0, 1), IVec2::new(0, 2)],
        };

        // rotate offsets based on direction
        rotate_offsets(north, dir)
    }
}

fn rotate_offsets(vecs: Vec<IVec2>, dir: Dir) -> Vec<IVec2> {
    match dir {
        Dir::North => vecs,
        Dir::East => vecs.into_iter().map(|v| IVec2::new(-v.y, v.x)).collect(),
        Dir::South => vecs.into_iter().map(|v| IVec2::new(-v.x, -v.y)).collect(),
        Dir::West => vecs.into_iter().map(|v| IVec2::new(v.y, -v.x)).collect(),
    }
}

#[derive(Component)]
pub struct AttackIndicator {
    pub dir: Dir,
    pub pattern: AttackPattern,
}
