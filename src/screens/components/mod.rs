pub mod inventory;

use bevy::prelude::*;

use self::inventory::InventoryPlugin;

pub struct ComponentPlugin;

impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InventoryPlugin);
    }
}
