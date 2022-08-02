use bevy::prelude::*;

use crate::{
    assets::{PrefabData, SpriteSheets},
    grid::GridPosition,
    player::Player,
};

// TODO unify this
const CELLWIDTH: f32 = 8.;

// TODO make a proper 2d direction utility
pub enum Dir {
    North,
    East,
    South,
    West,
}

pub enum AttackPattern {
    StraightOne,
    StraightTwo,
    Hammer,
}

#[derive(Component)]
pub struct AttackIndicator {
    pub dir: Dir,
    pub pattern: AttackPattern,
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

impl AttackIndicator {
    // default north
    pub fn to_offsets(&self) -> Vec<IVec2> {
        let north = match &self.pattern {
            AttackPattern::StraightOne => vec![IVec2::new(0, 1)],
            AttackPattern::StraightTwo => vec![IVec2::new(0, 1), IVec2::new(0, 2)],
            AttackPattern::Hammer => vec![
                IVec2::new(-1, 1),
                IVec2::new(0, 1),
                IVec2::new(1, 1),
                IVec2::new(-1, 2),
                IVec2::new(0, 2),
                IVec2::new(1, 2),
            ],
        };

        // rotate offsets based on direction
        Self::rotate_offsets(north, &self.dir)
    }

    fn rotate_offsets(vecs: Vec<IVec2>, dir: &Dir) -> Vec<IVec2> {
        match dir {
            Dir::North => vecs,
            Dir::East => vecs.into_iter().map(|v| IVec2::new(-v.y, v.x)).collect(),
            Dir::South => vecs.into_iter().map(|v| IVec2::new(-v.x, -v.y)).collect(),
            Dir::West => vecs.into_iter().map(|v| IVec2::new(v.y, -v.x)).collect(),
        }
    }
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

fn render(query: Query<(&AttackIndicator, &GridPosition)>) {
    for (attack_indicator, grid_position) in query.iter() {
        let pos: Vec<IVec2> = attack_indicator
            .to_offsets()
            .iter()
            .map(|v| *v + grid_position.pos())
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
        let offsets = attack_indictor.to_offsets();

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

fn despawn(
    mut cmd: Commands,
    mut events: EventReader<DespawnAttackIndicatorEvent>,
    query: Query<(Entity, &AttackIndicator)>,
) {
    for DespawnAttackIndicatorEvent { cancelled } in events.iter() {
        // TODO despawn all indicators for now
        for (e, attack_indicator) in query.iter() {
            // Spawn attack animations
            if !cancelled {
                let grid_positions = attack_indicator.to_offsets();
            }

            cmd.entity(e).despawn_recursive();
        }
    }
}

fn debug(
    keys: Res<Input<KeyCode>>,
    mut writer: EventWriter<SpawnAttackIndicatorEvent>,
    query: Query<&GridPosition, With<Player>>,
) {
    for grid_position in query.iter() {
        if keys.just_pressed(KeyCode::E) {
            writer.send(SpawnAttackIndicatorEvent {
                spawn_grid_pos: grid_position.pos(),
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
