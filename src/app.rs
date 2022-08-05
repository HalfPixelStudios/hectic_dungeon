use bevy::{animation::*, asset::AssetLoader, log::LogSettings, prelude::*};
use bevy_bobs::health_bar::HealthBarPlugin;
use bevy_editor_pls::prelude::*;
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
use crate::{attack::AttackPlugin, map::MapPlugin, weapon::WeaponPlugin};

pub fn app() {
    let window_descriptor = WindowDescriptor {
        present_mode: bevy::window::PresentMode::Fifo,
        title: "hectic_dungeon".into(),
        width: 800.,
        height: 600.,
        ..default()
    };

    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
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
        .add_plugin(AttackPlugin)
        .add_plugin(EditorPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(HealthBarPlugin)
        .add_plugin(WeaponPlugin)
        .add_plugin(GridPlugin);

    app.add_startup_system(setup).add_system(debug);

    app.run();
}

fn setup(
    mut spawn_player: EventWriter<SpawnPlayerEvent>,
    mut spawn_enemy: EventWriter<SpawnEnemyEvent>,
) {
    spawn_player.send(SpawnPlayerEvent {
        spawn_pos: IVec2::new(8, 8),
    });
}

fn debug(
    keys: Res<Input<KeyCode>>,
    mut spawn_enemy: EventWriter<SpawnEnemyEvent>,
    mut player_move: EventWriter<PlayerMovedEvent>,
) {
    // if keys.just_pressed(KeyCode::Y) {
    //     player_move.send(PlayerMovedEvent);
    // }
}
