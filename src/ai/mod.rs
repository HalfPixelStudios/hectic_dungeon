pub mod simple_ai;

use bevy::prelude::*;
use big_brain::BigBrainPlugin;

use self::simple_ai::SimpleAIPlugin;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BigBrainPlugin).add_plugin(SimpleAIPlugin);
    }
}
