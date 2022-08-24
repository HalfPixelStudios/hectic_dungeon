use autodefault::*;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::{
    state::ScreenState,
    utils::{destroy_ui, UIRoot},
};
use crate::{assets::SpriteSheet, spritesheet_constants::SpriteIndex};

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(ScreenState::Ingame, render_ui)
            .add_exit_system(ScreenState::Ingame, destroy_ui);
    }
}

#[autodefault]
fn render_ui(mut cmd: Commands, asset_sheet: Res<SpriteSheet>) {
    cmd.spawn()
        .insert(UIRoot)
        .insert_bundle(NodeBundle {})
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: SpriteIndex::ItemSlotBg as usize,
                        ..default()
                    },
                    texture_atlas: asset_sheet.clone(),
                    transform: Transform {
                        translation: Vec2::ZERO.extend(10.),
                        ..default()
                    },
                })
                .insert(Style::default())
                .insert(Node::default())
                .insert(CalculatedSize::default());
        });
}
