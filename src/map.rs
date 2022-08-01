use autodefault::autodefault;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct MapPlugin;
pub const MAPWIDTH: f32 = 256.;
pub const MAPHEIGHT: f32 = 256.;

pub const TILEWIDTH: f32 = 16.;
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
            translation: Vec3::new(-MAPWIDTH / 2., -MAPHEIGHT / 2., -1.),
            ..default()
        },
        ldtk_handle: asset_server.load("map/plains.ldtk"),
    });
}
//TODO add gridposition to every GridCoord from map
