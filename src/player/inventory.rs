use bevy::prelude::*;
use bevy_bobs::prefab::PrefabId;

#[derive(Component)]
pub struct Inventory {
    pub weapon_primary: PrefabId,
    pub weapon_secondary: PrefabId,
    pub armor: PrefabId,
    pub ability: PrefabId,
    // pub slots: Vec<PrefabId>,
    // capactiy: u32,
}

impl Inventory {}
