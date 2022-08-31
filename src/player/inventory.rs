use bevy::prelude::*;
use bevy_bobs::prefab::PrefabId;

#[derive(Component)]
pub struct Inventory {
    pub weapon_primary: Option<PrefabId>,
    pub weapon_secondary: Option<PrefabId>,
    pub armor: Option<PrefabId>,
    pub ability: Option<PrefabId>,
    // pub slots: Vec<PrefabId>,
    // capactiy: u32,
}

impl Inventory {}
