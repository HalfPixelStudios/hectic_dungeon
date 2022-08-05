use bevy::prelude::*;
use bevy_bobs::prefab::{PrefabId, PrefabLib};
use serde::Deserialize;

use crate::attack::AttackPattern;

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
    "dagger": (
        attack_pattern: StraightOne,
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
