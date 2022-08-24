use autodefault::*;
use bevy::prelude::*;
use bevy_bobs::component::health::Health;

use crate::{player::Player, screens::utils::FONT_PATH, utils::ok_or_return};

#[derive(Component)]
struct HealthNode;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update);
    }
}

#[autodefault]
pub fn HealthBar(cmd: &mut ChildBuilder, assets: &Res<AssetServer>) -> Entity {
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
