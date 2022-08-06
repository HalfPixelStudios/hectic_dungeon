use bevy::prelude::*;
use bevy_bobs::component::health::Health;
use bevy_ecs_ldtk::{EntityInstance, TileMetadata};

const FLOOR_HEALTH: u32 = 2;

#[derive(Component)]
pub struct CollapsableFloor {
    health: Health,
}

impl CollapsableFloor {
    pub fn new() -> Self {
        CollapsableFloor {
            health: Health::new(FLOOR_HEALTH),
        }
    }
}

pub struct CollapsableFloorPlugin;

impl Plugin for CollapsableFloorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update).add_system(spawn_from_ldtk);
    }
}

fn update(query: Query<&CollapsableFloor, Changed<CollapsableFloor>>) {
    for floor in query.iter() {
        if floor.health.is_zero() {}
    }
}

fn spawn_from_ldtk(query: Query<&TileMetadata, Added<TileMetadata>>) {
    for tile_meta in query.iter() {
        info!("tile_meta {:?}", tile_meta);
    }
}
