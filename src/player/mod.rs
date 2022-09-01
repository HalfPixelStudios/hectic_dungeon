pub mod inventory;
pub mod prefab;

use std::time::Duration;

use bevy::prelude::*;
use bevy_bobs::{
    component::health::*,
    prefab::{PrefabId, PrefabLib},
};
use bevy_ecs_ldtk::{prelude::FieldValue, EntityInstance};
use iyes_loopless::{prelude::*, state::NextState};
use leafwing_input_manager::prelude::*;

use self::{
    inventory::Inventory,
    prefab::{PlayerPrefab, PrefabPlugin},
};
use crate::{
    animation::Animation,
    assets::{BeingPrefab, PrefabData, SpriteSheet},
    attack::{AttackEvent, AttackPattern},
    camera::CameraFollow,
    constants::BEING_LAYER,
    enviro::dropped_item::DroppedItem,
    game::GameState,
    grid::{to_world_coords, CellType, Grid, GridEntity},
    map::ldtk_to_bevy,
    movement::Movement,
    room::Room,
    screens::state::ScreenState,
    spritesheet_constants::{SpriteFrames, SpriteIndex},
    ui::{
        attack_animation::SpawnAttackAnimEvent, attack_indicator::AttackIndicator,
        floating_text::FloatingText, move_indicator::MoveIndicator,
    },
    utils::{cardinal_dirs, ok_or_return, some_or_continue, Dir},
    weapon::CurrentWeapon,
};

#[derive(Component)]
pub struct Player;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum PlayerState {
    None,
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
    Cancel,
    Interact,
}
pub struct PlayerMovedEvent;

pub struct SpawnPlayerEvent {
    pub spawn_pos: IVec2,
    pub prefab_id: PrefabId,
}

pub struct DamagePlayerEvent {
    pub entity: Entity,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_plugin(PrefabPlugin)
            .add_loopless_state(PlayerState::Move)
            .add_event::<SpawnPlayerEvent>()
            .add_event::<PlayerMovedEvent>()
            .add_event::<DamagePlayerEvent>()
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(ScreenState::Ingame)
                    .with_system(controller.run_in_state(GameState::PlayerInput))
                    .with_system(
                        move_controller
                            .run_in_state(GameState::PlayerInput)
                            .run_in_state(PlayerState::Move),
                    )
                    .with_system(
                        attack_controller
                            .run_in_state(GameState::PlayerInput)
                            .run_in_state(PlayerState::Attack),
                    )
                    .with_system(spawn)
                    .with_system(take_damage)
                    .with_system(update_move_indicator.run_in_state(GameState::PlayerInput))
                    .with_system(spawn_from_ldtk)
                    .into(),
            )
            .add_enter_system(PlayerState::Attack, transition_to_attack)
            .add_enter_system(PlayerState::Move, transition_to_move)
            .add_enter_system(PlayerState::None, transition_to_none)
            .add_enter_system(GameState::PlayerInput, on_turn_start)
            .add_exit_system(GameState::PlayerInput, reset_on_turn_end);
    }
}

fn spawn(
    mut cmd: Commands,
    mut events: EventReader<SpawnPlayerEvent>,
    asset_sheet: Res<SpriteSheet>,
    prefab_lib: Res<PrefabLib<PlayerPrefab>>,
    mut room_state: ResMut<Room>,
) {
    for SpawnPlayerEvent {
        spawn_pos,
        prefab_id,
    } in events.iter()
    {
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
            (KeyCode::Escape, PlayerAction::Cancel),
            (KeyCode::E, PlayerAction::Interact),
        ]);

        let prefab = some_or_continue!(prefab_lib.get(prefab_id));

        let id = cmd.spawn().id();

        cmd.entity(id)
            .insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: prefab.sprite_index as usize,
                    ..default()
                },
                texture_atlas: asset_sheet.clone(),
                transform: Transform {
                    translation: to_world_coords(spawn_pos).extend(BEING_LAYER),
                    ..default()
                },
                ..default()
            })
            .insert(Player)
            .insert(GridEntity::new(*spawn_pos, CellType::Player(id)))
            .insert(Health::new(prefab.health))
            .insert_bundle(InputManagerBundle::<PlayerAction> {
                action_state: ActionState::default(),
                input_map,
            })
            .insert(CameraFollow)
            .insert(CurrentWeapon(prefab.default_primary.to_owned().unwrap()))
            .insert(Movement::new())
            .insert(Inventory {
                weapon_primary: prefab.default_primary.to_owned(),
                weapon_secondary: prefab.default_secondary.to_owned(),
                armor: prefab.default_armor.to_owned(),
                ability: prefab.default_ability.to_owned(),
            });

        // ui related
        cmd.entity(id)
            .insert(AttackIndicator::default())
            .insert(MoveIndicator::default());
        // .insert(FloatingText {
        //     text: "hello world".into(),
        //     offset: Vec2::new(0., 8.),
        //     ..default()
        // });
        room_state.register_player();
    }
}

fn controller(
    mut cmd: Commands,
    mut query: Query<(&GridEntity, &ActionState<PlayerAction>), With<Player>>,
    item_query: Query<&DroppedItem, Without<Player>>,
    grid: Res<Grid>,
) {
    let (grid_entity, action_state) = ok_or_return!(query.get_single_mut());

    if action_state.just_pressed(PlayerAction::Interact) {
        for cell_entity in grid.get_cell(&grid_entity.pos).unwrap().iter() {
            if let CellType::DroppedItem(entity) = cell_entity {
                let dropped_item = item_query.get(*entity).unwrap();

                info!("picked up {}", dropped_item.prefab_id);

                cmd.entity(*entity).despawn();
            }
        }
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
    grid: Res<Grid>,
) {
    let (mut grid_position, mut movement, mut attack_indicator, action_state) =
        ok_or_return!(query.get_single_mut());
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

fn update_move_indicator(
    mut query: Query<(&GridEntity, &mut MoveIndicator), With<Player>>,
    grid: Res<Grid>,
) {
    let (grid_entity, mut move_indicator) = ok_or_return!(query.get_single_mut());
    // TODO duplicated valid move checking logic from move_controller function
    move_indicator.dirs.clear();
    for dir in cardinal_dirs().iter() {
        let next_pos = IVec2::from(*dir) + grid_entity.pos;
        if grid.bounds_check(&next_pos) && !grid.contains_at(&next_pos, CellType::Wall).unwrap() {
            move_indicator.dirs.push(*dir);
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
    mut anim_writer: EventWriter<SpawnAttackAnimEvent>,
    mut player_moved: EventWriter<PlayerMovedEvent>,
) {
    let (entity, mut attack_indicator, grid_entity, action_state) =
        ok_or_return!(query.get_single_mut());

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
    if action_state.just_pressed(PlayerAction::Cancel) {
        cmd.insert_resource(NextState(PlayerState::Move));
    }
    if action_state.just_pressed(PlayerAction::Attack) {
        // deal damage
        let grid_positions = attack_indicator
            .get_pattern()
            .iter()
            .map(|v| *v + grid_entity.pos)
            .collect::<Vec<_>>();
        // TODO the entity in the CellType::ENemy is just a dummy value, this is pretty
        // disgusting

        // spawn attack animation
        for pos in grid_positions.iter() {
            anim_writer.send(SpawnAttackAnimEvent {
                frames: SpriteFrames::PlayerAttack.frames(),
                animation_speed: 0.1,
                spawn_pos: *pos,
            });
        }

        writer.send(AttackEvent {
            grid_positions,
            cell_type: CellType::Enemy(entity),
        });

        player_moved.send(PlayerMovedEvent);
    }
}

/// If player turn expires or ends, disable their AttackIndicator and reset them to move state
fn reset_on_turn_end(mut cmd: Commands) {
    cmd.insert_resource(NextState(PlayerState::None));
}
/// Default to move state on turn start
fn on_turn_start(mut cmd: Commands) {
    cmd.insert_resource(NextState(PlayerState::Move));
}

fn transition_to_move(mut query: Query<(&mut AttackIndicator, &mut MoveIndicator), With<Player>>) {
    let (mut attack_indicator, mut move_indicator) = ok_or_return!(query.get_single_mut());
    attack_indicator.hidden = true;
    move_indicator.hidden = false;
}
fn transition_to_attack(
    mut query: Query<(&mut AttackIndicator, &mut MoveIndicator), With<Player>>,
) {
    let (mut attack_indicator, mut move_indicator) = ok_or_return!(query.get_single_mut());
    attack_indicator.hidden = false;
    move_indicator.hidden = true;
}
fn transition_to_none(mut query: Query<(&mut AttackIndicator, &mut MoveIndicator), With<Player>>) {
    let (mut attack_indicator, mut move_indicator) = ok_or_return!(query.get_single_mut());
    attack_indicator.hidden = true;
    move_indicator.hidden = true;
}

fn spawn_from_ldtk(
    query: Query<&EntityInstance, Added<EntityInstance>>,
    mut writer: EventWriter<SpawnPlayerEvent>,
) {
    for entity_instance in query.iter().filter(|e| e.identifier == "PlayerSpawn") {
        if let Some(field) = entity_instance
            .field_instances
            .iter()
            .find(|field| field.identifier == "id")
        {
            if let FieldValue::String(Some(v)) = &field.value {
                writer.send(SpawnPlayerEvent {
                    spawn_pos: ldtk_to_bevy(&entity_instance.grid),
                    prefab_id: v.to_owned(),
                });
            }
        }
    }
}

fn take_damage(
    mut cmd: Commands,
    mut events: EventReader<DamagePlayerEvent>,
    mut query: Query<(&mut Health, &GridEntity)>,
) {
    for DamagePlayerEvent { entity } in events.iter() {
        let (mut health, grid_entity) = query.get_mut(*entity).unwrap();

        health.take(1);
        if health.is_zero() {}
    }
}
