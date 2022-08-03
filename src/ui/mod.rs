// TODO this does not need (should not need) to be pub
pub mod attack_indicator;

use bevy::prelude::*;

use self::attack_indicator::AttackIndicatorPlugin;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AttackIndicatorPlugin);
    }
}
