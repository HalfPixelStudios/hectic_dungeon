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
fn render_ui(mut cmd: Commands) {
    cmd.spawn()
        .insert(UIRoot)
        .insert_bundle(NodeBundle {
            color: UiColor(Color::NONE),
            style: Style {
                align_self: AlignSelf::FlexEnd,
                // justify_content: JustifyContent::Center,
            },
        })
        .with_children(|mut parent| {
            HealthBar(&mut parent);
        });
}
