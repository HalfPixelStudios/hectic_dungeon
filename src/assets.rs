use std::collections::HashMap;

use bevy::{prelude::*, reflect::TypeUuid};
use serde::*;

use super::animation::*;
use crate::{
    prefab::*,
    spritesheet_constants::{SPRITE_SIZE, TILESHEET_HEIGHT, TILESHEET_WIDTH},
};

#[derive(Deref)]
pub struct SpriteSheet(pub Handle<TextureAtlas>);

#[derive(Debug, Deref)]
pub struct PrefabData(pub HashMap<String, HandleUntyped>);

#[derive(Deserialize, TypeUuid, Debug)]
#[uuid = "e60395c0-f873-41dc-adfa-42d3ca74b8fc"]
pub struct BeingPrefab {
    pub display_name: String,
    pub anim: AnimationPrefab,
}

pub fn load_assets(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut data: HashMap<String, HandleUntyped> = HashMap::new();

    let tilesheet_handle = texture_atlases.add(TextureAtlas::from_grid(
        assets.load("tilesheet/bandit_hideout.png"),
        Vec2::new(SPRITE_SIZE as f32, SPRITE_SIZE as f32),
        TILESHEET_WIDTH as usize,
        TILESHEET_HEIGHT as usize,
    ));
    cmd.insert_resource(SpriteSheet(tilesheet_handle));

    // data.insert(
    //     "archer".to_string(),
    //     assets.load_untyped("beings/archer.being"),
    // );

    cmd.insert_resource(PrefabData(data));
}
pub struct AssetLoadPlugin;

impl Plugin for AssetLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<BeingPrefab>::new(&["being"]))
            .add_startup_system(load_assets);
    }
}
