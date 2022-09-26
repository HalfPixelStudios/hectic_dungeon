pub mod arrow_trap;
pub mod collapsable_floor;
pub mod destructible;
pub mod door;
pub mod dropped_item;
pub mod spike_trap;
pub mod water_tile;

use bevy::prelude::*;

use self::{
    arrow_trap::ArrowTrapPlugin, collapsable_floor::CollapsableFloorPlugin,
    destructible::DestructiblePlugin, door::DoorPlugin, dropped_item::DroppedItemPlugin,
    spike_trap::SpikeTrapPlugin, water_tile::WaterTilePlugin,
};

pub struct EnviroPlugin;

impl Plugin for EnviroPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CollapsableFloorPlugin)
            .add_plugin(ArrowTrapPlugin)
            .add_plugin(DoorPlugin)
            .add_plugin(DroppedItemPlugin)
            .add_plugin(DestructiblePlugin)
            .add_plugin(SpikeTrapPlugin)
            .add_plugin(WaterTilePlugin);
    }
}
