use std::collections::HashSet;

use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;
use priority_queue::PriorityQueue;

use crate::{
    animation::Animation,
    assets::{BeingPrefab, PrefabData, SpriteSheets},
    grid::{CellType, Grid, GridPosition},
    movement::Movement,
    player::PlayerMovedEvent,
};

#[derive(Component)]
pub struct Enemy;
pub struct SpawnEnemyEvent {
    pub spawn_pos: Vec2,
}
pub struct EnemyUpdateEvent;
fn spawn(
    mut cmd: Commands,
    mut events: EventReader<SpawnEnemyEvent>,
    asset_sheet: Res<SpriteSheets>,
    prefab_data: Res<PrefabData>,
    beings: Res<Assets<BeingPrefab>>,
) {
    for SpawnEnemyEvent { spawn_pos } in events.iter() {
        // let enemy = beings.get(prefab_data.get("archer").unwrap()).unwrap();
        cmd.spawn()
            .insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 0,
                    color: Color::ORANGE,
                    ..default()
                },
                texture_atlas: asset_sheet.get("orc").unwrap().clone(),
                transform: Transform {
                    translation: spawn_pos.extend(0.),
                    ..default()
                },
                ..default()
            })
            // .insert(Animation::new(&enemy.anim))
            .insert(GridPosition::new(spawn_pos, CellType::Enemy))
            .insert(Movement::new())
            .insert(Enemy);
    }
}
//TODO player cant move if go neg or enemy on top
//Maybe both are knocked back when two beings try to move into the same cell
fn ai(
    mut query: Query<(&Transform, &mut GridPosition, &mut Movement), With<Enemy>>,
    mut events: EventReader<PlayerMovedEvent>,
    grid: Res<Grid>,
) {
    let player_pos = grid.find(CellType::Player);

    for (transform, mut grid_pos, mut mv) in query.iter_mut() {
        if let Some(p) = player_pos {
            let diff = p - grid_pos.pos();
            if diff.x.abs() > diff.y.abs() {
                mv.next_move = diff.x.signum() * IVec2::X;
            } else {
                mv.next_move = diff.y.signum() * IVec2::Y;
            }
        } else {
            mv.next_move = IVec2::ZERO;
        }
    }
}

fn a_star(start: &IVec2, dest: &IVec2, grid: &Res<Grid>) {
    let mut search: PriorityQueue<IVec2, i32> = PriorityQueue::new();
    search.push_decrease(*start, 0);

    while search.len() > 0 {
        let (cur_pos, cur_cost) = search.pop().unwrap();

        // done
        if cur_pos == *dest {
            return;
        }

        // insert potential search cells
        for next_pos in tiles_around(&cur_pos, grid).iter() {
            search.push_decrease(*next_pos, heuristic(next_pos, dest));
        }
    }
}

fn tiles_around(pos: &IVec2, grid: &Res<Grid>) -> Vec<IVec2> {
    [
        IVec2::new(1, 0),
        IVec2::new(-1, 0),
        IVec2::new(0, 1),
        IVec2::new(0, -1),
    ]
    .into_iter()
    .map(|d| *pos + d)
    .filter(|pos| grid.inbounds(pos))
    .collect()
}

fn heuristic(cur: &IVec2, dest: &IVec2) -> i32 {
    // manhatten distance
    (cur.x - dest.x).abs() + (cur.y - dest.y).abs()
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEnemyEvent>()
            .add_event::<EnemyUpdateEvent>()
            .add_system(spawn)
            .add_system_set(
                ConditionSet::new()
                    .run_on_event::<EnemyUpdateEvent>()
                    .with_system(ai)
                    .into(),
            );
    }
}
