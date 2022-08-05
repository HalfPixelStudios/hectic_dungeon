use bevy::prelude::*;

use crate::{
    assets::{PrefabData, SpriteSheet},
    attack::{rotate_offsets, AttackPattern},
    grid::GridEntity,
    player::Player,
    utils::Dir,
};

// TODO unify this
const CELLWIDTH: f32 = 8.;

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
            // .add_system(despawn)
            .add_system(render);
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
    asset_sheet: Res<SpriteSheet>,
    query: Query<
        (Entity, &AttackIndicator, Option<&Children>),
        Or<(Added<AttackIndicator>, Changed<AttackIndicator>)>,
    >,
    child_query: Query<&AttackIndicatorRoot>,
) {
    for (entity, attack_indictor, children) in query.iter() {
        // despawn existing
        if let Some(children) = children {
            for child in children.iter() {
                if let Ok(_) = child_query.get_component::<AttackIndicatorRoot>(*child) {
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
            .insert_bundle(TransformBundle::from_transform(
                Transform::from_translation(Vec2::ZERO.extend(2.)),
            ))
            .insert(AttackIndicatorRoot);

        cmd.entity(entity).add_child(root);

        // spawn children
        for offset in attack_indictor
            .get_pattern()
            .iter()
            .map(|v| v.as_vec2().extend(0.) * CELLWIDTH)
        {
            let child = cmd.spawn().id();

            cmd.entity(child).insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 8,
                    ..default()
                },
                texture_atlas: asset_sheet.clone(),
                transform: Transform {
                    translation: offset,
                    ..default()
                },
                ..default()
            });
            cmd.entity(root).add_child(child);
        }
    }
}
