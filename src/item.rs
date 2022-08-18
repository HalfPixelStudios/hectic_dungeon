use bevy::prelude::*;
use bevy_bobs::prefab::PrefabLib;
use serde::Deserialize;

use crate::spritesheet_constants::SpriteIndex;

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
    pub item_type: ItemType,
    pub rarity: Rarity,
    pub sprite_index: SpriteIndex,
}

const RON_STRING: &str = r#"
{
    "steel_sword": (
        item_type: Weapon,
        rarity: Common,
        sprite_index: SteelSword,
    )
}
"#;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PrefabLib::<ItemPrefab>::new(RON_STRING));
    }
}
