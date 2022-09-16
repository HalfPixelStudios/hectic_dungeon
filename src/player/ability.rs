use bevy::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;
use pino_utils::ok_or_return;

use super::{controller::TroopAction, PlayerMovedEvent, SelectedPlayer};
use crate::{
    attack::{AttackEvent, AttackPattern},
    game::GameState,
    grid::{CellType, GridEntity},
    screens::state::ScreenState,
    spritesheet::SpriteFrames,
    ui::attack_animation::SpawnAttackAnimEvent,
};

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ScreenState::Ingame)
                .with_system(ability_controller.run_in_state(GameState::PlayerInput))
                .into(),
        );
    }
}

pub fn ability_controller(
    mut cmd: Commands,
    mut query: Query<(Entity, &GridEntity, &ActionState<TroopAction>), With<SelectedPlayer>>,
    mut writer: EventWriter<AttackEvent>,
    mut anim_writer: EventWriter<SpawnAttackAnimEvent>,
    mut player_moved: EventWriter<PlayerMovedEvent>,
) {
    let (entity, grid_entity, action_state) = ok_or_return!(query.get_single_mut());

    if action_state.just_pressed(TroopAction::Ability) {
        let grid_positions = AttackPattern::Around
            .to_offsets()
            .iter()
            .map(|v| *v + grid_entity.pos)
            .collect::<Vec<_>>();

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
