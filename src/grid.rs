use std::{fmt::Debug, ops::Deref};

use anyhow::{anyhow, Result};
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use thiserror::Error;

use crate::{map::CollisionMap, player::PlayerMovedEvent};

// TODO: make grid not have a constant size, we need to be able to switch out the map later

// TODO unify these constants with the map constants
const CELL_WIDTH: i32 = 8;
const CELL_HEIGHT: i32 = 8;
const MAP_WIDTH: i32 = 16;
const MAP_HEIGHT: i32 = 16;

#[derive(Copy, Clone, PartialEq)]
pub enum CellType {
    Empty = 0,
    Player = 1,
    Enemy = 2,
    Wall = 3,
}

pub struct Grid {
    width: i32,
    height: i32,
    grid: Vec<Vec<CellType>>,
}

#[derive(Error, Debug)]
pub enum GridError {
    #[error("tried to access position outside of grid {0}")]
    OutOfBounds(IVec2),
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Self {
        let grid_vec = Vec::new();
        grid_vec.reserve((width * height) as usize);

        Grid {
            width,
            height,
            grid: grid_vec,
        }
    }

    fn bounds_check(&self, pos: &IVec2) -> bool {
        0 <= pos.x && pos.x < self.width && 0 <= pos.y && pos.y < self.height
    }

    fn pos_to_index(&self, pos: &IVec2) -> Result<usize> {
        if self.bounds_check(pos) {
            Ok((pos.y * self.width + pos.x) as usize)
        } else {
            Err(anyhow!(GridError::OutOfBounds(pos.to_owned())))
        }
    }

    fn get_cell(&self, pos: &IVec2) -> Result<&Vec<CellType>> {
        let ind = self.pos_to_index(pos)?;
        // shouldn't panic (since already bounds checked)?
        Ok(self.grid.get(ind).unwrap())
    }

    fn get_cell_mut(&mut self, pos: &IVec2) -> Result<&mut Vec<CellType>> {
        let ind = self.pos_to_index(pos)?;
        Ok(self.grid.get_mut(ind).unwrap())
    }

    pub fn insert_at(&mut self, pos: &IVec2, val: CellType) -> Result<()> {
        self.get_cell_mut(pos)?.push(val);
        Ok(())
    }

    pub fn contains_at(&self, pos: &IVec2, val: CellType) -> Result<bool> {
        Ok(self.get_cell(pos)?.contains(&val))
    }

    pub fn clear(&mut self) {
        for cell in self.grid.iter_mut() {
            cell.clear();
        }
    }
}

/*
/// Collection of grid positions that can be queried and manipulated
///
/// The grid is a read-only structure. It is not a source of truth. GridPositions are the actual
/// source of truth. The grid is just a visual representation of where all the GridPosition objects
/// are relative to each other.
// TODO should use cell type enum (or as generic) for value
#[derive(DerefMut, Deref)]
pub struct Grid([[i32; MAP_WIDTH as usize]; MAP_HEIGHT as usize]);

impl Grid {

    /// Checks if given cell value is empty
    pub fn is_empty(&self, v: &IVec2) -> bool {
        self.0[v.y as usize][v.x as usize] == CellType::Empty as i32
    }

    /// Checks if position is in bounds
    pub fn inbounds(&self, v: &IVec2) -> bool {
        0 <= v.x && v.x < MAP_WIDTH && 0 <= v.y && v.y < MAP_HEIGHT
    }

    pub fn at(&self, v: &IVec2) -> i32 {
        self[v.y as usize][v.x as usize]
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::from("\n");
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                out += &format!("{} ", self[(MAP_HEIGHT - 1 - y) as usize][x as usize]);
            }
            out += "\n";
        }
        write!(f, "{}", out)
    }
}
*/

#[derive(Component)]
pub struct GridPosition {
    pos: IVec2,
    pub value: CellType,
}

impl GridPosition {
    pub fn new(pos: &Vec2, value: CellType) -> GridPosition {
        GridPosition {
            pos: snap_to_grid(pos),
            value,
        }
    }

    /// Absolute movement
    pub fn move_to(&mut self, pos: &IVec2) {
        // TODO bounds checking
        self.pos = *pos;
    }

    /// Move relative to current position
    pub fn move_relative(&mut self, offset: &IVec2) {
        self.pos += *offset;
    }

    pub fn pos(&self) -> IVec2 {
        self.pos
    }
}

impl Deref for GridPosition {
    type Target = IVec2;

    fn deref(&self) -> &Self::Target {
        &self.pos
    }
}

pub fn to_world_coords(p: &IVec2) -> Vec2 {
    Vec2::new((p.x * CELL_WIDTH) as f32, (p.y * CELL_WIDTH) as f32)
}

pub fn snap_to_grid(p: &Vec2) -> IVec2 {
    Vec2::new(p.x / CELL_WIDTH as f32, p.y / CELL_WIDTH as f32).as_ivec2()
}

/// Grabs all grid positions and updates the grid
fn update_grid(
    mut grid: ResMut<Grid>,
    collision_map: Res<CollisionMap>,
    query: Query<&GridPosition>,
) {
    for y in 0..MAP_HEIGHT as usize {
        for x in 0..MAP_WIDTH as usize {
            grid[y][x] = 0;
        }
    }
    // TOOD: not very efficient to reload collisions every time, consider making a 'background'
    // grid that gets loaded
    for col in collision_map.iter() {
        if grid.inbounds(col) {
            grid[col.y as usize][col.x as usize] = CellType::Wall as i32;
        }
    }
    for grid_pos in query.iter() {
        if grid.inbounds(grid_pos) {
            grid[grid_pos.y as usize][grid_pos.x as usize] = grid_pos.value as i32;
        }
    }
}

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_grid)
            // .add_system(gizmo)
            .add_system(debug)
            .insert_resource(Grid([[0; MAP_WIDTH as usize]; MAP_HEIGHT as usize]));
    }
}

fn debug(grid: Res<Grid>, mut events: EventReader<PlayerMovedEvent>) {
    for _ in events.iter() {
        // info!("{:?}", grid);
    }
}

fn gizmo(mut lines: ResMut<DebugLines>) {
    for y in 0..MAP_HEIGHT {
        lines.line(
            Vec3::new(0., (y * CELL_HEIGHT) as f32, 0.),
            Vec3::new(
                ((MAP_WIDTH - 1) * CELL_WIDTH) as f32,
                (y * CELL_HEIGHT) as f32,
                0.,
            ),
            0.,
        );
    }
    for x in 0..MAP_WIDTH {
        lines.line(
            Vec3::new((x * CELL_WIDTH) as f32, 0., 0.),
            Vec3::new(
                (x * CELL_WIDTH) as f32,
                ((MAP_HEIGHT - 1) * CELL_HEIGHT) as f32,
                0.,
            ),
            0.,
        );
    }
}
