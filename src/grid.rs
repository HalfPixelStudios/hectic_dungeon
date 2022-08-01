use std::{fmt::Debug, ops::Deref};

use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

use crate::player::PlayerMovedEvent;

// TODO: make grid not have a constant size, we need to be able to switch out the map later

// TODO unify these constants with the map constants
const CELL_WIDTH: i32 = 8;
const CELL_HEIGHT: i32 = 8;
const MAP_WIDTH: i32 = 16;
const MAP_HEIGHT: i32 = 16;

#[derive(Copy, Clone)]
pub enum CellType {
    Empty = 0,
    Player = 1,
    Enemy = 2,
    Wall = 3,
}

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
    pub fn is_empty(&self, v: IVec2) -> bool {
        self.0[v.y as usize][v.x as usize] == CellType::Empty as i32
    }

    /// Checks if position is in bounds
    pub fn inbounds(&self, v: &IVec2) -> bool {
        0 <= v.x && v.x < MAP_WIDTH && 0 <= v.y && v.y < MAP_HEIGHT
    }

    /// Returns the first found cell of a given cell type
    pub fn find(&self, t: CellType) -> Option<IVec2> {
        for y in 0..MAP_HEIGHT as usize {
            for x in 0..MAP_WIDTH as usize {
                if self[y][x] == t as i32 {
                    return Some(IVec2::new(x as i32, y as i32));
                }
            }
        }
        return None;
    }

    /* probably not needed
    /// Move the current contents of a cell to a new cell
    ///
    /// Leaves the old cell empty. Furthermore, moving a cell that is currently empty has no
    /// effect.
    pub fn move_cell(&mut self, cur_pos: IVec2, dest_pos: IVec2) {
        let cur_value = self[cur_pos.y as usize][cur_pos.x as usize];
        if cur_value == 0 {
            return;
        }
        self[dest_pos.y as usize][dest_pos.x as usize] = cur_value;
    }
    */
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
fn update_grid(mut grid: ResMut<Grid>, mut query: Query<&GridPosition>) {
    for y in 0..MAP_HEIGHT as usize {
        for x in 0..MAP_WIDTH as usize {
            grid[y][x] = 0;
        }
    }
    for (grid_pos) in query.iter_mut() {
        if grid.inbounds(grid_pos) {
            grid[grid_pos.y as usize][grid_pos.x as usize] = grid_pos.value as i32;
        }
    }
}

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(gizmo)
            .add_system(update_grid)
            .add_system(debug)
            .insert_resource(Grid([[0; MAP_WIDTH as usize]; MAP_HEIGHT as usize]));
    }
}

fn debug(grid: Res<Grid>, mut events: EventReader<PlayerMovedEvent>) {
    for _ in events.iter() {
        info!("{:?}", grid);
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
