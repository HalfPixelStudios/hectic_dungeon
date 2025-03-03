use bevy::prelude::*;
use bevy_bobs::component::health::Health;
use bevy_ecs_ldtk::EntityInstance;
use iyes_loopless::prelude::*;

use crate::prelude::*;

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
            .add_exit_system(GameState::PlayerInput, detect_step_on)
            .add_system(despawn)
            .add_exit_system(ScreenState::Ingame, cleanup::<CollapsableFloor>);
    }
}

fn update(query: Query<&CollapsableFloor, Changed<CollapsableFloor>>) {
    for floor in &query {
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

        let grid_coords = ldtk_to_bevy(&entity_instance.grid);
        cmd.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 64,
                ..default()
            },
            texture_atlas: asset_sheet.clone(),
            transform: Transform {
                translation: to_world_coords(&grid_coords).extend(BEING_LAYER),
                ..default()
            },
            ..default()
        })
        .insert(CollapsableFloor::new())
        .insert(GridEntity {
            pos: grid_coords,
            value: CellType::CollapsableFloor(entity),
        });
    }
}

/// Detect whenever a player steps on a collapsable floor
fn detect_step_on(grid: Res<Grid>, mut query: Query<(&mut CollapsableFloor, &GridEntity)>) {
    for (mut floor, grid_entity) in &mut query {
        for cell_entity in grid.get_cell(&grid_entity.pos).unwrap().iter() {
            if let CellType::Player(_) = cell_entity {
                floor.health.take(1);
            }
        }
    }
}

fn despawn(
    mut cmd: Commands,
    query: Query<(Entity, &CollapsableFloor, &GridEntity)>,
    mut collision_map: ResMut<CollisionMap>,
) {
    for (entity, floor, grid_entity) in &query {
        if floor.health.is_zero() {
            cmd.entity(entity).despawn();
            collision_map.push(grid_entity.pos);
        }
    }
}
