pub mod pathfinding;
pub mod prefab;

use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    sync,
};

use bevy::prelude::*;
use bevy_bobs::{
    component::health::Health,
    health_bar::{spawn_health_bar, HealthBar},
    prefab::{PrefabId, PrefabLib},
};
use bevy_ecs_ldtk::{prelude::FieldValue, EntityInstance};
use big_brain::{prelude::FirstToScore, thinker::Thinker, BigBrainPlugin};
use iyes_loopless::prelude::*;

use self::prefab::{EnemyPrefab, PrefabPlugin, AI};
use crate::{
    ai::simple_ai::{AttackAction, AttackRangeScorer, MoveAction},
    animation::Animation,
    assets::{BeingPrefab, PrefabData, SpriteSheet},
    attack::{AttackEvent, AttackPattern},
    constants::{BEING_LAYER, INGAME_UI_LAYER},
    enviro::dropped_item::SpawnDroppedItemEvent,
    game::GameState,
    grid::{to_world_coords, CellType, Grid, GridEntity},
    level::Level,
    map::ldtk_to_bevy,
    movement::Movement,
    player::Player,
    screens::state::ScreenState,
    spritesheet_constants::SpriteIndex,
    ui::{attack_animation::SpawnAttackAnimEvent, attack_indicator::AttackIndicator},
    utils::{some_or_continue, Dir},
    weapon::CurrentWeapon,
};

#[derive(Component)]
pub struct Enemy;

pub type DropTable = bevy_bobs::component::droptable::DropTable<String>;

pub struct SpawnEnemyEvent {
    pub prefab_id: PrefabId,
    pub spawn_pos: IVec2,
}

pub struct DamageEnemyEvent {
    pub entity: Entity,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PrefabPlugin)
            .add_event::<SpawnEnemyEvent>()
            .add_event::<DamageEnemyEvent>()
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(ScreenState::Ingame)
                    .with_system(spawn)
                    .with_system(take_damage)
                    .with_system(sync_health_bars)
                    .with_system(spawn_from_ldtk)
                    .into(),
            );
    }
}

fn spawn(
    mut cmd: Commands,
    mut events: EventReader<SpawnEnemyEvent>,
    asset_sheet: Res<SpriteSheet>,
    prefab_lib: Res<PrefabLib<EnemyPrefab>>,
    mut room_state: ResMut<Level>,
) {
    for SpawnEnemyEvent {
        spawn_pos,
        prefab_id,
    } in events.iter()
    {
        let prefab = some_or_continue!(prefab_lib.get(prefab_id));

        let id = cmd.spawn().id();

        cmd.entity(id)
            .insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: prefab.sprite_index as usize,
                    ..default()
                },
                texture_atlas: asset_sheet.clone(),
                transform: Transform {
                    translation: to_world_coords(spawn_pos).extend(BEING_LAYER),
                    ..default()
                },
                ..default()
            })
            // .insert(Animation::new(&enemy.anim))
            .insert(GridEntity::new(*spawn_pos, CellType::Enemy(id)))
            .insert(Movement::new())
            .insert(AttackIndicator::default())
            .insert(CurrentWeapon(prefab.weapon_id.to_owned()))
            .insert(Enemy)
            .insert(Health::new(prefab.health))
            .insert(DropTable::new(prefab.drops.clone()));

        match prefab.ai {
            AI::Simple { attack_range } => {
                cmd.entity(id).insert(
                    Thinker::build()
                        .picker(FirstToScore { threshold: 0.8 })
                        .when(
                            AttackRangeScorer {
                                range: attack_range,
                            },
                            AttackAction,
                        )
                        .otherwise(MoveAction),
                );
            },
        }

        let hp_bar = spawn_health_bar(
            &mut cmd,
            bevy_bobs::health_bar::HealthBarPrefab {
                dimension: Vec2::new(8., 2.),
                bg_color: Color::BLACK,
                fg_color: Color::GREEN,
                translation: Vec2::ZERO.extend(INGAME_UI_LAYER),
            },
        );
        cmd.entity(id).push_children(&[hp_bar]);

        room_state.register_enemy();
    }
}

fn take_damage(
    mut cmd: Commands,
    mut events: EventReader<DamageEnemyEvent>,
    mut query: Query<(&mut Health, Option<&DropTable>, &GridEntity)>,
    mut writer: EventWriter<SpawnDroppedItemEvent>,
    mut room_state: ResMut<Level>,
) {
    for DamageEnemyEvent { entity } in events.iter() {
        let (mut health, droptable, grid_entity) = query.get_mut(*entity).unwrap();

        health.take(1);
        if health.is_zero() {
            cmd.entity(*entity).despawn_recursive();
            room_state.deregister_enemy();

            // select item to drop
            if let Some(droptable) = droptable {
                if let Some(drop) = droptable.select_single_drop() {
                    writer.send(SpawnDroppedItemEvent {
                        spawn_pos: grid_entity.pos,
                        prefab_id: drop.into(),
                    });
                }
            }
        }
    }
}

fn sync_health_bars(query: Query<(&Health, &Children)>, mut hp_bar_query: Query<&mut HealthBar>) {
    for (health, children) in &query {
        for child in children.iter() {
            if let Ok(mut hp_bar) = hp_bar_query.get_mut(*child) {
                hp_bar.set_percent(health.percent());
            }
        }
    }
}

fn spawn_from_ldtk(
    query: Query<&EntityInstance, Added<EntityInstance>>,
    mut writer: EventWriter<SpawnEnemyEvent>,
    mut room: ResMut<Level>,
) {
    for entity_instance in query.iter().filter(|e| e.identifier == "EnemySpawn") {
        // TODO handle not found
        if let Some(field) = entity_instance
            .field_instances
            .iter()
            .find(|field| field.identifier == "id")
        {
            if let FieldValue::String(Some(v)) = &field.value {
                writer.send(SpawnEnemyEvent {
                    spawn_pos: ldtk_to_bevy(&entity_instance.grid),
                    prefab_id: v.to_owned(),
                });
            }
        }
    }
}
