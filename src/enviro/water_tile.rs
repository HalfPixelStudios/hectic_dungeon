use bevy::prelude::*;
use bevy_ecs_ldtk::EntityInstance;
use iyes_loopless::prelude::*;
use pino_utils::ok_or_continue;

use crate::{
    constants::BEING_LAYER,
    game::GameState,
    grid::{to_world_coords, CellType, Grid, GridEntity},
    map::ldtk_to_bevy,
    spritesheet::{SpriteFrames, SpriteIndex, SpriteSheet},
    utils::Dir,
};

#[derive(Component)]
pub struct WaterTile {
    dir: Dir,
}

pub struct WaterTilePlugin;

impl Plugin for WaterTilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_from_ldtk)
            .add_enter_system(GameState::WorldUpdate, update);
    }
}

fn update(
    water_query: Query<(&WaterTile, &GridEntity), With<WaterTile>>,
    mut entity_query: Query<&mut GridEntity, Without<WaterTile>>,
    grid: Res<Grid>,
) {
    for (WaterTile { dir }, water_grid_entity) in &water_query {
        // find all entities that are on the water tile
        let grid_cells = ok_or_continue!(grid.get_cell(&water_grid_entity.pos));
        for grid_cell in grid_cells.iter() {
            // TODO for now only move player troops
            if let CellType::Player(entity) = grid_cell {
                let mut grid_entity = ok_or_continue!(entity_query.get_mut(*entity));

                // TODO check bounds
                grid_entity.pos += IVec2::from(*dir);
            }
        }
    }
}

fn spawn_from_ldtk(
    mut cmd: Commands,
    query: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
    asset_sheet: Res<SpriteSheet>,
) {
    for (entity, entity_instance) in query.iter().filter(|(_, t)| t.identifier == "WaterTile") {
        let grid_coords = ldtk_to_bevy(&entity_instance.grid);

        cmd.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: SpriteIndex::WaterTile as usize,
                ..default()
            },
            texture_atlas: asset_sheet.clone(),
            transform: Transform {
                translation: to_world_coords(&grid_coords).extend(BEING_LAYER),
                ..default()
            },
            ..default()
        })
        .insert(WaterTile { dir: Dir::North })
        .insert(GridEntity {
            pos: grid_coords,
            value: CellType::None,
        });
    }
}
