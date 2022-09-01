use bevy::prelude::*;
use bevy_bobs::prefab::{PrefabId, PrefabLib};
use serde::Deserialize;

use crate::spritesheet::SpriteIndex;

#[derive(Deserialize, Component, PartialEq, Eq)]
pub enum Class {
    Warrior,
    Samurai,
    Magician,
    Summoner,
    Archer,
}

#[derive(Deserialize)]
pub struct PlayerPrefab {
    pub health: u32,
    pub class: Class,
    pub sprite_index: SpriteIndex,
    pub weapon: PrefabId,
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
        sprite_index: PlayerWarrior,
        weapon: "steel_sword",
    ),
    "archer": (
        health: 10,
        class: Archer,
        sprite_index: PlayerArcher,
        weapon: "wooden_bow",
    )
}
"#;
