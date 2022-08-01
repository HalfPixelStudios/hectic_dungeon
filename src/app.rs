use bevy::{animation::*, asset::AssetLoader, log::LogSettings, prelude::*};
use bevy_prototype_debug_lines::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use super::{
    animation::AnimatePlugin,
    assets::*,
    camera::CameraPlugin,
    enemy::{EnemyPlugin, SpawnEnemyEvent},
    game::{GamePlugin, GameState},
    grid::GridPlugin,
    movement::MovementPlugin,
    player::{PlayerMovedEvent, PlayerPlugin, SpawnPlayerEvent},
    ui::UIPlugin,
};
use crate::map::MapPlugin;

pub fn app() {
    let window_descriptor = WindowDescriptor {
        present_mode: bevy::window::PresentMode::Fifo,
        title: "bevy_test".into(),
        ..default()
    };

    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(window_descriptor);
    // .insert_resource(LogSettings {
    //     level: bevy::log::Level::DEBUG,
    //     ..default()
    // });

    app.add_loopless_state(GameState::EnemyPhase)
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(GamePlugin)
        .add_plugin(AssetLoadPlugin)
        .add_plugin(PlayerPlugin)
        // .add_plugin(AnimatePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(GridPlugin);

    app.add_startup_system(setup).add_system(debug);

    app.run();
}

fn setup(
    mut spawn_player: EventWriter<SpawnPlayerEvent>,
    mut spawn_enemy: EventWriter<SpawnEnemyEvent>,
) {
    spawn_player.send(SpawnPlayerEvent {
        spawn_pos: Vec2::ZERO,
    });
    spawn_enemy.send(SpawnEnemyEvent {
        spawn_pos: Vec2::new(96., 96.),
    });
}

fn debug(
    keys: Res<Input<KeyCode>>,
    mut spawn_enemy: EventWriter<SpawnEnemyEvent>,
    mut player_move: EventWriter<PlayerMovedEvent>,
) {
    if keys.just_pressed(KeyCode::Q) {
        spawn_enemy.send(SpawnEnemyEvent {
            spawn_pos: Vec2::new(96., 96.),
        });
    }
    if keys.just_pressed(KeyCode::Y) {
        player_move.send(PlayerMovedEvent);
    }
}
