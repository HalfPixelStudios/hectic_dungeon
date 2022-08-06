use bevy::{core::Stopwatch, prelude::*};
use iyes_loopless::{
    prelude::*,
    state::{CurrentState, NextState},
};

use crate::{enemy::EnemyUpdateEvent, player::PlayerMovedEvent};

const PLAYER_INPUT_TIME_LIMIT: f32 = 3.;
const PLAYER_ANIM_TIME_LIMIT: f32 = 0.5;
const ENEMY_INPUT_TIME_LIMIT: f32 = 1.;
const ENEMY_ANIM_TIME_LIMIT: f32 = 0.5;

/// Four phases of the game loop
///
/// - `PlayerInput` is awaiting player input on their turn (either attack or move)
/// - `PlayerAnimation` is a short period allocated to playing animations like moving and attacking
/// - `EnemyInput` is for enemies to compute or carry out their next move and attacks
/// - `EnemyAnimation` similar to `PlayerAnimation` phase
///
/// Can hook into each of these phases by adding your system to on_enter or on_update
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    PlayerInput,
    PlayerAnimation,
    EnemyInput,
    EnemyAnimation,
}

impl GameState {
    pub fn time_limit(&self) -> f32 {
        match self {
            GameState::PlayerInput => PLAYER_INPUT_TIME_LIMIT,
            GameState::PlayerAnimation => PLAYER_ANIM_TIME_LIMIT,
            GameState::EnemyInput => ENEMY_INPUT_TIME_LIMIT,
            GameState::EnemyAnimation => ENEMY_ANIM_TIME_LIMIT,
        }
    }

    pub fn next_state(&self) -> GameState {
        match self {
            GameState::PlayerInput => GameState::PlayerAnimation,
            GameState::PlayerAnimation => GameState::EnemyInput,
            GameState::EnemyInput => GameState::EnemyAnimation,
            GameState::EnemyAnimation => GameState::PlayerInput,
        }
    }
}

#[derive(Deref, DerefMut, Default)]
pub struct StateTimer(pub Stopwatch);

/// Advance the game state based on defined time limits
pub fn game_loop(
    mut cmd: Commands,
    time: Res<Time>,
    mut state_timer: ResMut<StateTimer>,
    state: Res<CurrentState<GameState>>,
) {
    let time_limit = state.0.time_limit();

    if state_timer.elapsed_secs() > time_limit {
        cmd.insert_resource(NextState(state.0.next_state()));
        info!("advancing to next state {:?}", state.0.next_state());
        state_timer.reset();
    }

    state_timer.tick(time.delta());
}

/// End player input stage early if player sends input
pub fn end_player_input(
    mut cmd: Commands,
    mut state_timer: ResMut<StateTimer>,
    state: Res<CurrentState<GameState>>,
) {
    cmd.insert_resource(NextState(state.0.next_state()));
    info!("skipping player input");
    state_timer.reset();
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_loopless_state(GameState::PlayerInput)
            .insert_resource(StateTimer::default())
            .add_system(game_loop)
            .add_system(
                end_player_input
                    .run_in_state(GameState::PlayerInput)
                    .run_on_event::<PlayerMovedEvent>(),
            );
    }
}
