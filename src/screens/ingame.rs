use autodefault::*;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::{
    components::health::HealthBar,
    state::ScreenState,
    utils::{destroy_ui, destroy_ui_tag, UIRoot},
};
use crate::{
    game::PauseGame,
    level::{LevelCleared, LevelFailed},
    map::{CurrentLevel, SwitchLevelEvent},
};

#[derive(Component)]
struct WinMenuRoot;

#[derive(Component)]
struct LoseMenuRoot;

#[derive(Component)]
struct PauseMenuRoot;

#[derive(Component)]
enum ButtonTag {
    Resume,
    NextLevel,
    RetryLevel,
    MainMenu,
}

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(ScreenState::Ingame, render_ui)
            .add_system(render_win_menu.run_on_event::<LevelCleared>())
            .add_system(render_lose_menu.run_on_event::<LevelFailed>())
            .add_system(button_listener)
            .add_system(pause_menu_controller)
            .add_exit_system(ScreenState::Ingame, destroy_ui)
            .add_exit_system(ScreenState::Ingame, destroy_ui_tag::<WinMenuRoot>)
            .add_exit_system(ScreenState::Ingame, destroy_ui_tag::<LoseMenuRoot>);
    }
}

#[autodefault]
fn render_ui(mut cmd: Commands, assets: Res<AssetServer>) {
    cmd.spawn()
        .insert(UIRoot)
        .insert_bundle(NodeBundle {
            color: UiColor(Color::NONE),
            style: Style {
                align_self: AlignSelf::FlexEnd,
                // justify_content: JustifyContent::Center,
            },
        })
        .with_children(|parent| {
            HealthBar(parent, assets);
            // InventoryDisplay(&mut parent, assets);
        });
}

#[autodefault]
fn render_win_menu(mut cmd: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts/arcadeclassic.ttf");

    cmd.spawn()
        .insert(WinMenuRoot)
        .insert_bundle(NodeBundle {
            color: UiColor(Color::NONE),
            style: Style {
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
            },
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "Level cleared",
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.,
                        color: Color::WHITE,
                    },
                ),
            });

            for (tag, text) in vec![
                (ButtonTag::NextLevel, "Next Level"),
                (ButtonTag::RetryLevel, "Retry Level"),
                (ButtonTag::MainMenu, "Main Menu"),
            ] {
                parent
                    .spawn_bundle(ButtonBundle {
                        color: UiColor(Color::GRAY),
                        style: Style {
                            margin: UiRect::all(Val::Px(5.)),
                        },
                    })
                    .insert(tag)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::from_section(
                                text,
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 20.,
                                    color: Color::WHITE,
                                },
                            ),
                        });
                    });
            }
        });
}

// TODO don't really like how this is duplicating render_win_ui
#[autodefault]
fn render_lose_menu(mut cmd: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts/arcadeclassic.ttf");

    cmd.spawn()
        .insert(LoseMenuRoot)
        .insert_bundle(NodeBundle {
            color: UiColor(Color::NONE),
            style: Style {
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
            },
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "Level failed",
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.,
                        color: Color::WHITE,
                    },
                ),
            });

            for (tag, text) in vec![
                (ButtonTag::RetryLevel, "Retry Level"),
                (ButtonTag::MainMenu, "Main Menu"),
            ] {
                parent
                    .spawn_bundle(ButtonBundle {
                        color: UiColor(Color::GRAY),
                        style: Style {
                            margin: UiRect::all(Val::Px(5.)),
                        },
                    })
                    .insert(tag)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::from_section(
                                text,
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 20.,
                                    color: Color::WHITE,
                                },
                            ),
                        });
                    });
            }
        });
}

/// Respond to changing the [PauseGame] resource by spawning/despawning the ui
// TODO kinda wonk that this is calling two other systems directly
fn pause_menu_controller(
    cmd: Commands,
    assets: Res<AssetServer>,
    query: Query<Entity, With<PauseMenuRoot>>,
    paused: Res<PauseGame>,
) {
    if paused.is_changed() {
        if paused.0 {
            render_pause_menu(cmd, assets);
        } else {
            destroy_ui_tag(cmd, query);
        }
    }
}

#[autodefault]
fn render_pause_menu(mut cmd: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts/arcadeclassic.ttf");

    cmd.spawn()
        .insert(PauseMenuRoot)
        .insert_bundle(NodeBundle {
            color: UiColor(Color::NONE),
            style: Style {
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
            },
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "Paused",
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.,
                        color: Color::WHITE,
                    },
                ),
            });

            for (tag, text) in vec![
                (ButtonTag::Resume, "Resume"),
                (ButtonTag::RetryLevel, "Retry Level"),
                (ButtonTag::MainMenu, "Main Menu"),
            ] {
                parent
                    .spawn_bundle(ButtonBundle {
                        color: UiColor(Color::GRAY),
                        style: Style {
                            margin: UiRect::all(Val::Px(5.)),
                        },
                    })
                    .insert(tag)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::from_section(
                                text,
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 20.,
                                    color: Color::WHITE,
                                },
                            ),
                        });
                    });
            }
        });
}

fn button_listener(
    mut cmd: Commands,
    query: Query<(&Interaction, &ButtonTag), Changed<Interaction>>,
    current_level: Res<CurrentLevel>,
    mut switch_level_writer: EventWriter<SwitchLevelEvent>,
) {
    for (interaction, button_tag) in &query {
        match interaction {
            Interaction::Clicked => match button_tag {
                ButtonTag::Resume => {
                    cmd.insert_resource(PauseGame(false));
                },
                ButtonTag::NextLevel => {
                    switch_level_writer.send(SwitchLevelEvent {
                        level_index: current_level.0 + 1,
                    });
                },
                ButtonTag::MainMenu => {
                    cmd.insert_resource(NextState(ScreenState::MainMenu));
                },
                ButtonTag::RetryLevel => {
                    switch_level_writer.send(SwitchLevelEvent {
                        level_index: current_level.0,
                    });
                },
            },
            _ => {},
        }
    }
}
