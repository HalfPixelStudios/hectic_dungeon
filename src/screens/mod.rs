mod components;
mod ingame;
mod main_menu;
pub mod state;
mod utils;

use bevy::prelude::*;

use self::{
    components::ComponentPlugin, ingame::IngamePlugin, main_menu::MainMenuPlugin,
    state::StatePlugin,
};

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(StatePlugin)
            .add_plugin(IngamePlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(ComponentPlugin)
            .add_startup_system(debug);
    }
}

fn debug(mut cmd: Commands) {
    // cmd.insert_resource(NextState(ScreenState::Ingame));
}
