use bevy::prelude::*;
use bevy_bobs::prefab::PrefabLib;
use serde::Deserialize;

use crate::spritesheet::SpriteIndex;

#[derive(Deserialize)]
pub struct EnemyPrefab {
    pub health: u32,
    pub ai: AI,
    pub weapon_id: String,
    pub sprite_index: SpriteIndex,
    pub drops: Vec<(String, f32)>,
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
    "orc_dagger": (
        health: 3,
        ai: Simple ( attack_range: 2. ),
        weapon_id: "steel_dagger",
        sprite_index: OrcDagger,
        drops: []
    ),
    "orc_swordsman": (
        health: 3,
        ai: Simple ( attack_range: 3. ),
        weapon_id: "steel_sword",
        sprite_index: OrcSwordsman,
        drops: [("steel_sword", 0.5)]
    ),
    "orc_twinblade": (
        health: 3,
        ai: Simple ( attack_range: 3. ),
        weapon_id: "twin_blade",
        sprite_index: OrcTwinblade,
        drops: []
    ),
}
"#;
