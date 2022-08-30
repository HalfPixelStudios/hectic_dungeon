use std::thread::current;

use autodefault::*;
use bevy::prelude::*;
use bevy_bobs::component::health::Health;

use crate::{
    assets::SpriteSheet, player::Player, screens::utils::FONT_PATH,
    spritesheet_constants::SpriteIndex, utils::ok_or_return,
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
        })
        .id()
}

#[autodefault]
fn update(
    mut cmd: Commands,
    mut player_query: Query<&Health, With<Player>>,
    mut ui_query: Query<(Entity, &mut HealthNode), Without<Player>>,
    asset_server: Res<AssetServer>,
) {
    let health = ok_or_return!(player_query.get_single());

    // TODO kinda inefficnet to despawn all health nodes and respawn every frame
    for (entity, mut health_node) in ui_query.iter_mut() {
        cmd.entity(entity).despawn_descendants();

        for i in 0..health.current() {
            cmd.entity(entity).with_children(|parent| {
                parent.spawn().insert_bundle(ImageBundle {
                    image: UiImage(asset_server.load("tilesheet/heart.png")),
                    style: Style {},
                });
            });
        }
    }
}
