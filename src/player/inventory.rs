use bevy::prelude::*;
use bevy_bobs::prefab::PrefabId;

#[derive(Component)]
pub struct Inventory {
    pub weapon: PrefabId,
    pub armor: PrefabId,
    pub ability: PrefabId,
    pub slots: Vec<PrefabId>,
    capacity: u32,
}

impl Inventory {}
