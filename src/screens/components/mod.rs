pub mod health;
pub mod inventory;

use bevy::prelude::*;

use self::{health::HealthPlugin, inventory::InventoryDisplayPlugin};

pub struct ComponentPlugin;

impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InventoryDisplayPlugin)
            .add_plugin(HealthPlugin);
    }
}
