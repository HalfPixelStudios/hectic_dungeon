use bevy::{
    animation::*, asset::AssetLoader, log::LogSettings, prelude::*, render::texture::ImageSettings,
};
use bevy_bobs::health_bar::HealthBarPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use iyes_loopless::{prelude::AppLooplessStateExt, state::NextState};

use super::{
    animation::AnimatePlugin,
    assets::*,
    camera::CameraPlugin,
    enemy::{EnemyPlugin, SpawnEnemyEvent},
    game::{GamePlugin, GameState},
    grid::GridPlugin,
    material::MaterialPlugin,
    movement::MovementPlugin,
    player::{PlayerMovedEvent, PlayerPlugin, SpawnPlayerEvent},
    ui::UIPlugin,
};
use crate::{
    ability::AbilityPlugin,
    ai::AIPlugin,
    attack::AttackPlugin,
    enviro::EnviroPlugin,
    item::ItemPlugin,
    map::MapPlugin,
    room::RoomPlugin,
    screens::{state::ScreenState, ScreensPlugin},
    weapon::WeaponPlugin,
};

pub struct AppConfig {
    pub fullscreen: bool,
    pub egui_enabled: bool,
    pub start_state: ScreenState,
}

pub fn app(config: AppConfig) {
    let mut window_descriptor = WindowDescriptor {
        present_mode: bevy::window::PresentMode::Fifo,
        title: "hectic_dungeon".into(),
        ..default()
    };

    if !config.fullscreen {
        window_descriptor.width = 800.;
        window_descriptor.height = 600.;
    }

    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(window_descriptor);
    // .insert_resource(LogSettings {
    //     level: bevy::log::Level::DEBUG,
    //     ..default()
    // });

    app.add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_plugin(AIPlugin)
        .add_plugin(AbilityPlugin)
        .add_plugin(AssetLoadPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(RoomPlugin)
        // .add_plugin(AnimatePlugin)
        .add_plugin(AttackPlugin)
        .add_plugin(ItemPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(MaterialPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(HealthBarPlugin)
        .add_plugin(EnviroPlugin)
        .add_plugin(WeaponPlugin)
        .add_plugin(GridPlugin)
        .add_plugin(ScreensPlugin);

    if config.egui_enabled {
        app.add_plugin(WorldInspectorPlugin::new());
    }

    app.insert_resource(NextState(config.start_state));

    app.run();
}
