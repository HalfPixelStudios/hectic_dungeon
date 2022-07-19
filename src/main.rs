use bevy::prelude::*;

use hectic_dungeon::{player::PlayerPlugin, ui::UIPlugin};

fn main() {
    let window_descriptor = WindowDescriptor {
        present_mode: bevy::window::PresentMode::Fifo,
        title: "hectic_dungeon".into(),
        ..default()
    };

    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(window_descriptor);

    // 3rd party plugins
    app.add_plugins(DefaultPlugins);

    // internal plugins
    app.add_plugin(UIPlugin).add_plugin(PlayerPlugin);

    // systems
    app.add_startup_system(setup);

    app.run();
}

fn setup(mut commands: Commands) {}
