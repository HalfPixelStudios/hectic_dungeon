/// Floating text above entity

// TODO support multiple text
// TODO stylized text

const FONT_SIZE: f32 = 8.;

use bevy::prelude::*;

#[derive(Component)]
pub struct FloatingText {
    pub text: String,
    pub offset: Vec2,
}

impl Default for FloatingText {
    fn default() -> Self {
        FloatingText {
            text: String::new(),
            offset: Vec2::ZERO,
        }
    }
}

#[derive(Component)]
pub struct FloatingTextRoot;

pub struct FloatingTextPlugin;

impl Plugin for FloatingTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn);
    }
}

fn spawn(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &FloatingText, Option<&Children>), Added<FloatingText>>,
) {
    let font = asset_server.load("fonts/arcadeclassic.ttf");

    for (entity, FloatingText { text, offset }, children) in query.iter() {
        let id = cmd.spawn().id();

        cmd.entity(id).insert_bundle(Text2dBundle {
            transform: Transform::from_translation(offset.extend(3.)),
            text: Text::from_section(
                text,
                TextStyle {
                    font: font.clone(),
                    font_size: FONT_SIZE,
                    color: Color::BLACK,
                },
            )
            .with_alignment(TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            }),
            ..default()
        });

        cmd.entity(entity).add_child(id);
    }
}
