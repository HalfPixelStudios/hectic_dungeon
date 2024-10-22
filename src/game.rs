use bevy::{prelude::*, time::*};
use iyes_loopless::{
    prelude::*,
    state::{CurrentState, NextState},
};

use crate::prelude::*;

const PLAYER_INPUT_TIME_LIMIT: f32 = 30.;
const PLAYER_ANIM_TIME_LIMIT: f32 = 0.1;
const ENEMY_INPUT_TIME_LIMIT: f32 = 0.3;
const ENEMY_ANIM_TIME_LIMIT: f32 = 0.1;
const WORLD_UPDATE_TIME_LIMIT: f32 = 0.5;

const START_STATE: GameState = GameState::PlayerInput;

/// Four phases of the game loop
///
/// - `PlayerInput` is awaiting player input on their turn (either attack or move)
/// - `PlayerAnimation` is a short period allocated to playing animations like moving and attacking
/// - `EnemyInput` is for enemies to compute or carry out their next move and attacks
/// - `EnemyAnimation` similar to `PlayerAnimation` phase
/// - `WorldUpdate` is for updates that are not related to either the player or enemy
///
/// Can hook into each of these phases by adding your system to on_enter or on_update
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    PlayerInput,
    PlayerAnimation,
    EnemyInput,
    EnemyAnimation,
    WorldUpdate,
}

/// Resource that is used to pause and unpause the game
pub struct PauseGame(pub bool);

fn is_paused(paused: Res<PauseGame>) -> bool {
    paused.0
}

impl GameState {
    pub fn time_limit(&self) -> f32 {
        match self {
            GameState::PlayerInput => PLAYER_INPUT_TIME_LIMIT,
            GameState::PlayerAnimation => PLAYER_ANIM_TIME_LIMIT,
            GameState::EnemyInput => ENEMY_INPUT_TIME_LIMIT,
            GameState::EnemyAnimation => ENEMY_ANIM_TIME_LIMIT,
            GameState::WorldUpdate => WORLD_UPDATE_TIME_LIMIT,
        }
    }

    pub fn next_state(&self) -> GameState {
        match self {
            GameState::PlayerInput => GameState::PlayerAnimation,
            GameState::PlayerAnimation => GameState::EnemyInput,
            GameState::EnemyInput => GameState::EnemyAnimation,
            GameState::EnemyAnimation => GameState::WorldUpdate,
            GameState::WorldUpdate => GameState::PlayerInput,
        }
    }
}

#[derive(Deref, DerefMut, Default)]
struct StateTimer(pub Stopwatch);

/// Advance the game state based on defined time limits
fn game_loop(
    mut cmd: Commands,
    time: Res<Time>,
    mut state_timer: ResMut<StateTimer>,
    state: Res<CurrentState<GameState>>,
) {
    let time_limit = state.0.time_limit();

    if state_timer.elapsed_secs() > time_limit {
        cmd.insert_resource(NextState(state.0.next_state()));
        state_timer.reset();
    }

    state_timer.tick(time.delta());
}

/// End player input stage early if player sends input
fn end_player_input(
    mut cmd: Commands,
    mut state_timer: ResMut<StateTimer>,
    state: Res<CurrentState<GameState>>,
) {
    cmd.insert_resource(NextState(state.0.next_state()));
    state_timer.reset();
}

// TODO need to reset game state back to start state
fn reset(mut cmd: Commands, mut state_timer: ResMut<StateTimer>) {
    state_timer.reset();
    cmd.insert_resource(NextState(START_STATE));
}

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_loopless_state(START_STATE)
            .insert_resource(StateTimer::default())
            .insert_resource(PauseGame(false))
            .add_enter_system(ScreenState::Ingame, reset)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(ScreenState::Ingame)
                    .with_system(game_loop.run_if_not(is_paused))
                    .with_system(
                        end_player_input
                            .run_in_state(GameState::PlayerInput)
                            .run_on_event::<PlayerMovedEvent>(),
                    )
                    .into(),
            );
    }
}
