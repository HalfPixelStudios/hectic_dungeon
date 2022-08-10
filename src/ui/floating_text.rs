use bevy::prelude::*;

#[derive(Component)]
pub struct FloatingText {
    pub text: String,
    pub offset: Vec2,
    pub hidden: bool,
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
    query: Query<(&FloatingText, Option<&Children>), Added<FloatingText>>,
) {
    for (
        FloatingText {
            text,
            offset,
            hidden,
        },
        children,
    ) in query.iter()
    {
        let id = cmd.spawn().id();
        let font = asset_server.load("fonts/arcadeclassic.tff");

        cmd.entity(id).insert_bundle(Text2dBundle {
            transform: Transform::from_translation(offset.extend(0.)),
            text: Text::with_section(
                text,
                TextStyle {
                    font: font.clone(),
                    font_size: 30.,
                    color: Color::WHITE,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..default()
        });
    }
}
