use bevy::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;
use pino_utils::ok_or_return;

use super::{controller::TroopAction, prefab::Class, Player, PlayerMovedEvent, SelectedPlayer};
use crate::{prelude::*, ui::attack_animation::SpawnAttackAnimEvent};

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ScreenState::Ingame)
                .with_system(warrior_ability_controller.run_in_state(GameState::PlayerInput))
                .into(),
        );
    }
}

fn warrior_ability_controller(
    mut cmd: Commands,
    mut query: Query<
        (Entity, &Player, &GridEntity, &ActionState<TroopAction>),
        With<SelectedPlayer>,
    >,
    mut writer: EventWriter<AttackEvent>,
    mut anim_writer: EventWriter<SpawnAttackAnimEvent>,
    mut player_moved: EventWriter<PlayerMovedEvent>,
) {
    let (entity, player, grid_entity, action_state) = ok_or_return!(query.get_single_mut());

    if player.0 != Class::Warrior {
        return;
    }

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
