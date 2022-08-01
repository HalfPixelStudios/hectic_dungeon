use bevy::prelude::*;

const CELLWIDTH: i32 = 16;
const CELLHEIGHT: i32 = 16;
const MAPWIDTH: i32 = 100;
const MAPHEIGHT: i32 = 100;

#[derive(Copy, Clone)]
pub enum CellType {
    Empty = 0,
    Player = 1,
    Enemy = 2,
    Wall = 3,
}

#[derive(DerefMut, Deref)]
pub struct Grid([[i32; MAPWIDTH as usize]; MAPHEIGHT as usize]);
impl Grid {
    pub fn is_open(&self, v: IVec2) -> bool {
        self.0[v.y as usize][v.x as usize] == 0
    }
    pub fn inbounds(&self, v: IVec2) -> bool {
        0 <= v.x && v.x < MAPWIDTH && 0 <= v.y && v.y < MAPHEIGHT
    }
    pub fn find(&self, t: CellType) -> Option<IVec2> {
        for y in 0..MAPHEIGHT as usize {
            for x in 0..MAPWIDTH as usize {
                if self[y][x] == t as i32 {
                    return Some(IVec2::new(x as i32, y as i32));
                }
            }
        }
        return None;
    }
}

#[derive(Component, DerefMut, Deref)]
pub struct GridPosition(pub IVec2);

impl GridPosition {
    pub fn new(v: &Vec2) -> GridPosition {
        info!("snap{}", snap_to_grid(v));
        GridPosition(snap_to_grid(v))
    }
}

pub fn to_world_coords(p: &IVec2) -> Vec2 {
    Vec2::new((p.x * CELLWIDTH) as f32, (p.y * CELLWIDTH) as f32)
}
pub fn snap_to_grid(p: &Vec2) -> IVec2 {
    info!("{:?}", p);
    Vec2::new(p.x / CELLWIDTH as f32, p.y / CELLWIDTH as f32).as_ivec2()
}

pub fn generate_grid(mut grid: ResMut<Grid>, mut query: Query<&GridPosition>) {
    for y in 0..MAPHEIGHT as usize {
        for x in 0..MAPWIDTH as usize {
            grid[y][x] = 0;
        }
    }
    for (grid_pos) in query.iter_mut() {
        if grid.inbounds(grid_pos.0) {
            grid[grid_pos.y as usize][grid_pos.x as usize] = 1;
        }
    }
}

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        //TODO generate_grid doesnt need to be run every fram?
        app.add_system(generate_grid)
            .insert_resource(Grid([[0; MAPWIDTH as usize]; MAPHEIGHT as usize]));
    }
}
