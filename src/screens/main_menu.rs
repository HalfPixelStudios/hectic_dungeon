use autodefault::*;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::{
    components::health::HealthBar,
    state::ScreenState,
    utils::{destroy_ui, UIRoot},
};
use crate::{assets::SpriteSheet, spritesheet_constants::SpriteIndex};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(ScreenState::MainMenu, render_ui)
            .add_exit_system(ScreenState::MainMenu, destroy_ui);
    }
}

#[autodefault]
fn render_ui(mut cmd: Commands, assets: Res<AssetServer>) {
    cmd.spawn()
        .insert(UIRoot)
        .insert_bundle(NodeBundle {
            color: UiColor(Color::NONE),
            style: Style {
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
            },
        })
        .with_children(|mut parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    color: UiColor(Color::GRAY),
                })
                .with_children(|mut parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::from_section(
                            "play",
                            TextStyle {
                                font: assets.load("fonts/arcadeclassic.ttf"),
                                font_size: 20.,
                                color: Color::BLACK,
                            },
                        ),
                    });
                });
        });
}
