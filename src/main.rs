use bevy::asset::AssetLoader;
use bevy::prelude::*;
use bevy::animation::*;
use hectic_dungeon::animation::AnimatePlugin;
use hectic_dungeon::assets::*;
use hectic_dungeon::camera::CameraPlugin;
use hectic_dungeon::enemy::EnemyPlugin;
use hectic_dungeon::enemy::SpawnEnemyEvent;
use hectic_dungeon::game::GamePlugin;
use hectic_dungeon::game::GameState;
use hectic_dungeon::grid::GridPlugin;
use hectic_dungeon::movement::MovementPlugin;
use hectic_dungeon::player::PlayerMovedEvent;
use hectic_dungeon::player::SpawnPlayerEvent;


use hectic_dungeon::{player::PlayerPlugin, ui::UIPlugin};
use iyes_loopless::prelude::AppLooplessStateExt;

fn main() {
    App::new() 
        .add_loopless_state(GameState::EnemyPhase)
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(WindowDescriptor{
            present_mode: bevy::window::PresentMode::Fifo,
            title: "Hectic Dungeon".into(),
            ..default()})
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_plugin(AssetLoadPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AnimatePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(GridPlugin)
        .add_system(debug)
        .run();
}

fn debug(
    keys: Res<Input<KeyCode>>,
    mut spawn_player: EventWriter<SpawnPlayerEvent>,
    mut spawn_enemy: EventWriter<SpawnEnemyEvent>,
    mut player_move: EventWriter<PlayerMovedEvent>
) {
    if keys.just_pressed(KeyCode::T) {
        info!("send event");
        spawn_player.send(SpawnPlayerEvent{spawn_pos: Vec2::ZERO});

    }
    if keys.just_pressed(KeyCode::Q) {
        spawn_enemy.send(SpawnEnemyEvent{spawn_pos: Vec2::new(96.,96.)});
        
    }
    if keys.just_pressed(KeyCode::Y) {
        player_move.send(PlayerMovedEvent);
    }
}
