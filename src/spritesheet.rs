use std::collections::HashMap;

use bevy::prelude::*;
use serde::Deserialize;

pub const TILESHEET_WIDTH: u32 = 16;
pub const TILESHEET_HEIGHT: u32 = 16;
pub const SPRITE_SIZE: u32 = 8;

// beings
#[derive(Deserialize, Clone, Copy)]
pub enum SpriteIndex {
    PlayerWarrior = sprite!(0, 3),
    PlayerArcher = sprite!(1, 3),
    OrcDagger = sprite!(1, 4),
    OrcSwordsman = sprite!(2, 4),
    OrcTwinblade = sprite!(3, 4),

    SteelSword = sprite!(2, 5),

    Barrel = sprite!(0, 7),

    AttackIndicator = sprite!(0, 8),
    // MoveIndicatorW = sprite!(4, 10),
    // MoveIndicatorN = sprite!(5, 10),
    // MoveIndicatorS = sprite!(6, 10),
    // MoveIndicatorE = sprite!(7, 10),
    ItemSlotBg = sprite!(0, 11),
    WeaponSlot = sprite!(1, 11),
    ArmorSlot = sprite!(2, 11),
    AbilitySlot = sprite!(3, 11),

    HeartFull = sprite!(0, 12),
    HeartEmpty = sprite!(0, 13),
}

// ui
// pub const PLAYER_ATTACK_ANIM: Vec<(u32, u32)> = vec![(0, 9), (1, 9), (2, 9), (3, 9)];
// pub const ENEMY_ATTACK_ANIM: Vec<(u32, u32)> = vec![(0, 10), (1, 10), (2, 10)];

pub enum SpriteFrames {
    PlayerAttack,
    EnemyAttack,
}

impl SpriteFrames {
    fn value(&self) -> Vec<(u32, u32)> {
        match self {
            Self::PlayerAttack => vec![(0, 9), (1, 9), (2, 9), (3, 9)],
            Self::EnemyAttack => vec![(0, 10), (1, 10), (2, 10)],
        }
    }

    pub fn frames(&self) -> Vec<usize> {
        self.value()
            .iter()
            .map(|(x, y)| sprite!(x, y) as usize)
            .collect()
    }
}

macro_rules! sprite {
    ($x:expr, $y:expr) => {
        ($y * TILESHEET_HEIGHT + $x) as isize
    };
}
pub(crate) use sprite;

#[derive(Deref)]
pub struct SpriteSheet(pub Handle<TextureAtlas>);

pub fn load_assets(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut data: HashMap<String, HandleUntyped> = HashMap::new();

    let tilesheet_handle = texture_atlases.add(TextureAtlas::from_grid(
        assets.load("tilesheet/bandit_hideout.png"),
        Vec2::new(SPRITE_SIZE as f32, SPRITE_SIZE as f32),
        TILESHEET_WIDTH as usize,
        TILESHEET_HEIGHT as usize,
    ));
    cmd.insert_resource(SpriteSheet(tilesheet_handle));
}

pub struct SpritesheetPlugin;

impl Plugin for SpritesheetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_assets);
    }
}
