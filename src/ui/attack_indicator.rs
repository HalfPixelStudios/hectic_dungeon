use bevy::prelude::*;

use crate::{
    assets::{PrefabData, SpriteSheets},
    attack::{rotate_offsets, AttackPattern},
    grid::GridEntity,
    player::Player,
    utils::Dir,
};

// TODO decouple attack / pattern logic from the visual logic

// TODO unify this
const CELLWIDTH: f32 = 8.;

#[derive(Component)]
pub struct AttackIndicator {
    pub dir: Dir,
    pub pattern: AttackPattern,
}

impl AttackIndicator {
    pub fn get_pattern(&self) -> Vec<IVec2> {
        let offsets = self.pattern.to_offsets();
        let dir = self.dir;
        rotate_offsets(offsets, dir)
    }
}

pub struct SpawnAttackIndicatorEvent {
    spawn_grid_pos: IVec2,
}

pub struct DespawnAttackIndicatorEvent {
    /// If the attack was cancelled or not
    ///
    /// If false, will spawn attack particles
    cancelled: bool,
}

pub struct AttackIndicatorPlugin;

impl Plugin for AttackIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnAttackIndicatorEvent>()
            .add_event::<DespawnAttackIndicatorEvent>()
            .add_system(spawn)
            .add_system(despawn)
            .add_system(render)
            .add_system(debug)
            .add_system(control);
    }
}

fn render(query: Query<(&AttackIndicator, &GridEntity)>) {
    for (attack_indicator, grid_position) in query.iter() {
        let pos: Vec<IVec2> = attack_indicator
            .get_pattern()
            .iter()
            .map(|v| *v + grid_position.pos)
            .collect();
    }
}

fn spawn(
    mut cmd: Commands,
    asset_sheet: Res<SpriteSheets>,
    prefab_data: Res<PrefabData>,
    mut events: EventReader<SpawnAttackIndicatorEvent>,
) {
    for SpawnAttackIndicatorEvent { spawn_grid_pos } in events.iter() {
        let attack_indictor = AttackIndicator {
            dir: Dir::North,
            pattern: AttackPattern::Hammer,
        };
        let offsets = attack_indictor.pattern.to_offsets();

        let parent = cmd.spawn().id();
        cmd.entity(parent)
            .insert(attack_indictor)
            .insert_bundle(TransformBundle::from_transform(
                Transform::from_translation(spawn_grid_pos.as_vec2().extend(2.) * CELLWIDTH),
            ));

        for offset in offsets.iter().map(|v| v.as_vec2().extend(0.) * CELLWIDTH) {
            let child = cmd.spawn().id();

            cmd.entity(child).insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 8,
                    ..default()
                },
                texture_atlas: asset_sheet.get("tilesheet").unwrap().clone(),
                transform: Transform {
                    translation: offset,
                    ..default()
                },
                ..default()
            });
            cmd.entity(parent).push_children(&[child]);
        }
    }
}

// TODO this function is disgusting
fn despawn(
    mut cmd: Commands,
    mut events: EventReader<DespawnAttackIndicatorEvent>,
    query: Query<(Entity, &AttackIndicator), Without<Player>>,
    player_query: Query<&GridEntity, With<Player>>,
) {
    for DespawnAttackIndicatorEvent { cancelled } in events.iter() {
        // TODO despawn all indicators for now
        for (e, attack_indicator) in query.iter() {
            // Spawn attack animations
            if !cancelled {}

            cmd.entity(e).despawn_recursive();
        }
    }
}

fn debug(
    keys: Res<Input<KeyCode>>,
    mut writer: EventWriter<SpawnAttackIndicatorEvent>,
    query: Query<&GridEntity, With<Player>>,
) {
    for grid_position in query.iter() {
        if keys.just_pressed(KeyCode::E) {
            writer.send(SpawnAttackIndicatorEvent {
                spawn_grid_pos: grid_position.pos,
            });
        }
    }
}

fn control(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut AttackIndicator, &mut Transform)>,
    mut writer: EventWriter<DespawnAttackIndicatorEvent>,
) {
    use std::f32::consts::PI;

    for (mut attack_indicator, mut transform) in query.iter_mut() {
        // TODO changing rotation is a quick and dirty implementation. maybe better to rearrange
        // existing children

        if keys.just_pressed(KeyCode::Up) {
            attack_indicator.dir = Dir::North;
            transform.rotation = Quat::from_rotation_z(0.);
        }
        if keys.just_pressed(KeyCode::Left) {
            attack_indicator.dir = Dir::West;
            transform.rotation = Quat::from_rotation_z(PI / 2.);
        }
        if keys.just_pressed(KeyCode::Down) {
            attack_indicator.dir = Dir::South;
            transform.rotation = Quat::from_rotation_z(PI);
        }
        if keys.just_pressed(KeyCode::Right) {
            attack_indicator.dir = Dir::East;
            transform.rotation = Quat::from_rotation_z(3. * PI / 2.);
        }
        if keys.just_pressed(KeyCode::Space) {
            writer.send(DespawnAttackIndicatorEvent { cancelled: false });
        }
    }
}
