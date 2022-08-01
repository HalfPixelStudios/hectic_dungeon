use autodefault::autodefault;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub const MAPWIDTH: f32 = 16.;
pub const MAPHEIGHT: f32 = 16.;
pub const TILEWIDTH: f32 = 8.;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::Index(0))
            .add_startup_system(setup);
    }
}

#[autodefault]
fn setup(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn_bundle(LdtkWorldBundle {
        transform: Transform {
            translation: Vec3::new(-TILEWIDTH / 2., -TILEWIDTH / 2., -1.),
            ..default()
        },
        ldtk_handle: asset_server.load("maps/testing.ldtk"),
    });
}
