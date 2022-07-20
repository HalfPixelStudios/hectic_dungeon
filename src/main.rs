use bevy::asset::AssetLoader;
use bevy::prelude::*;
use bevy::animation::*;
use hectic_dungeon::animation::AnimatePlugin;
use hectic_dungeon::assets::*;
use hectic_dungeon::camera::CameraPlugin;
use hectic_dungeon::movement::MovementPlugin;
use hectic_dungeon::player::SpawnPlayerEvent;
pub enum AppState{
    Menu,
    InGame,
}

use hectic_dungeon::{player::PlayerPlugin, ui::UIPlugin};

fn main() {
    App::new() 
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(WindowDescriptor{
            present_mode: bevy::window::PresentMode::Fifo,
            title: "Hectic Dungeon".into(),
            ..default()})
        .add_plugins(DefaultPlugins)
        .add_plugin(AssetLoadPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AnimatePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(MovementPlugin)
        .add_system(debug)
        .run();
}

fn debug(
    keys: Res<Input<KeyCode>>,
    mut spawn_player: EventWriter<SpawnPlayerEvent>,
) {
    if keys.just_pressed(KeyCode::T) {
        info!("send event");
        spawn_player.send(SpawnPlayerEvent{spawn_pos: Vec2::ZERO});

    }
    if keys.just_pressed(KeyCode::Q) {
    }
}
