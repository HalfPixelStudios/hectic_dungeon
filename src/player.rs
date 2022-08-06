use std::time::Duration;

use bevy::prelude::*;
use bevy_bobs::component::health::*;
use bevy_ecs_ldtk::EntityInstance;
use iyes_loopless::{prelude::*, state::NextState};
use leafwing_input_manager::prelude::*;

use crate::{
    animation::Animation,
    assets::{BeingPrefab, PrefabData, SpriteSheet},
    attack::{AttackEvent, AttackPattern},
    camera::CameraFollow,
    game::GameState,
    grid::{to_world_coords, CellType, Grid, GridEntity},
    map::ldtk_to_bevy,
    movement::Movement,
    ui::attack_indicator::AttackIndicator,
    utils::Dir,
    weapon::CurrentWeapon,
};

#[derive(Component)]
pub struct Player;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum PlayerState {
    Move,
    Attack,
}

//TODO add direction vectors to PlayerAction definition
#[derive(Actionlike, Clone)]
pub enum PlayerAction {
    Left,
    Right,
    Up,
    Down,
    Attack,
}
pub struct PlayerMovedEvent;

pub struct SpawnPlayerEvent {
    pub spawn_pos: IVec2,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_loopless_state(PlayerState::Move)
            .add_event::<SpawnPlayerEvent>()
            .add_event::<PlayerMovedEvent>()
            .add_system(
                move_controller
                    .run_in_state(GameState::PlayerInput)
                    .run_in_state(PlayerState::Move),
            )
            .add_system(
                attack_controller
                    .run_in_state(GameState::PlayerInput)
                    .run_in_state(PlayerState::Attack),
            )
            .add_system(spawn)
            .add_system(spawn_from_ldtk);
    }
}

fn spawn(
    mut cmd: Commands,
    mut events: EventReader<SpawnPlayerEvent>,
    asset_sheet: Res<SpriteSheet>,
    prefab_data: Res<PrefabData>,
    beings: Res<Assets<BeingPrefab>>,
) {
    for SpawnPlayerEvent { spawn_pos } in events.iter() {
        let input_map = InputMap::new([
            // (KeyCode::Left, PlayerAction::Left),
            (KeyCode::A, PlayerAction::Left),
            // (KeyCode::Right, PlayerAction::Right),
            (KeyCode::D, PlayerAction::Right),
            // (KeyCode::Up, PlayerAction::Up),
            (KeyCode::W, PlayerAction::Up),
            // (KeyCode::Down, PlayerAction::Down),
            (KeyCode::S, PlayerAction::Down),
            (KeyCode::Space, PlayerAction::Attack),
        ]);

        // let handle = prefab_data.get("player").unwrap();
        // let player = beings.get(handle).unwrap();

        let id = cmd.spawn().id();

        cmd.entity(id)
            .insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 41,
                    ..default()
                },
                texture_atlas: asset_sheet.clone(),
                transform: Transform {
                    translation: to_world_coords(spawn_pos).extend(1.),
                    ..default()
                },
                ..default()
            })
            .insert(Player)
            .insert(GridEntity::new(*spawn_pos, CellType::Player(id)))
            .insert(Health::new(10))
            .insert_bundle(InputManagerBundle::<PlayerAction> {
                action_state: ActionState::default(),
                input_map,
            })
            .insert(CameraFollow)
            .insert(CurrentWeapon("hammer".into()))
            .insert(AttackIndicator::default())
            .insert(Children::default())
            .insert(Movement::new());
    }
}

//TODO check collision with tiled map
// TODO i dont really like having to include AttackIndicator in query
fn move_controller(
    mut cmd: Commands,
    mut query: Query<
        (
            &mut GridEntity,
            &mut Movement,
            &mut AttackIndicator,
            &ActionState<PlayerAction>,
        ),
        With<Player>,
    >,
    mut player_moved: EventWriter<PlayerMovedEvent>,
    grid: Res<Grid<CellType>>,
) {
    if let Ok((mut grid_position, mut movement, mut attack_indicator, action_state)) =
        query.get_single_mut()
    {
        let mut dir = IVec2::ZERO;

        if action_state.just_pressed(PlayerAction::Left) {
            dir += IVec2::new(-1, 0);
        }
        if action_state.just_pressed(PlayerAction::Right) {
            dir += IVec2::new(1, 0);
        }
        if action_state.just_pressed(PlayerAction::Up) {
            dir += IVec2::new(0, 1);
        }
        if action_state.just_pressed(PlayerAction::Down) {
            dir += IVec2::new(0, -1);
        }
        if action_state.just_pressed(PlayerAction::Attack) {
            attack_indicator.hidden = false;
            cmd.insert_resource(NextState(PlayerState::Attack));
        }

        // TODO movement collision logic shouldn't be here?
        if movement.next_move == IVec2::ZERO {
            let next_pos = grid_position.pos + dir;
            if dir != IVec2::ZERO
                && grid.bounds_check(&next_pos)
                && !grid.contains_at(&next_pos, CellType::Wall).unwrap()
            {
                player_moved.send(PlayerMovedEvent);
                movement.next_move = dir;
            }
        }
    }
}

fn attack_controller(
    mut cmd: Commands,
    mut query: Query<
        (
            Entity,
            &mut AttackIndicator,
            &GridEntity,
            &ActionState<PlayerAction>,
        ),
        With<Player>,
    >,
    mut writer: EventWriter<AttackEvent>,
    mut player_moved: EventWriter<PlayerMovedEvent>,
) {
    if let Ok((entity, mut attack_indicator, grid_entity, action_state)) = query.get_single_mut() {
        if action_state.just_pressed(PlayerAction::Up) {
            attack_indicator.dir = Dir::North;
        }
        if action_state.just_pressed(PlayerAction::Left) {
            attack_indicator.dir = Dir::West;
        }
        if action_state.just_pressed(PlayerAction::Down) {
            attack_indicator.dir = Dir::South;
        }
        if action_state.just_pressed(PlayerAction::Right) {
            attack_indicator.dir = Dir::East;
        }
        if action_state.just_pressed(PlayerAction::Attack) {
            attack_indicator.hidden = true;
            cmd.insert_resource(NextState(PlayerState::Move));

            // deal damage
            let grid_positions = attack_indicator
                .get_pattern()
                .iter()
                .map(|v| *v + grid_entity.pos)
                .collect();
            // TODO the entity in the CellType::ENemy is just a dummy value, this is pretty
            // disgusting
            writer.send(AttackEvent {
                grid_positions,
                cell_type: CellType::Enemy(entity),
            });

            player_moved.send(PlayerMovedEvent);
        }
    }
}

fn spawn_from_ldtk(
    query: Query<&EntityInstance, Added<EntityInstance>>,
    mut writer: EventWriter<SpawnPlayerEvent>,
) {
    for entity_instance in query.iter().filter(|e| e.identifier == "PlayerSpawn") {
        writer.send(SpawnPlayerEvent {
            spawn_pos: ldtk_to_bevy(&entity_instance.grid),
        });
    }
}
