pub mod arrow_trap;
pub mod collapsable_floor;

use bevy::prelude::*;

use self::{arrow_trap::ArrowTrapPlugin, collapsable_floor::CollapsableFloorPlugin};

pub struct EnviroPlugin;

impl Plugin for EnviroPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CollapsableFloorPlugin)
            .add_plugin(ArrowTrapPlugin);
    }
}
