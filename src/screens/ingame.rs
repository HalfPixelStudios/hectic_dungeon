use autodefault::*;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::{
    components::health::HealthBar,
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
fn render_ui(mut cmd: Commands, assets: Res<AssetServer>, asset_sheet: Res<SpriteSheet>) {
    cmd.spawn()
        .insert(UIRoot)
        .insert_bundle(NodeBundle {
            style: Style {
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
            },
        })
        .with_children(|mut parent| {
            HealthBar(&mut parent, &assets);
            /*
            parent
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: SpriteIndex::ItemSlotBg as usize,
                        ..default()
                    },
                    texture_atlas: asset_sheet.clone(),
                })
                .insert(Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Px(5.),
                        right: Val::Px(300.)
                    }
                })
                .insert(Node::default())
                .insert(CalculatedSize::default());
            */
        });
}
