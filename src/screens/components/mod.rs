pub mod health;
pub mod inventory;

use bevy::prelude::*;

use self::{health::HealthPlugin, inventory::InventoryPlugin};

pub struct ComponentPlugin;

impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InventoryPlugin).add_plugin(HealthPlugin);
    }
}
