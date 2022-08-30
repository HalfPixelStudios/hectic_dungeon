use bevy::prelude::*;
use serde::Deserialize;

use crate::{
    enemy::DamageEnemyEvent,
    grid::{CellType, Grid},
    player::DamagePlayerEvent,
    utils::{ok_or_continue, variant_eq, Dir},
};

#[derive(Deserialize, Clone, Copy)]
pub enum AttackPattern {
    None,
    StraightOne,
    StraightTwo,
    StraightSix,
    Hammer,
    TwinBlade,
}

impl AttackPattern {
    // default north
    pub fn to_offsets(&self) -> Vec<IVec2> {
        match &self {
            AttackPattern::None => vec![],
            AttackPattern::StraightOne => vec![IVec2::new(0, 1)],
            AttackPattern::StraightTwo => vec![IVec2::new(0, 1), IVec2::new(0, 2)],
            AttackPattern::StraightSix => (1..=6).map(|i| IVec2::new(0, i)).collect(),
            AttackPattern::Hammer => vec![
                IVec2::new(-1, 1),
                IVec2::new(0, 1),
                IVec2::new(1, 1),
                IVec2::new(-1, 2),
                IVec2::new(0, 2),
                IVec2::new(1, 2),
            ],
            AttackPattern::TwinBlade => vec![
                IVec2::new(-1, 1),
                IVec2::new(-2, 2),
                IVec2::new(1, 1),
                IVec2::new(2, 2),
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
    /// target cell type to hit
    pub cell_type: CellType,
}

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackEvent>().add_system(process_attack);
    }
}

fn process_attack(
    mut events: EventReader<AttackEvent>,
    mut enemy_writer: EventWriter<DamageEnemyEvent>,
    mut player_writer: EventWriter<DamagePlayerEvent>,
    grid: Res<Grid>,
) {
    for AttackEvent {
        grid_positions,
        cell_type,
    } in events.iter()
    {
        for grid_position in grid_positions.iter() {
            let cell = ok_or_continue!(grid.get_cell(grid_position));
            for cell_entity in cell.iter() {
                if !variant_eq::<CellType>(cell_entity, cell_type) {
                    continue;
                }

                match cell_entity {
                    CellType::Enemy(entity) => {
                        enemy_writer.send(DamageEnemyEvent { entity: *entity });
                    },
                    CellType::Player(entity) => {
                        info!("player hit!");
                        player_writer.send(DamagePlayerEvent { entity: *entity });
                    },
                    _ => {},
                }
            }
        }
    }
}
