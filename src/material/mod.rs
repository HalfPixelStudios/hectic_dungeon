pub mod outline;

use bevy::{
    prelude::*,
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
};

use self::outline::OutlineMaterial;

pub struct MaterialPlugin;

impl Plugin for MaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<OutlineMaterial>::default())
            .add_startup_system(debug);
    }
}

fn debug(
    mut cmd: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<OutlineMaterial>>,
    assets: Res<AssetServer>,
) {
    cmd.spawn_bundle(MaterialMesh2dBundle {
        mesh: mesh_assets
            .add(Mesh::from(shape::Quad::new(Vec2::new(80., 80.))))
            .into(),
        material: material_assets
            .add(OutlineMaterial {
                color: Color::RED,
                offset: 0.01,
                image: assets.load("mascot.png"),
            })
            .into(),
        transform: Transform::from_translation(Vec2::ZERO.extend(5.)),
        ..default()
    });
}
