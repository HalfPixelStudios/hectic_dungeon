use bevy::prelude::*;

use crate::{
    constants::{INGAME_UI_LAYER, TILE_SIZE},
    spritesheet::{SpriteIndex, SpriteSheet},
    utils::Dir,
};

pub struct MoveIndicatorPlugin;

#[derive(Component)]
pub struct MoveIndicator {
    pub hidden: bool,
    pub dirs: Vec<Dir>,
}

impl Default for MoveIndicator {
    fn default() -> Self {
        MoveIndicator {
            hidden: true,
            dirs: vec![],
        }
    }
}

#[derive(Component)]
struct MoveIndicatorRoot;

impl Plugin for MoveIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn);
    }
}

fn spawn(
    mut cmd: Commands,
    asset_sheet: Res<SpriteSheet>,
    query: Query<
        (Entity, &MoveIndicator, Option<&Children>),
        Or<(Added<MoveIndicator>, Changed<MoveIndicator>)>,
    >,
    child_query: Query<&MoveIndicatorRoot>,
) {
    for (entity, move_indicator, children) in &query {
        // despawn existing (TODO duplicated code from attack_indicator.rs)
        if let Some(children) = children {
            for child in children.iter() {
                if child_query
                    .get_component::<MoveIndicatorRoot>(*child)
                    .is_ok()
                {
                    cmd.entity(*child).despawn_recursive();
                }
            }
        }

        if move_indicator.hidden {
            continue;
        }

        let root = cmd.spawn().id();
        cmd.entity(root)
            .insert_bundle(SpatialBundle::from_transform(Transform::from_translation(
                Vec2::ZERO.extend(INGAME_UI_LAYER),
            )))
            .insert(MoveIndicatorRoot);

        cmd.entity(entity).add_child(root);

        // spawn children
        for dir in move_indicator.dirs.iter() {
            let sprite_index = match dir {
                Dir::West => SpriteIndex::MoveIndicatorW as usize,
                Dir::North => SpriteIndex::MoveIndicatorN as usize,
                Dir::South => SpriteIndex::MoveIndicatorS as usize,
                Dir::East => SpriteIndex::MoveIndicatorE as usize,
                _ => unreachable!(), // TODO not safe
            };

            let child = cmd.spawn().id();
            let offset = IVec2::from(*dir).as_vec2() * (TILE_SIZE as f32);

            cmd.entity(child).insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: sprite_index,
                    ..default()
                },
                texture_atlas: asset_sheet.clone(),
                transform: Transform {
                    translation: offset.extend(0.),
                    ..default()
                },
                ..default()
            });
            cmd.entity(root).add_child(child);
        }
    }
}
