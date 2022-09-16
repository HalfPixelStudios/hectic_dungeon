use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::FieldValue, EntityInstance};
use iyes_loopless::prelude::*;

use crate::{
    prelude::*,
    ui::{attack_indicator::AttackIndicator, projectile::SpawnProjectileEvent},
};

// how many turns between each attack
const ATTACK_SPEED: u32 = 3;
const CELL_TYPE: u32 = 8;

#[derive(Component)]
pub struct ArrowTrap {
    turn_count: u32,
    dir: Dir,
}

impl ArrowTrap {
    pub fn new(dir: Dir) -> Self {
        ArrowTrap { turn_count: 0, dir }
    }
}

pub struct ArrowTrapPlugin;

impl Plugin for ArrowTrapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_from_ldtk)
            .add_enter_system(GameState::EnemyInput, ai)
            .add_exit_system(ScreenState::Ingame, cleanup::<ArrowTrap>);
    }
}

fn ai(
    mut query: Query<(Entity, &GridEntity, &mut ArrowTrap, &mut AttackIndicator)>,
    mut writer: EventWriter<AttackEvent>,
    mut projectile_writer: EventWriter<SpawnProjectileEvent>,
) {
    for (entity, grid_entity, mut arrow_trap, mut attack_indicator) in &mut query {
        if !attack_indicator.hidden {
            let grid_positions = attack_indicator
                .get_pattern()
                .iter()
                .map(|v| *v + grid_entity.pos)
                .collect();

            // TODO dummy entity (super stupid)
            writer.send(AttackEvent {
                grid_positions,
                cell_type: CellType::Player(entity),
            });

            projectile_writer.send(SpawnProjectileEvent {
                sprite_index: 39,
                spawn_pos: grid_entity.pos,
                dir: arrow_trap.dir,
                distance: 6 * CELL_TYPE,
                speed: 200.,
            });
        }

        attack_indicator.hidden = true;
        arrow_trap.turn_count += 1;
        if arrow_trap.turn_count >= ATTACK_SPEED {
            arrow_trap.turn_count = 0;
            attack_indicator.hidden = false;
        }
    }
}

fn spawn_from_ldtk(
    mut cmd: Commands,
    query: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
    asset_sheet: Res<SpriteSheet>,
) {
    for (entity, entity_instance) in query.iter().filter(|(_, t)| t.identifier == "ArrowTrap") {
        // TODO this code is sorta cringe
        let dir: Dir = entity_instance
            .field_instances
            .iter()
            .find(|field| field.identifier.to_lowercase() == "dir")
            .map_or(Dir::East, |field_inst| {
                if let FieldValue::String(Some(dir)) = &field_inst.value {
                    Dir::from(dir.to_owned())
                } else {
                    Dir::East
                }
            });

        let sprite_index = match dir {
            Dir::North => 53,
            Dir::East => 37,
            _ => unreachable!(),
        };

        let grid_coords = ldtk_to_bevy(&entity_instance.grid);

        cmd.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: sprite_index,
                ..default()
            },
            texture_atlas: asset_sheet.clone(),
            transform: Transform {
                translation: to_world_coords(&grid_coords).extend(BEING_LAYER),
                ..default()
            },
            ..default()
        })
        .insert(ArrowTrap::new(dir))
        .insert(AttackIndicator { dir, ..default() })
        .insert(CurrentWeapon("arrow_trap".into()))
        .insert(GridEntity {
            pos: grid_coords,
            value: CellType::Wall,
        });
    }
}
