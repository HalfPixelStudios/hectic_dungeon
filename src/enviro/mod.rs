pub mod collapsable_floor;

use bevy::prelude::*;

use self::collapsable_floor::CollapsableFloorPlugin;

pub struct EnviroPlugin;

impl Plugin for EnviroPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CollapsableFloorPlugin);
    }
}
