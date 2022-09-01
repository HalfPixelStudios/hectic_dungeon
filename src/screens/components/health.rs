use autodefault::*;
use bevy::{prelude::*, ui::FocusPolicy};
use bevy_bobs::component::health::Health;
use pino_utils::ok_or_return;

use crate::{
    player::Player,
    spritesheet::{SpriteIndex, SpriteSheet},
};

#[derive(Component)]
struct HealthNode;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update);
    }
}

#[autodefault]
#[allow(non_snake_case)]
pub fn HealthBar(cmd: &mut ChildBuilder) -> Entity {
    cmd.spawn()
        .insert(HealthNode)
        .insert_bundle(NodeBundle {
            style: Style {
                display: Display::Flex,
            },
            color: UiColor(Color::NONE),
        })
        .id()
}

#[autodefault]
fn update(
    mut cmd: Commands,
    mut player_query: Query<&Health, With<Player>>,
    mut ui_query: Query<(Entity, &mut HealthNode), Without<Player>>,
    assets: Res<AssetServer>,
    asset_sheet: Res<SpriteSheet>,
) {
    let health = ok_or_return!(player_query.get_single());

    // TODO kinda inefficnet to despawn all health nodes and respawn every frame
    for (entity, mut health_node) in ui_query.iter_mut() {
        cmd.entity(entity).despawn_descendants();

        // TODO this does not work
        cmd.entity(entity).with_children(|parent| {
            parent
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: SpriteIndex::PlayerWarrior as usize,
                    },
                    texture_atlas: asset_sheet.clone(),
                })
                .insert(Node::default())
                .insert(Style::default())
                .insert(CalculatedSize {
                    size: Size::new(8., 8.),
                })
                .insert(FocusPolicy::default())
                .insert(UiImage::default())
                .insert(UiColor::default())
                .insert_bundle(TransformBundle::default())
                .insert_bundle(VisibilityBundle::default());
        });

        for i in 0..health.current() {
            cmd.entity(entity).with_children(|parent| {
                parent.spawn().insert_bundle(ImageBundle {
                    image: UiImage(assets.load("tilesheet/heart.png")),
                    style: Style {},
                });
            });
        }
    }
}
