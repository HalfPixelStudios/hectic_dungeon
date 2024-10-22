use bevy::prelude::*;
use bevy_bobs::prefab::{PrefabId, PrefabLib};
use serde::Deserialize;

use crate::prelude::*;

#[derive(Deserialize)]
pub enum Damage {
    Fixed(u32),
    Range(u32, u32),
}

#[derive(Deserialize)]
pub struct WeaponPrefab {
    pub display_name: Option<String>,
    pub attack_pattern: AttackPattern,
    pub damage: Damage,
}

#[derive(Component, Deref, DerefMut)]
pub struct CurrentWeapon(pub PrefabId);

const RON_STRING: &str = r#"
{
    "hammer": (
        attack_pattern: Hammer,
        damage: Fixed(1),
    ),
    "steel_dagger": (
        attack_pattern: StraightOne,
        damage: Fixed(1),
    ),
    "twin_blade": (
        attack_pattern: TwinBlade,
        damage: Fixed(1),
    ),
    "steel_sword": (
        attack_pattern: StraightTwo,
        damage: Fixed(1),
    ),
    "wooden_bow": (
        attack_pattern: PointThree,
        damage: Fixed(1),
    ),
    "arrow_trap": (
        attack_pattern: StraightSix,
        damage: Fixed(1),
    ),
}
"#;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PrefabLib::<WeaponPrefab>::new(RON_STRING));
    }
}
