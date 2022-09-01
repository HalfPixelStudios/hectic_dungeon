use bevy::prelude::*;
use bevy_bobs::prefab::{PrefabId, PrefabLib};
use serde::Deserialize;

use crate::spritesheet::SpriteIndex;

#[derive(Deserialize, Component)]
pub enum Class {
    Warrior,
    Samurai,
    Magician,
    Summoner,
}

#[derive(Deserialize)]
pub struct PlayerPrefab {
    pub health: u32,
    pub class: Class,
    pub sprite_index: SpriteIndex,
    pub default_primary: Option<PrefabId>,
    pub default_secondary: Option<PrefabId>,
    pub default_ability: Option<PrefabId>,
    pub default_armor: Option<PrefabId>,
}

pub struct PrefabPlugin;

impl Plugin for PrefabPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PrefabLib::<PlayerPrefab>::new(RON_STRING));
    }
}

const RON_STRING: &str = r#"
{
    "warrior": (
        health: 10,
        class: Warrior,
        sprite_index: Player,
        default_primary: Some("steel_sword"),
        default_secondary: None,
        default_ability: None,
        default_armor: None,
    )
}
"#;
