use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    sync,
};

use bevy::prelude::*;
use bevy_bobs::{
    component::health::Health,
    health_bar::{spawn_health_bar, HealthBar},
};
use bevy_ecs_ldtk::{prelude::FieldValue, EntityInstance};
use iyes_loopless::prelude::*;
use priority_queue::PriorityQueue;

use crate::{
    animation::Animation,
    assets::{BeingPrefab, PrefabData, SpriteSheet},
    attack::{AttackEvent, AttackPattern},
    game::GameState,
    grid::{to_world_coords, CellType, Grid, GridEntity},
    map::ldtk_to_bevy,
    movement::Movement,
    player::Player,
    ui::attack_indicator::AttackIndicator,
    utils::Dir,
    weapon::CurrentWeapon,
};

#[derive(Component)]
pub struct Enemy;

pub struct SpawnEnemyEvent {
    pub spawn_pos: IVec2,
}

pub struct DamageEnemyEvent {
    pub entity: Entity,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEnemyEvent>()
            .add_event::<DamageEnemyEvent>()
            .add_system(spawn)
            .add_system(take_damage)
            .add_system(sync_health_bars)
            .add_system(spawn_from_ldtk)
            .add_enter_system(GameState::EnemyInput, ai);
    }
}

fn spawn(
    mut cmd: Commands,
    mut events: EventReader<SpawnEnemyEvent>,
    asset_sheet: Res<SpriteSheet>,
    prefab_data: Res<PrefabData>,
    beings: Res<Assets<BeingPrefab>>,
) {
    for SpawnEnemyEvent { spawn_pos } in events.iter() {
        // let enemy = beings.get(prefab_data.get("archer").unwrap()).unwrap();
        let id = cmd.spawn().id();

        cmd.entity(id)
            .insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 82,
                    color: Color::ORANGE,
                    ..default()
                },
                texture_atlas: asset_sheet.clone(),
                transform: Transform {
                    translation: to_world_coords(spawn_pos).extend(1.),
                    ..default()
                },
                ..default()
            })
            // .insert(Animation::new(&enemy.anim))
            .insert(GridEntity::new(*spawn_pos, CellType::Enemy(id)))
            .insert(Movement::new())
            .insert(AttackIndicator::default())
            .insert(CurrentWeapon("dagger".into()))
            .insert(Enemy)
            .insert(Health::new(3));

        let hp_bar = spawn_health_bar(
            &mut cmd,
            bevy_bobs::health_bar::HealthBarPrefab {
                dimension: Vec2::new(8., 2.),
                bg_color: Color::BLACK,
                fg_color: Color::GREEN,
                translation: Vec3::ZERO,
            },
        );
        cmd.entity(id).add_child(hp_bar);
    }
}
//TODO player cant move if go neg or enemy on top
//Maybe both are knocked back when two beings try to move into the same cell
fn ai(
    mut player_query: Query<(&GridEntity), (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<
        (
            Entity,
            &Transform,
            &mut GridEntity,
            &mut Movement,
            &mut AttackIndicator,
        ),
        (With<Enemy>, Without<Player>),
    >,
    mut writer: EventWriter<AttackEvent>,
    grid: Res<Grid<CellType>>,
) {
    let player_grid_pos = player_query.single().pos;

    for (entity, transform, mut grid_entity, mut mv, mut attack_indicator) in enemy_query.iter_mut()
    {
        // run attack if queued in last turn
        if !attack_indicator.hidden {
            let grid_positions = attack_indicator
                .get_pattern()
                .iter()
                .map(|v| *v + grid_entity.pos)
                .collect();

            // TODO the entity in the CellType::Player is just a dummy value, this is pretty
            // disgusting
            writer.send(AttackEvent {
                grid_positions,
                cell_type: CellType::Player(entity),
            });
            attack_indicator.hidden = true;
        } else {
            // movement phase
            let cur_pos = grid_entity.pos;
            if let Some(path) = a_star(&cur_pos, &player_grid_pos, &grid) {
                let next_pos = path.get(0).unwrap_or(&cur_pos);
                mv.next_move = *next_pos - cur_pos;
            } else {
                info!("failed to calculate path");
            }

            // attempt attack
            // TODO hardcoded attack logic
            if player_grid_pos.as_vec2().distance(cur_pos.as_vec2()) < 3. {
                // determine direction to attack in
                let dir: Dir = (player_grid_pos - cur_pos).into();

                attack_indicator.dir = dir;
                attack_indicator.hidden = false;
            } else {
                attack_indicator.hidden = true;
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

    while !search.is_empty() {
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

fn take_damage(
    mut cmd: Commands,
    mut events: EventReader<DamageEnemyEvent>,
    mut query: Query<&mut Health, With<Enemy>>,
) {
    for DamageEnemyEvent { entity } in events.iter() {
        let mut health = query.get_mut(*entity).unwrap();

        health.take(1);
        if health.is_zero() {
            cmd.entity(*entity).despawn_recursive();
        }
    }
}

fn sync_health_bars(query: Query<(&Health, &Children)>, mut hp_bar_query: Query<&mut HealthBar>) {
    for (health, children) in query.iter() {
        for child in children.iter() {
            if let Ok(mut hp_bar) = hp_bar_query.get_mut(*child) {
                hp_bar.set_percent(health.percent());
            }
        }
    }
}

fn spawn_from_ldtk(
    query: Query<&EntityInstance, Added<EntityInstance>>,
    mut writer: EventWriter<SpawnEnemyEvent>,
) {
    for entity_instance in query.iter().filter(|e| e.identifier == "EnemySpawn") {
        // TODO handle not found
        if let Some(field) = entity_instance
            .field_instances
            .iter()
            .find(|field| field.identifier == "id")
        {
            if let FieldValue::String(Some(v)) = &field.value {
                writer.send(SpawnEnemyEvent {
                    spawn_pos: ldtk_to_bevy(&entity_instance.grid),
                });
            }
        }
    }
}
