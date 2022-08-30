mod components;
mod ingame;
mod state;
mod utils;

use bevy::prelude::*;
use iyes_loopless::state::NextState;

use self::{
    components::ComponentPlugin,
    ingame::IngamePlugin,
    state::{ScreenState, StatePlugin},
};

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(StatePlugin)
            .add_plugin(IngamePlugin)
            .add_plugin(ComponentPlugin)
            .add_startup_system(debug);
    }
}

fn debug(mut cmd: Commands) {
    cmd.insert_resource(NextState(ScreenState::Ingame));
}
