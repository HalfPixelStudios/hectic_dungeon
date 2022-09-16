use autodefault::*;
use bevy::prelude::*;
use bevy_bobs::component::health::Health;
use pino_utils::ok_or_return;

use crate::{
    player::{Player, SelectedPlayer},
    spritesheet::SpriteSheet,
};

#[derive(Component)]
struct HealthNode;

#[derive(Component)]
struct ClassNameText;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_health).add_system(update_class_text);
    }
}

#[autodefault]
#[allow(non_snake_case)]
pub fn HealthBar(cmd: &mut ChildBuilder, assets: Res<AssetServer>) -> Entity {
    let font = assets.load("fonts/arcadeclassic.ttf");

    cmd.spawn()
        .insert_bundle(NodeBundle {
            color: UiColor(Color::NONE),
            style: Style {
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
            },
        })
        .with_children(|parent| {
            parent.spawn().insert(HealthNode).insert_bundle(NodeBundle {
                style: Style {
                    display: Display::Flex,
                },
                color: UiColor(Color::NONE),
            });

            parent
                .spawn()
                .insert(ClassNameText)
                .insert_bundle(TextBundle {
                    text: Text::from_section(
                        String::new(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 20.,
                            color: Color::WHITE,
                        },
                    ),
                });
        })
        .id()
}

#[autodefault]
fn update_health(
    mut cmd: Commands,
    mut player_query: Query<&Health, With<SelectedPlayer>>,
    mut ui_query: Query<(Entity, &mut HealthNode), Without<SelectedPlayer>>,
    assets: Res<AssetServer>,
    asset_sheet: Res<SpriteSheet>,
) {
    let health = ok_or_return!(player_query.get_single());

    // TODO kinda inefficnet to despawn all health nodes and respawn every frame
    for (entity, mut health_node) in ui_query.iter_mut() {
        cmd.entity(entity).despawn_descendants();

        // TODO this does not work
        /*
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
        */

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

fn update_class_text(
    mut cmd: Commands,
    mut player_query: Query<&Player, With<SelectedPlayer>>,
    mut ui_query: Query<(&mut Text), (With<ClassNameText>, Without<SelectedPlayer>)>,
) {
    let mut text = ok_or_return!(ui_query.get_single_mut());
    let player = ok_or_return!(player_query.get_single());

    text.sections[0].value = player.0.to_string();
}
