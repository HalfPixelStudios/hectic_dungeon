use std::{fmt::Debug, ops::Deref};

use anyhow::{anyhow, Result};
use bevy::prelude::*;
use bevy_bobs::prefab::PrefabId;
use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum GridError {
    #[error("tried to access position outside of grid {0}")]
    OutOfBounds(IVec2),
}

/// Collection of grid positions that can be queried and manipulated
///
/// The grid is a read-only structure. It is not a source of truth. GridPositions are the actual
/// source of truth. The grid is just a visual representation of where all the GridPosition objects
/// are relative to each other.
pub struct Grid<T: PartialEq> {
    width: i32,
    height: i32,
    grid: Vec<Vec<T>>,
}

impl<T: PartialEq> Grid<T> {
    pub fn new(width: i32, height: i32) -> Self {
        // TODO this is sorta stupid lol (maybe just force T to derive Clone)
        let mut grid_vec = (0..width * height).into_iter().map(|_| vec![]).collect();

        Grid {
            width,
            height,
            grid: grid_vec,
        }
    }

    pub fn bounds_check(&self, pos: &IVec2) -> bool {
        0 <= pos.x && pos.x < self.width && 0 <= pos.y && pos.y < self.height
    }

    pub fn pos_to_index(&self, pos: &IVec2) -> Result<usize> {
        if self.bounds_check(pos) {
            Ok((pos.y * self.width + pos.x) as usize)
        } else {
            Err(anyhow!(GridError::OutOfBounds(pos.to_owned())))
        }
    }

    pub fn get_cell(&self, pos: &IVec2) -> Result<&Vec<T>> {
        let ind = self.pos_to_index(pos)?;
        // shouldn't panic (since already bounds checked)?
        Ok(self.grid.get(ind).unwrap())
    }

    pub fn get_cell_mut(&mut self, pos: &IVec2) -> Result<&mut Vec<T>> {
        let ind = self.pos_to_index(pos)?;
        Ok(self.grid.get_mut(ind).unwrap())
    }

    pub fn insert_at(&mut self, pos: &IVec2, val: T) -> Result<()> {
        self.get_cell_mut(pos)?.push(val);
        Ok(())
    }

    pub fn contains_at(&self, pos: &IVec2, val: T) -> Result<bool> {
        Ok(self.get_cell(pos)?.contains(&val))
    }

    pub fn empty_at(&self, pos: &IVec2) -> Result<bool> {
        Ok(self.get_cell(pos)?.is_empty())
    }

    pub fn clear(&mut self) {
        for cell in self.grid.iter_mut() {
            cell.clear();
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}

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
fn sync_grid_positions(mut query: Query<(&mut Transform, &GridEntity)>, grid: Res<Grid<CellType>>) {
    for (mut transform, grid_position) in query.iter_mut() {
        let z = transform.translation.z;
        transform.translation = grid_position.as_vec2().extend(z) * (TILE_SIZE as f32);
    }
}

/// Grabs all grid positions and updates the grid
// TODO maybe use a Changed<> query to not have to keep wiping the map
fn update_grid(
    mut grid: ResMut<Grid<CellType>>,
    collision_map: Res<CollisionMap>,
    query: Query<&GridEntity>,
) {
    grid.clear();

    // TOOD: not very efficient to reload collisions every time, consider making a 'background'
    // grid that gets loaded
    for col in collision_map.iter() {
        grid.insert_at(col, CellType::Wall);
    }
    for grid_pos in query.iter() {
        grid.insert_at(grid_pos, grid_pos.value.clone());
    }
}

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_grid)
            // .add_system(sync_grid_positions)
            .insert_resource(Grid::<CellType>::new(MAP_WIDTH as i32, MAP_HEIGHT as i32));
    }
}

// TODO i dont like these funcionts
pub fn to_world_coords(p: &IVec2) -> Vec2 {
    Vec2::new((p.x * TILE_SIZE) as f32, (p.y * TILE_SIZE) as f32)
}

pub fn snap_to_grid(p: &Vec2) -> IVec2 {
    Vec2::new(p.x / TILE_SIZE as f32, p.y / TILE_SIZE as f32).as_ivec2()
}
