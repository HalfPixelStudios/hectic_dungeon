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
pub fn HealthBar(
    cmd: &mut ChildBuilder,
    assets: &Res<AssetServer>,
    asset_sheet: Res<SpriteSheet>,
) -> Entity {
    /*
    cmd.spawn()
        .insert(HealthNode)
        .insert_bundle(TextBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font: assets.load(FONT_PATH),
                    font_size: 20.,
                    color: Color::CRIMSON,
                },
            ),
        })
        .id()
    */

    cmd.spawn_bundle(NodeBundle {
        color: UiColor(Color::NONE),
    })
    .with_children(|parent| {
        parent
            .spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: SpriteIndex::Player as usize,
                    ..default()
                },
                texture_atlas: asset_sheet.clone(),
            })
            .insert(Style::default())
            .insert(Node::default())
            .insert(CalculatedSize {
                size: Size::new(30., 30.),
            });
    })
    .id()
}

fn update(
    mut player_query: Query<&Health, With<Player>>,
    mut ui_query: Query<&mut Text, With<HealthNode>>,
) {
    let health = ok_or_return!(player_query.get_single());

    for mut text in ui_query.iter_mut() {
        text.sections[0].value = format!("health {}", health.current());
    }
}
