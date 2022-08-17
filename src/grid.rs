use std::{fmt::Debug, ops::Deref};

use bevy::prelude::*;
use bevy_bobs::prefab::PrefabId;

use crate::{
    constants::{MAP_HEIGHT, MAP_WIDTH, TILE_SIZE},
    map::CollisionMap,
};

#[derive(Clone, PartialEq)]
pub enum CellType {
    Player(Entity),
    Enemy(Entity),
    Wall,
    CollapsableFloor(Entity),
    DroppedItem(Entity),
}

pub type Grid = bevy_bobs::grid::Grid<CellType>;

#[derive(Component)]
pub struct GridEntity {
    pub pos: IVec2,
    pub value: CellType,
}

impl GridEntity {
    pub fn new(pos: IVec2, value: CellType) -> GridEntity {
        GridEntity { pos, value }
    }
}

impl Deref for GridEntity {
    type Target = IVec2;

    fn deref(&self) -> &Self::Target {
        &self.pos
    }
}

// TODO maybe don't use this (cant really lerp position anymore)
fn sync_grid_positions(mut query: Query<(&mut Transform, &GridEntity)>, grid: Res<Grid>) {
    for (mut transform, grid_position) in &mut query {
        let z = transform.translation.z;
        transform.translation = grid_position.as_vec2().extend(z) * (TILE_SIZE as f32);
    }
}

/// Grabs all grid positions and updates the grid
// TODO maybe use a Changed<> query to not have to keep wiping the map
fn update_grid(
    mut grid: ResMut<Grid>,
    collision_map: Res<CollisionMap>,
    query: Query<&GridEntity>,
) {
    grid.clear();

    // TOOD: not very efficient to reload collisions every time, consider making a 'background'
    // grid that gets loaded
    for col in collision_map.iter() {
        grid.insert_at(col, CellType::Wall);
    }
    for grid_pos in &query {
        grid.insert_at(grid_pos, grid_pos.value.clone());
    }
}

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_grid)
            // .add_system(sync_grid_positions)
            .insert_resource(Grid::new(MAP_WIDTH as i32, MAP_HEIGHT as i32));
    }
}

// TODO i dont like these funcionts
pub fn to_world_coords(p: &IVec2) -> Vec2 {
    Vec2::new((p.x * TILE_SIZE) as f32, (p.y * TILE_SIZE) as f32)
}

pub fn snap_to_grid(p: &Vec2) -> IVec2 {
    Vec2::new(p.x / TILE_SIZE as f32, p.y / TILE_SIZE as f32).as_ivec2()
}
