//! Handles input and attack/moving states
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;
use pino_utils::ok_or_return;

use super::{Player, PlayerMovedEvent, SelectedPlayer};
use crate::{
    enviro::dropped_item::DroppedItem,
    prelude::*,
    ui::{
        attack_animation::SpawnAttackAnimEvent, attack_indicator::AttackIndicator,
        move_indicator::MoveIndicator,
    },
};

/// Indicator for the current troop action
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum TroopState {
    None,
    Move,
    Attack,
}

/// Actions the troop can take
//TODO add direction vectors to PlayerAction definition
#[derive(Actionlike, Clone)]
pub enum TroopAction {
    Left,
    Right,
    Up,
    Down,
    Attack,
    ToggleState,
    Interact,
    Ability,
}

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<TroopAction>::default())
            .add_loopless_state(TroopState::Move);

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ScreenState::Ingame)
                .with_system(controller.run_in_state(GameState::PlayerInput))
                .with_system(
                    move_controller
                        .run_in_state(GameState::PlayerInput)
                        .run_in_state(TroopState::Move),
                )
                .with_system(
                    attack_controller
                        .run_in_state(GameState::PlayerInput)
                        .run_in_state(TroopState::Attack),
                )
                .with_system(update_move_indicator.run_in_state(GameState::PlayerInput))
                .with_system(ui_enabler)
                .into(),
        )
        .add_enter_system(GameState::PlayerInput, on_turn_start)
        .add_exit_system(GameState::PlayerInput, reset_on_turn_end);
    }
}

fn controller(
    mut cmd: Commands,
    mut query: Query<(&GridEntity, &ActionState<TroopAction>), With<SelectedPlayer>>,
    item_query: Query<&DroppedItem, Without<SelectedPlayer>>,
    grid: Res<Grid>,
) {
    let (grid_entity, action_state) = ok_or_return!(query.get_single_mut());

    if action_state.just_pressed(TroopAction::Interact) {
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
            &ActionState<TroopAction>,
        ),
        With<SelectedPlayer>,
    >,
    mut player_moved: EventWriter<PlayerMovedEvent>,
    grid: Res<Grid>,
) {
    let (grid_position, mut movement, _attack_indicator, action_state) =
        ok_or_return!(query.get_single_mut());
    let mut dir = IVec2::ZERO;

    if action_state.just_pressed(TroopAction::Left) {
        dir += IVec2::new(-1, 0);
    }
    if action_state.just_pressed(TroopAction::Right) {
        dir += IVec2::new(1, 0);
    }
    if action_state.just_pressed(TroopAction::Up) {
        dir += IVec2::new(0, 1);
    }
    if action_state.just_pressed(TroopAction::Down) {
        dir += IVec2::new(0, -1);
    }
    if action_state.just_pressed(TroopAction::ToggleState) {
        cmd.insert_resource(NextState(TroopState::Attack));
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
    for (grid_entity, mut move_indicator) in query.iter_mut() {
        // TODO duplicated valid move checking logic from move_controller function
        move_indicator.dirs.clear();
        for dir in cardinal_dirs().iter() {
            let next_pos = IVec2::from(*dir) + grid_entity.pos;
            if grid.bounds_check(&next_pos) && !grid.contains_at(&next_pos, CellType::Wall).unwrap()
            {
                move_indicator.dirs.push(*dir);
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
            &ActionState<TroopAction>,
        ),
        With<SelectedPlayer>,
    >,
    mut writer: EventWriter<AttackEvent>,
    mut anim_writer: EventWriter<SpawnAttackAnimEvent>,
    mut player_moved: EventWriter<PlayerMovedEvent>,
) {
    let (entity, mut attack_indicator, grid_entity, action_state) =
        ok_or_return!(query.get_single_mut());

    if action_state.just_pressed(TroopAction::Up) {
        attack_indicator.dir = Dir::North;
    }
    if action_state.just_pressed(TroopAction::Left) {
        attack_indicator.dir = Dir::West;
    }
    if action_state.just_pressed(TroopAction::Down) {
        attack_indicator.dir = Dir::South;
    }
    if action_state.just_pressed(TroopAction::Right) {
        attack_indicator.dir = Dir::East;
    }
    if action_state.just_pressed(TroopAction::ToggleState) {
        cmd.insert_resource(NextState(TroopState::Move));
    }
    if action_state.just_pressed(TroopAction::Attack) {
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
    cmd.insert_resource(NextState(TroopState::None));
}
/// Default to move state on turn start
fn on_turn_start(mut cmd: Commands) {
    cmd.insert_resource(NextState(TroopState::Move));
}

/// Decides which ingame ui to show or hide depending on state
fn ui_enabler(
    mut query: Query<
        (
            &mut AttackIndicator,
            &mut MoveIndicator,
            Option<&SelectedPlayer>,
        ),
        With<Player>,
    >,
    current_state: Res<CurrentState<TroopState>>,
) {
    for (mut attack_indicator, mut move_indicator, selected) in query.iter_mut() {
        if selected.is_none() {
            attack_indicator.hidden = true;
            move_indicator.hidden = true;
        } else {
            match current_state.0 {
                TroopState::None => {
                    attack_indicator.hidden = true;
                    move_indicator.hidden = true;
                },
                TroopState::Move => {
                    attack_indicator.hidden = true;
                    move_indicator.hidden = false;
                },
                TroopState::Attack => {
                    attack_indicator.hidden = false;
                    move_indicator.hidden = true;
                },
            }
        }
    }
}
