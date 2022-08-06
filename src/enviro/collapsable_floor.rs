use bevy::prelude::*;
use bevy_bobs::component::health::Health;
use bevy_ecs_ldtk::{EntityInstance, GridCoords, TileMetadata};

use crate::{
    assets::SpriteSheet,
    grid::{to_world_coords, CellType, Grid, GridEntity},
    map::{ldtk_to_bevy, CollisionMap},
    player::PlayerMovedEvent,
};

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
        app.add_system(update)
            .add_system(spawn_from_ldtk)
            .add_system(detect_step_on)
            .add_system(despawn);
    }
}

fn update(query: Query<&CollapsableFloor, Changed<CollapsableFloor>>) {
    for floor in query.iter() {
        if floor.health.is_zero() {}
    }
}

fn spawn_from_ldtk(
    mut cmd: Commands,
    query: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
    asset_sheet: Res<SpriteSheet>,
) {
    for (entity, entity_instance) in query
        .iter()
        .filter(|(_, t)| t.identifier == "CollapsableFloor")
    {
        info!("tile_meta {:?}", entity_instance);

        cmd.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 32,
                ..default()
            },
            texture_atlas: asset_sheet.clone(),
            transform: Transform {
                translation: to_world_coords(&ldtk_to_bevy(&entity_instance.grid)).extend(1.),
                ..default()
            },
            ..default()
        })
        .insert(CollapsableFloor::new())
        .insert(GridEntity {
            pos: entity_instance.grid,
            value: CellType::CollapsableFloor(entity),
        });
    }
}

/// Detect whenever a player steps on a collapsable floor
// TODO better turn system (don't depend on PlayerMovedEvent)
fn detect_step_on(
    grid: Res<Grid<CellType>>,
    mut query: Query<(&mut CollapsableFloor, &GridEntity)>,
    mut events: EventReader<PlayerMovedEvent>,
) {
    for event in events.iter() {
        for (mut floor, grid_entity) in query.iter_mut() {
            for cell_entity in grid.get_cell(&grid_entity.pos).unwrap().iter() {
                if let CellType::Player(_) = cell_entity {
                    floor.health.take(1);
                }
            }
        }
    }
}

fn despawn(
    mut cmd: Commands,
    query: Query<(Entity, &CollapsableFloor, &GridEntity)>,
    mut collision_map: ResMut<CollisionMap>,
) {
    for (entity, floor, grid_entity) in query.iter() {
        if floor.health.is_zero() {
            cmd.entity(entity).despawn();
            collision_map.push(grid_entity.pos);
        }
    }
}
