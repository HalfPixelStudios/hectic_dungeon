use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{grid::GridEntity, player::Player};

/// Track distance to the player
///
/// Decided to attack depending on how close the player is
#[derive(Component)]
pub struct AttackRangeScorer {
    pub range: f32,
}

fn attack_range_scorer(
    player_query: Query<&GridEntity, With<Player>>,
    mut score_query: Query<(&Actor, &mut Score, &AttackRangeScorer), Without<Player>>,
    query: Query<&GridEntity, Without<Player>>,
) {
    if let Ok(player_grid_entity) = player_query.get_single() {
        for (Actor(actor), mut score, scorer) in score_query.iter_mut() {
            if let Ok(grid_entity) = query.get(*actor) {
                let distance = player_grid_entity
                    .pos
                    .as_vec2()
                    .distance(grid_entity.pos.as_vec2());
                score.set(if distance < scorer.range { 1. } else { 0. });
            }
        }
    }
}

#[derive(Component, Clone)]
pub struct AttackAction;

fn attack_action(mut action_query: Query<(&Actor, &mut ActionState), With<AttackAction>>) {
    for (Actor(actor), mut state) in action_query.iter_mut() {
        match *state {
            ActionState::Requested => {
                *state = ActionState::Executing;
            },
            ActionState::Executing => {
                *state = ActionState::Success;
            },
            ActionState::Cancelled => {
                *state = ActionState::Failure;
            },
            _ => {},
        }
    }
}

pub struct SimpleAIPlugin;

impl Plugin for SimpleAIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(BigBrainStage::Scorers, attack_range_scorer)
            .add_system_to_stage(BigBrainStage::Actions, attack_action);
    }
}
