pub mod outline;

use bevy::{prelude::*, sprite::Material2dPlugin};

use self::outline::OutlineMaterial;

pub struct MaterialPlugin;

impl Plugin for MaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<OutlineMaterial>::default());
    }
}
