//! Controller systems related to user actions
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;
use pino_utils::ok_or_return;

use super::SelectedPlayer;
use crate::{game::PauseGame, level::Level, screens::state::ScreenState};

/// Tag component for user input controller
#[derive(Component)]
pub struct UserController;

/// Actions the user can take
#[derive(Actionlike, Clone)]
pub enum UserAction {
    PrevTroop,
    NextTroop,
    PauseGame,
}

/// Resource for selecting the current controlled troop
#[derive(Default)]
pub struct TroopSelector {
    index: usize,
}

pub struct UserActionPlugin;

impl Plugin for UserActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<UserAction>::default())
            .insert_resource(TroopSelector::default());

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ScreenState::Ingame)
                .with_system(troop_selector)
                .with_system(game_pauser)
                .into(),
        )
        .add_startup_system(spawn_user_controller);
    }
}

fn spawn_user_controller(mut cmd: Commands) {
    let input_map = InputMap::new([
        (KeyCode::Left, UserAction::PrevTroop),
        (KeyCode::Right, UserAction::NextTroop),
        (KeyCode::Escape, UserAction::PauseGame),
    ]);
    cmd.spawn_bundle(InputManagerBundle::<UserAction> {
        action_state: ActionState::default(),
        input_map,
    });
}

fn troop_selector(
    mut cmd: Commands,
    query: Query<&ActionState<UserAction>>,
    mut selector: ResMut<TroopSelector>,
    level_state: Res<Level>,
) {
    let action_state = ok_or_return!(query.get_single());

    let offset: i32 = if action_state.just_pressed(UserAction::PrevTroop) {
        -1
    } else if action_state.just_pressed(UserAction::NextTroop) {
        1
    } else {
        return;
    };

    let players = level_state.players();
    if players.is_empty() {
        return;
    }

    let old_player = players.iter().nth(selector.index).unwrap();
    selector.index = (selector.index as i32 + offset).rem_euclid(players.len() as i32) as usize;
    let new_player = players.iter().nth(selector.index).unwrap();

    // update the SelectedPlayer marker component
    cmd.entity(*old_player).remove::<SelectedPlayer>();
    cmd.entity(*new_player).insert(SelectedPlayer);
}

fn game_pauser(
    mut cmd: Commands,
    query: Query<&ActionState<UserAction>>,
    mut selector: ResMut<TroopSelector>,
    paused: Res<PauseGame>,
) {
    let action_state = ok_or_return!(query.get_single());

    if action_state.just_pressed(UserAction::PauseGame) {
        cmd.insert_resource(PauseGame(!paused.0));
    }
}
