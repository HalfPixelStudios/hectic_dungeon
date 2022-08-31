use bevy::prelude::*;
use big_brain::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    attack::AttackEvent,
    enemy::pathfinding::a_star,
    game::GameState,
    grid::{CellType, Grid, GridEntity},
    movement::Movement,
    player::Player,
    screens::state::ScreenState,
    spritesheet_constants::SpriteFrames,
    ui::{attack_animation::SpawnAttackAnimEvent, attack_indicator::AttackIndicator},
    utils::{ok_or_continue, ok_or_return, Dir},
};

/// Track distance to the player
///
/// Decided to attack depending on how close the player is
#[derive(Component, Clone, Debug)]
pub struct AttackRangeScorer {
    pub range: f32,
}

fn attack_range_scorer(
    player_query: Query<&GridEntity, With<Player>>,
    mut score_query: Query<(&Actor, &mut Score, &AttackRangeScorer), Without<Player>>,
    query: Query<&GridEntity, Without<Player>>,
) {
    let player_grid_entity = ok_or_return!(player_query.get_single());

    for (Actor(actor), mut score, scorer) in &mut score_query {
        let grid_entity = ok_or_continue!(query.get(*actor));

        let distance = player_grid_entity
            .pos
            .as_vec2()
            .distance(grid_entity.pos.as_vec2());
        score.set(if distance < scorer.range { 1. } else { 0. });
    }
}

// TODO
/// Only attack if attack pattern overlaps with player
#[derive(Component, Clone, Debug)]
pub struct AttackPatternScorer {}

#[derive(Component, Clone, Debug)]
pub struct AttackAction;

// TODO this action is doing too many things
fn attack_action(
    mut player_query: Query<(&GridEntity), With<Player>>,
    mut action_query: Query<(&Actor, &mut ActionState), With<AttackAction>>,
    mut query: Query<(&GridEntity, &mut AttackIndicator), Without<Player>>,
    mut anim_writer: EventWriter<SpawnAttackAnimEvent>,
    mut attack_writer: EventWriter<AttackEvent>,
) {
    let player_grid_entity = ok_or_return!(player_query.get_single());

    for (Actor(actor), mut state) in &mut action_query {
        let (grid_entity, mut attack_indicator) = ok_or_continue!(query.get_mut(*actor));
        match *state {
            ActionState::Requested => {
                // info!("attack requested");

                // enable attack animation
                let dir: Dir = (player_grid_entity.pos - grid_entity.pos).into();

                attack_indicator.dir = dir;
                attack_indicator.hidden = false;

                *state = ActionState::Executing;
            },
            ActionState::Executing => {
                // info!("attack executing");

                // perform attack
                let grid_positions = attack_indicator
                    .get_pattern()
                    .iter()
                    .map(|v| *v + grid_entity.pos)
                    .collect::<Vec<_>>();

                for pos in grid_positions.iter() {
                    anim_writer.send(SpawnAttackAnimEvent {
                        // TODO need constants for this
                        frames: SpriteFrames::EnemyAttack.frames(),
                        animation_speed: 0.1,
                        spawn_pos: *pos,
                    });
                }

                // TODO the entity in the CellType::Player is just a dummy value, this is pretty
                // disgusting
                attack_writer.send(AttackEvent {
                    grid_positions,
                    cell_type: CellType::Player(*actor),
                });

                attack_indicator.hidden = true;
                *state = ActionState::Success;
            },
            ActionState::Cancelled => {
                *state = ActionState::Failure;
                info!("attack cancelled");
            },
            _ => {},
        }
    }
}

/// Pathfind to player
#[derive(Component, Clone, Debug)]
pub struct MoveAction;

fn move_action(
    grid: Res<Grid>,
    mut player_query: Query<(&GridEntity), With<Player>>,
    mut action_query: Query<(&Actor, &mut ActionState), With<MoveAction>>,
    mut query: Query<(&GridEntity, &mut Movement, &mut AttackIndicator), Without<Player>>,
) {
    let player_grid_entity = ok_or_return!(player_query.get_single());

    for (Actor(actor), mut state) in &mut action_query {
        let (grid_entity, mut movement, mut attack_indicator) =
            ok_or_continue!(query.get_mut(*actor));
        match *state {
            ActionState::Requested => {
                // movement phase
                let cur_pos = grid_entity.pos;
                if let Some(path) = a_star(&cur_pos, &player_grid_entity.pos, &grid) {
                    let next_pos = path.get(0).unwrap_or(&cur_pos);
                    movement.next_move = *next_pos - cur_pos;
                } else {
                    info!("failed to calculate path");
                    *state = ActionState::Failure;
                }
                *state = ActionState::Success;
            },
            ActionState::Cancelled => {
                *state = ActionState::Failure;
                info!("attack cancelled");
            },
            _ => {},
        }
    }
}

pub struct SimpleAIPlugin;

impl Plugin for SimpleAIPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system_to_stage(BigBrainStage::Scorers, attack_range_scorer)
        //     .add_system_to_stage(BigBrainStage::Actions, attack_action);
        app.add_enter_system(GameState::EnemyInput, attack_action)
            .add_enter_system(GameState::EnemyInput, move_action)
            .add_enter_system(GameState::EnemyInput, attack_range_scorer);
    }
}
