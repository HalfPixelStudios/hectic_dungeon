use bevy::{animation::*, asset::AssetLoader, log::LogSettings, prelude::*};
use bevy_bobs::health_bar::HealthBarPlugin;
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
use crate::{
    ai::AIPlugin, attack::AttackPlugin, enviro::EnviroPlugin, map::MapPlugin, room::RoomPlugin,
    weapon::WeaponPlugin,
};

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

    app.add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_plugin(AIPlugin)
        .add_plugin(AssetLoadPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(RoomPlugin)
        // .add_plugin(AnimatePlugin)
        .add_plugin(AttackPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(HealthBarPlugin)
        .add_plugin(EnviroPlugin)
        .add_plugin(WeaponPlugin)
        .add_plugin(GridPlugin);

    app.run();
}
