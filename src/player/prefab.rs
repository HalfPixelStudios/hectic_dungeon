use bevy::prelude::*;
use bevy_bobs::prefab::{PrefabId, PrefabLib};
use serde::Deserialize;

use crate::spritesheet_constants::SpriteIndex;
use crate::buffs::StatsPrefab;

#[derive(Deserialize, Component)]
pub enum Class {
    Samurai,
    Magician,
    Summoner,
}

#[derive(Deserialize)]
pub struct PlayerPrefab {
    pub class: Class,
    pub sprite_index: SpriteIndex,
    pub default_weapon: PrefabId,
    pub default_ability: PrefabId,
    pub default_armor: PrefabId,
    pub base_stats: StatsPrefab
}

pub struct PrefabPlugin;

impl Plugin for PrefabPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PrefabLib::<PlayerPrefab>::new(RON_STRING));
    }
}

const RON_STRING: &str = r#"
{
    "samurai": (
        class: Samurai,
        sprite_index: Player,
        default_weapon: "hammer",
        default_ability: "",
        default_armor: "",
        base_stats: (
            max_health: 10,
            armor: 2,
            speed: 1,
            damage: 2,
            crit: 0.5
        )
    )
}
"#;
