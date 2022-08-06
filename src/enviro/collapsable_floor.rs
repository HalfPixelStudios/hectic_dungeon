use bevy::prelude::*;
use bevy_bobs::component::health::Health;
use bevy_ecs_ldtk::{EntityInstance, GridCoords, TileMetadata};

use crate::{assets::SpriteSheet, grid::to_world_coords};

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

fn spawn_from_ldtk(
    mut cmd: Commands,
    query: Query<&EntityInstance, Added<EntityInstance>>,
    asset_sheet: Res<SpriteSheet>,
) {
    for entity_instance in query.iter().filter(|t| t.identifier == "CollapsableFloor") {
        info!("tile_meta {:?}", entity_instance);

        cmd.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 32,
                ..default()
            },
            texture_atlas: asset_sheet.clone(),
            transform: Transform {
                translation: to_world_coords(&entity_instance.grid).extend(1.),
                ..default()
            },
            ..default()
        })
        .insert(CollapsableFloor::new());
    }
}
