use bevy::prelude::*;
use bevy_bobs::prefab::PrefabLib;

use crate::{
    assets::{PrefabData, SpriteSheet},
    attack::{rotate_offsets, AttackPattern},
    constants::{INGAME_UI_LAYER, TILE_SIZE},
    grid::GridEntity,
    player::Player,
    spritesheet_constants::SpriteIndex,
    utils::Dir,
    weapon::{CurrentWeapon, WeaponPrefab},
};

#[derive(Component)]
pub struct AttackIndicator {
    pub dir: Dir,
    pub pattern: AttackPattern,
    pub hidden: bool,
}

#[derive(Component)]
struct AttackIndicatorRoot;

impl AttackIndicator {
    pub fn get_pattern(&self) -> Vec<IVec2> {
        let offsets = self.pattern.to_offsets();
        let dir = self.dir;
        rotate_offsets(offsets, dir)
    }
}

impl Default for AttackIndicator {
    fn default() -> Self {
        AttackIndicator {
            dir: Dir::North,
            pattern: AttackPattern::None,
            hidden: true,
        }
    }
}

pub struct AttackIndicatorPlugin;

impl Plugin for AttackIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn)
            .add_system(sync_attack_pattern)
            .add_system(render);
    }
}

fn render(query: Query<(&AttackIndicator, &GridEntity)>) {
    for (attack_indicator, grid_position) in &query {
        let pos: Vec<IVec2> = attack_indicator
            .get_pattern()
            .iter()
            .map(|v| *v + grid_position.pos)
            .collect();
    }
}

fn spawn(
    mut cmd: Commands,
    asset_sheet: Res<SpriteSheet>,
    query: Query<
        (Entity, &AttackIndicator, Option<&Children>),
        Or<(Added<AttackIndicator>, Changed<AttackIndicator>)>,
    >,
    child_query: Query<&AttackIndicatorRoot>,
) {
    for (entity, attack_indictor, children) in &query {
        // despawn existing
        if let Some(children) = children {
            for child in children.iter() {
                if child_query
                    .get_component::<AttackIndicatorRoot>(*child)
                    .is_ok()
                {
                    cmd.entity(*child).despawn_recursive();
                }
            }
        }

        if attack_indictor.hidden {
            continue;
        }

        // spawn root
        let root = cmd.spawn().id();
        cmd.entity(root)
            .insert_bundle(SpatialBundle::from_transform(Transform::from_translation(
                Vec2::ZERO.extend(INGAME_UI_LAYER),
            )))
            .insert(AttackIndicatorRoot);

        cmd.entity(entity).push_children(&[root]);

        // spawn children
        for offset in attack_indictor
            .get_pattern()
            .iter()
            .map(|v| v.as_vec2().extend(0.) * (TILE_SIZE as f32))
        {
            let child = cmd.spawn().id();

            cmd.entity(child).insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: SpriteIndex::AttackIndicator as usize,
                    ..default()
                },
                texture_atlas: asset_sheet.clone(),
                transform: Transform {
                    translation: offset,
                    ..default()
                },
                ..default()
            });
            cmd.entity(root).push_children(&[child]);
        }
    }
}

fn sync_attack_pattern(
    mut query: Query<(&mut CurrentWeapon, &mut AttackIndicator)>,
    prefabs: Res<PrefabLib<WeaponPrefab>>,
) {
    for (cur_weapon, mut attack_indicator) in &mut query {
        let prefab = prefabs.get(&cur_weapon).unwrap();
        attack_indicator.pattern = prefab.attack_pattern;
    }
}
