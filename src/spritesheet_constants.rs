use serde::Deserialize;

pub const TILESHEET_WIDTH: usize = 16;
pub const TILESHEET_HEIGHT: usize = 16;
pub const SPRITE_SIZE: usize = 8;

// beings
#[derive(Deserialize, Clone, Copy)]
pub enum SpriteIndex {
    Player = sprite!(0, 3),
    OrcSwordsman = sprite!(2, 4),
    OrcTwinblade = sprite!(3, 4),

    AttackIndicator = sprite!(0, 8),
    // MoveIndicatorW = sprite!(4, 10),
    // MoveIndicatorN = sprite!(5, 10),
    // MoveIndicatorS = sprite!(6, 10),
    // MoveIndicatorE = sprite!(7, 10),
}

// ui
// pub const PLAYER_ATTACK_ANIM: Vec<(u32, u32)> = vec![(0, 9), (1, 9), (2, 9), (3, 9)];
// pub const ENEMY_ATTACK_ANIM: Vec<(u32, u32)> = vec![(0, 10), (1, 10), (2, 10)];

macro_rules! sprite {
    ($x:literal, $y:literal) => {
        ($y * TILESHEET_HEIGHT + $x) as isize
    };
}
pub(crate) use sprite;
