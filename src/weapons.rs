use bevy::prelude::*;
use bevy_bobs::prefab::PrefabId;
use serde::Deserialize;

use crate::attack::AttackPattern;

#[derive(Deserialize)]
pub enum Damage {
    Fixed(u32),
    Range(u32, u32),
}

#[derive(Deserialize)]
pub struct WeaponPrefab {
    pub attack_pattern: AttackPattern,
    pub damage: Damage,
}

#[derive(Component)]
pub struct CurrentWeapon {
    id: PrefabId,
}
