use bevy::prelude::*;

use crate::{
    enemy::DamageEnemyEvent,
    grid::{CellType, Grid},
    utils::Dir,
};

pub enum AttackPattern {
    None,
    StraightOne,
    StraightTwo,
    Hammer,
}

impl AttackPattern {
    // default north
    pub fn to_offsets(&self) -> Vec<IVec2> {
        match &self {
            AttackPattern::None => vec![],
            AttackPattern::StraightOne => vec![IVec2::new(0, 1)],
            AttackPattern::StraightTwo => vec![IVec2::new(0, 1), IVec2::new(0, 2)],
            AttackPattern::Hammer => vec![
                IVec2::new(-1, 1),
                IVec2::new(0, 1),
                IVec2::new(1, 1),
                IVec2::new(-1, 2),
                IVec2::new(0, 2),
                IVec2::new(1, 2),
            ],
        }
    }
}

pub fn rotate_offsets(vecs: Vec<IVec2>, dir: Dir) -> Vec<IVec2> {
    match dir {
        Dir::North | Dir::NorthEast => vecs,
        Dir::East | Dir::SouthEast => vecs.into_iter().map(|v| IVec2::new(v.y, -v.x)).collect(),
        Dir::South | Dir::SouthWest => vecs.into_iter().map(|v| IVec2::new(-v.x, -v.y)).collect(),
        Dir::West | Dir::NorthWest => vecs.into_iter().map(|v| IVec2::new(-v.y, v.x)).collect(),
        _ => vecs,
    }
}

pub struct AttackEvent {
    pub grid_positions: Vec<IVec2>,
}

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackEvent>().add_system(process_attack);
    }
}

fn process_attack(
    mut events: EventReader<AttackEvent>,
    mut writer: EventWriter<DamageEnemyEvent>,
    grid: Res<Grid<CellType>>,
) {
    for AttackEvent { grid_positions } in events.iter() {
        for grid_position in grid_positions.iter() {
            if let Ok(cell) = grid.get_cell(grid_position) {
                for cell_entity in cell.iter() {
                    match cell_entity {
                        CellType::Enemy(entity) => {
                            writer.send(DamageEnemyEvent { entity: *entity });
                        },
                        _ => {},
                    }
                }
            }
        }
    }
}
