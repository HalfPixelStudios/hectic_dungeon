pub mod health;

use bevy::prelude::*;

use self::health::HealthPlugin;

pub struct ComponentPlugin;

impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HealthPlugin);
    }
}
