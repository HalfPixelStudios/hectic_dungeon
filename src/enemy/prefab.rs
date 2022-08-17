use bevy::prelude::*;
use bevy_bobs::prefab::PrefabLib;
use serde::Deserialize;

use crate::spritesheet_constants::SpriteIndex;

#[derive(Deserialize)]
pub struct EnemyPrefab {
    pub health: u32,
    pub ai: AI,
    pub weapon_id: String,
    pub sprite_index: SpriteIndex,
}

#[derive(Deserialize)]
pub enum AI {
    Simple { attack_range: f32 },
}

pub struct PrefabPlugin;

impl Plugin for PrefabPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PrefabLib::<EnemyPrefab>::new(RON_STRING));
    }
}

const RON_STRING: &str = r#"
{
    "orc_swordsman": (
        health: 3,
        ai: Simple ( attack_range: 3. ),
        weapon_id: "steel_sword",
        sprite_index: OrcSwordsman
    )
}
"#;
