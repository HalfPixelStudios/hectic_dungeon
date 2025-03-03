use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_bobs::health_bar::HealthBarPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use iyes_loopless::state::NextState;

use super::{
    camera::CameraPlugin, enemy::EnemyPlugin, game::GamePlugin, grid::GridPlugin,
    material::MaterialPlugin, movement::MovementPlugin, player::PlayerPlugin,
    spritesheet::load_assets, ui::UIPlugin,
};
use crate::{
    ai::AIPlugin,
    attack::AttackPlugin,
    enviro::EnviroPlugin,
    item::ItemPlugin,
    level::LevelPlugin,
    map::MapPlugin,
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
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(window_descriptor);
    // .insert_resource(LogSettings {
    //     level: bevy::log::Level::DEBUG,
    //     ..default()
    // });

    app.add_plugins(DefaultPlugins).add_plugin(HealthBarPlugin);

    app.add_plugin(ScreensPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(AIPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(AttackPlugin)
        .add_plugin(ItemPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(MaterialPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(EnviroPlugin)
        .add_plugin(WeaponPlugin)
        .add_plugin(GridPlugin);

    if config.egui_enabled {
        app.add_plugin(WorldInspectorPlugin::new());
    }

    app.insert_resource(NextState(config.start_state));

    app.add_startup_system(load_assets);

    app.run();
}
