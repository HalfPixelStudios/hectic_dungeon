mod ability;
mod controller;
mod prefab;
mod useraction;

use bevy::prelude::*;
use bevy_bobs::{
    component::health::*,
    prefab::{PrefabId, PrefabLib},
};
use bevy_ecs_ldtk::{prelude::FieldValue, EntityInstance};
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;
use pino_utils::some_or_continue;

use self::{
    ability::AbilityPlugin,
    controller::{ControllerPlugin, TroopAction},
    prefab::{Class, PlayerPrefab, PrefabPlugin},
    useraction::UserActionPlugin,
};
use crate::{
    camera::CameraFollow,
    prelude::*,
    ui::{attack_indicator::AttackIndicator, move_indicator::MoveIndicator},
};

/// Tag component for the currently selected troop
#[derive(Component)]
pub struct SelectedPlayer;

/// Tag component for troops
#[derive(Component)]
pub struct Player(pub Class);

pub struct PlayerMovedEvent;

pub struct SpawnPlayerEvent {
    pub spawn_pos: IVec2,
    pub prefab_id: PrefabId,
}

pub struct DamagePlayerEvent {
    pub entity: Entity,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PrefabPlugin)
            .add_plugin(ControllerPlugin)
            .add_plugin(UserActionPlugin)
            .add_plugin(AbilityPlugin)
            .add_event::<SpawnPlayerEvent>()
            .add_event::<PlayerMovedEvent>()
            .add_event::<DamagePlayerEvent>();

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ScreenState::Ingame)
                .with_system(spawn)
                .with_system(take_damage)
                .with_system(spawn_from_ldtk)
                .into(),
        )
        .add_exit_system(ScreenState::Ingame, cleanup::<Player>);
    }
}

fn spawn(
    mut cmd: Commands,
    mut events: EventReader<SpawnPlayerEvent>,
    asset_sheet: Res<SpriteSheet>,
    prefab_lib: Res<PrefabLib<PlayerPrefab>>,
    mut room_state: ResMut<Level>,
) {
    for SpawnPlayerEvent {
        spawn_pos,
        prefab_id,
    } in events.iter()
    {
        let input_map = InputMap::new([
            (KeyCode::A, TroopAction::Left),
            (KeyCode::D, TroopAction::Right),
            (KeyCode::W, TroopAction::Up),
            (KeyCode::S, TroopAction::Down),
            (KeyCode::Space, TroopAction::Attack),
            (KeyCode::LShift, TroopAction::ToggleState),
            (KeyCode::E, TroopAction::Interact),
            (KeyCode::Q, TroopAction::Ability),
        ]);

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
            .insert(Player(prefab.class))
            .insert(GridEntity::new(*spawn_pos, CellType::Player(id)))
            .insert(Health::new(prefab.health))
            .insert_bundle(InputManagerBundle::<TroopAction> {
                action_state: ActionState::default(),
                input_map,
            })
            .insert(CameraFollow)
            .insert(CurrentWeapon(prefab.weapon.to_owned()))
            .insert(Movement::new());

        // ui related
        cmd.entity(id)
            .insert(AttackIndicator::default())
            .insert(MoveIndicator::default());
        // .insert(FloatingText {
        //     text: "hello world".into(),
        //     offset: Vec2::new(0., 8.),
        //     ..default()
        // });

        // TODO temp for debug
        if prefab.class == Class::Warrior {
            cmd.entity(id).insert(SelectedPlayer);
        }

        room_state.register_player(id);
    }
}

fn spawn_from_ldtk(
    query: Query<&EntityInstance, Added<EntityInstance>>,
    mut writer: EventWriter<SpawnPlayerEvent>,
) {
    for entity_instance in query.iter().filter(|e| e.identifier == "PlayerSpawn") {
        if let Some(field) = entity_instance
            .field_instances
            .iter()
            .find(|field| field.identifier == "id")
        {
            if let FieldValue::String(Some(v)) = &field.value {
                writer.send(SpawnPlayerEvent {
                    spawn_pos: ldtk_to_bevy(&entity_instance.grid),
                    prefab_id: v.to_owned(),
                });
            }
        }
    }
}

fn take_damage(
    mut cmd: Commands,
    mut events: EventReader<DamagePlayerEvent>,
    mut query: Query<(Entity, &mut Health, &GridEntity)>,
    mut room_state: ResMut<Level>,
) {
    for DamagePlayerEvent { entity } in events.iter() {
        let (entity, mut health, grid_entity) = query.get_mut(*entity).unwrap();

        health.take(1);
        if health.is_zero() {
            room_state.deregister_player(entity);
        }
    }
}
