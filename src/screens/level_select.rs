use autodefault::autodefault;
use bevy::prelude::*;
use bevy_ecs_ldtk::LevelSelection;
use iyes_loopless::prelude::*;

use super::{
    state::ScreenState,
    utils::{destroy_ui, UIRoot},
};

pub enum DungeonName {
    Doceo,
}

pub struct DungeonInfo {
    pub dungeon_name: DungeonName,
    pub level_count: usize,
}

#[derive(Default)]
pub struct LevelSelectState {
    index: usize,
}

pub struct LevelSelectPlugin;

impl Plugin for LevelSelectPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelectState::default())
            .add_enter_system(ScreenState::LevelSelect, render_ui)
            .add_system(input.run_in_state(ScreenState::LevelSelect))
            .add_exit_system(ScreenState::LevelSelect, destroy_ui);
    }
}

const DUNGEON_INFO: DungeonInfo = DungeonInfo {
    dungeon_name: DungeonName::Doceo,
    level_count: 2,
};
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

    cmd.entity(root).with_children(|parent| {
        for level_index in 0..DUNGEON_INFO.level_count {
            parent.spawn_bundle(TextBundle {
                node: Node {
                    size: Vec2::new(16., 16.),
                },
                text: Text::from_section(
                    format!("{}", level_index),
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.,
                        color: Color::WHITE,
                    },
                ),
                style: Style {
                    margin: UiRect::all(Val::Px(4.)),
                },
            });
        }
    });
}

// TODO maybe use leafwing input manager
fn input(mut cmd: Commands, keys: Res<Input<KeyCode>>, mut state: ResMut<LevelSelectState>) {
    if keys.just_pressed(KeyCode::A) {
        state.index = std::cmp::max(state.index as i32 - 1, 0) as usize;
    }
    if keys.just_pressed(KeyCode::D) {
        state.index = std::cmp::min(state.index + 1, DUNGEON_INFO.level_count - 1);
    }
    if keys.just_pressed(KeyCode::Space) {
        cmd.insert_resource(NextState(ScreenState::Ingame));
        cmd.insert_resource(LevelSelection::Index(state.index));
    }
}
