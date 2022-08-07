use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::FieldValue, EntityInstance};
use iyes_loopless::prelude::*;

use crate::{
    assets::SpriteSheet,
    attack::AttackEvent,
    game::GameState,
    grid::{to_world_coords, CellType, GridEntity},
    map::ldtk_to_bevy,
    ui::attack_indicator::AttackIndicator,
    utils::Dir,
    weapon::CurrentWeapon,
};

// how many turns between each attack
const ATTACK_SPEED: u32 = 3;

#[derive(Component)]
pub struct ArrowTrap {
    turn_count: u32,
}

impl ArrowTrap {
    pub fn new() -> Self {
        ArrowTrap { turn_count: 0 }
    }
}

pub struct ArrowTrapPlugin;

impl Plugin for ArrowTrapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_from_ldtk)
            .add_enter_system(GameState::EnemyInput, ai);
    }
}

fn ai(
    mut query: Query<(Entity, &GridEntity, &mut ArrowTrap, &mut AttackIndicator)>,
    mut writer: EventWriter<AttackEvent>,
) {
    for (entity, grid_entity, mut arrow_trap, mut attack_indicator) in query.iter_mut() {
        if attack_indicator.hidden == false {
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

        info!("arrow trap facing {:?}", dir);

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
                translation: to_world_coords(&grid_coords).extend(1.),
                ..default()
            },
            ..default()
        })
        .insert(ArrowTrap::new())
        .insert(AttackIndicator { dir, ..default() })
        .insert(CurrentWeapon("arrow_trap".into()))
        .insert(GridEntity {
            pos: grid_coords,
            value: CellType::Wall,
        });
    }
}
