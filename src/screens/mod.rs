mod components;
mod ingame;
mod level_select;
mod main_menu;
pub mod state;
mod utils;

use bevy::prelude::*;

use self::{
    components::ComponentPlugin, ingame::IngamePlugin, level_select::LevelSelectPlugin,
    main_menu::MainMenuPlugin, state::StatePlugin,
};

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(StatePlugin)
            .add_plugin(IngamePlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(LevelSelectPlugin)
            .add_plugin(ComponentPlugin)
            .add_startup_system(debug);
    }
}

fn debug(_cmd: Commands) {
    // cmd.insert_resource(NextState(ScreenState::Ingame));
}
