use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;
use priority_queue::PriorityQueue;

use crate::{
    animation::Animation,
    assets::{BeingPrefab, PrefabData, SpriteSheets},
    grid::{CellType, Grid, GridEntity},
    movement::Movement,
    player::{Player, PlayerMovedEvent},
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
                    translation: spawn_pos.extend(1.),
                    ..default()
                },
                ..default()
            })
            // .insert(Animation::new(&enemy.anim))
            .insert(GridEntity::new(spawn_pos.as_ivec2(), CellType::Enemy))
            .insert(Movement::new())
            .insert(Enemy);
    }
}
//TODO player cant move if go neg or enemy on top
//Maybe both are knocked back when two beings try to move into the same cell
fn ai(
    mut player_query: Query<(&GridEntity), (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<
        (&Transform, &mut GridEntity, &mut Movement),
        (With<Enemy>, Without<Player>),
    >,
    mut events: EventReader<PlayerMovedEvent>,
    grid: Res<Grid<CellType>>,
) {
    for _ in events.iter() {
        let player_grid_pos = player_query.single().pos;

        for (transform, mut grid_pos, mut mv) in enemy_query.iter_mut() {
            let cur_pos = grid_pos.pos;
            if let Some(path) = a_star(&cur_pos, &player_grid_pos, &grid) {
                let next_pos = path.get(0).unwrap_or(&cur_pos);
                mv.next_move = *next_pos - cur_pos;
            } else {
                info!("failed to calculate path");
            }
        }
    }
}

pub fn a_star(start: &IVec2, dest: &IVec2, grid: &Res<Grid<CellType>>) -> Option<Vec<IVec2>> {
    // trivial case
    if start == dest {
        return Some(Vec::new());
    }

    // info!("starting a star search {:?} {:?}", start, dest);
    let mut search: PriorityQueue<IVec2, Reverse<i32>> = PriorityQueue::new();
    search.push_decrease(*start, Reverse(0));

    let mut costs: HashMap<IVec2, i32> = HashMap::new();
    let mut came_from: HashMap<IVec2, IVec2> = HashMap::new();

    costs.insert(*start, 0);

    while search.len() > 0 {
        let (cur_pos, cur_cost) = search.pop().unwrap();
        // info!("searching {:?}", cur_pos);

        // done
        if cur_pos == *dest {
            let mut pos = dest;
            let mut path: Vec<IVec2> = Vec::new();
            loop {
                let prev_pos = came_from.get(pos).unwrap();
                if prev_pos == start {
                    break;
                }
                path.push(*prev_pos);
                pos = prev_pos;
            }
            path.reverse();
            // info!("path {:?}", path);
            return Some(path);
        }

        // insert potential search cells
        for next_pos in tiles_around(&cur_pos, grid).iter() {
            let new_cost = costs.get(&cur_pos).unwrap() + 1;
            if costs.get(next_pos).unwrap_or(&std::i32::MAX) > &new_cost {
                search.push_decrease(*next_pos, Reverse(new_cost + heuristic(next_pos, dest)));
                costs.insert(*next_pos, new_cost);
                came_from.insert(*next_pos, cur_pos);
            }
        }
    }

    None
}

fn tiles_around(pos: &IVec2, grid: &Res<Grid<CellType>>) -> Vec<IVec2> {
    use rand::{seq::SliceRandom, thread_rng};

    let mut dirs = [
        IVec2::new(1, 0),
        IVec2::new(-1, 0),
        IVec2::new(0, 1),
        IVec2::new(0, -1),
    ];

    dirs.shuffle(&mut thread_rng());

    dirs.into_iter()
        .map(|d| *pos + d)
        .filter(|pos| grid.bounds_check(pos) && !grid.contains_at(pos, CellType::Wall).unwrap())
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
