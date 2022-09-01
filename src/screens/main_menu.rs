use autodefault::*;
use bevy::{app::AppExit, prelude::*};
use iyes_loopless::prelude::*;

use super::{
    state::ScreenState,
    utils::{destroy_ui, UIRoot},
};

pub struct MainMenuPlugin;

#[derive(Component)]
enum ButtonTag {
    Play,
    Settings,
    Quit,
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(ScreenState::MainMenu, render_ui)
            .add_system(button_listener.run_in_state(ScreenState::MainMenu))
            .add_exit_system(ScreenState::MainMenu, destroy_ui);
    }
}

#[autodefault]
fn render_ui(mut cmd: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts/arcadeclassic.ttf");

    let root = cmd
        .spawn()
        .insert(UIRoot)
        .insert_bundle(NodeBundle {
            color: UiColor(Color::NONE),
            style: Style {
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
            },
        })
        .id();

    cmd.entity(root).with_children(|mut parent| {
        for (tag, text) in vec![
            (ButtonTag::Play, "play"),
            (ButtonTag::Settings, "settings"),
            (ButtonTag::Quit, "quit"),
        ] {
            parent
                .spawn_bundle(ButtonBundle {
                    color: UiColor(Color::GRAY),
                    style: Style {
                        margin: UiRect::all(Val::Px(5.)),
                    },
                })
                .insert(tag)
                .with_children(|mut parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::from_section(
                            text,
                            TextStyle {
                                font: font.clone(),
                                font_size: 20.,
                                color: Color::BLACK,
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
    mut exit_writer: EventWriter<AppExit>,
) {
    for (interaction, button_tag) in &query {
        match interaction {
            Interaction::Clicked => match button_tag {
                ButtonTag::Play => {
                    cmd.insert_resource(NextState(ScreenState::Ingame));
                },
                ButtonTag::Settings => {},
                ButtonTag::Quit => exit_writer.send(AppExit),
            },
            _ => {},
        }
    }
}
