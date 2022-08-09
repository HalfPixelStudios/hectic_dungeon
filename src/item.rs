use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub enum ItemType {
    Weapon,
    Armor,
    Potion,
}

// TODO be more creative with rarity teirs
#[derive(Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

#[derive(Deserialize)]
pub struct ItemPrefab {
    pub display_name: String,
    pub item_type: ItemType,
    pub rarity: Rarity,
}
