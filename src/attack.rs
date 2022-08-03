use bevy::prelude::*;

use crate::utils::Dir;

pub enum AttackPattern {
    StraightOne,
    StraightTwo,
    Hammer,
}

impl AttackPattern {
    // default north
    pub fn to_offsets(&self) -> Vec<IVec2> {
        match &self {
            AttackPattern::StraightOne => vec![IVec2::new(0, 1)],
            AttackPattern::StraightTwo => vec![IVec2::new(0, 1), IVec2::new(0, 2)],
            AttackPattern::Hammer => vec![
                IVec2::new(-1, 1),
                IVec2::new(0, 1),
                IVec2::new(1, 1),
                IVec2::new(-1, 2),
                IVec2::new(0, 2),
                IVec2::new(1, 2),
            ],
        }
    }
}

pub fn rotate_offsets(vecs: Vec<IVec2>, dir: Dir) -> Vec<IVec2> {
    match dir {
        Dir::North => vecs,
        Dir::East => vecs.into_iter().map(|v| IVec2::new(-v.y, v.x)).collect(),
        Dir::South => vecs.into_iter().map(|v| IVec2::new(-v.x, -v.y)).collect(),
        Dir::West => vecs.into_iter().map(|v| IVec2::new(v.y, -v.x)).collect(),
    }
}
