use bevy::prelude::*;
use bevy_ecs_ldtk::EntityInstance;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::{prelude::*, ui::attack_animation::SpawnAttackAnimEvent};

#[derive(Component)]
pub struct SpikeTrap {
    /// How often the spike trap triggers
    cycle: u32,
    /// Counter to keep track when the trap should fire
    counter: u32,
}

impl SpikeTrap {
    pub fn new(cycle: u32, offset: u32) -> Self {
        SpikeTrap {
            cycle,
            counter: cycle + offset,
        }
    }

    fn should_trigger(&self) -> bool {
        self.counter == 0
    }

    fn tick(&mut self) {
        if self.counter > 0 {
            self.counter -= 1;
        }
    }

    fn reset(&mut self) {
        self.counter = self.cycle;
    }
}

pub struct SpikeTrapPlugin;

impl Plugin for SpikeTrapPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::EnemyInput, update)
            .add_system(spawn_from_ldtk);
    }
}

fn update(
    mut query: Query<(Entity, &mut SpikeTrap, &GridEntity)>,
    mut attack_writer: EventWriter<AttackEvent>,
    mut anim_writer: EventWriter<SpawnAttackAnimEvent>,
) {
    for (entity, mut spike_trap, grid_entity) in query.iter_mut() {
        spike_trap.tick();
        if spike_trap.should_trigger() {
            spike_trap.reset();

            // send attack
            attack_writer.send(AttackEvent {
                grid_positions: vec![grid_entity.pos],
                cell_type: CellType::Player(entity),
            });

            // send anim (temp)
            anim_writer.send(SpawnAttackAnimEvent {
                frames: SpriteFrames::EnemyAttack.frames(),
                animation_speed: 0.1,
                spawn_pos: grid_entity.pos,
            });
        }
    }
}

fn spawn_from_ldtk(
    mut cmd: Commands,
    query: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
    asset_sheet: Res<SpriteSheet>,
) {
    for (_entity, entity_instance) in query.iter().filter(|(_, t)| t.identifier == "SpikeTrap") {
        let grid_coords = ldtk_to_bevy(&entity_instance.grid);

        cmd.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: *SpriteFrames::SpikeTrap.frames().get(0).unwrap(),
                ..default()
            },
            texture_atlas: asset_sheet.clone(),
            transform: Transform {
                translation: to_world_coords(&grid_coords).extend(BEING_LAYER),
                ..default()
            },
            ..default()
        })
        .insert(SpikeTrap::new(3, 0))
        .insert(GridEntity {
            pos: grid_coords,
            value: CellType::None,
        });
    }
}
