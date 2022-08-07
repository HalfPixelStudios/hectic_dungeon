pub mod attack_animation;
pub mod attack_indicator;
pub mod move_indicator;

use bevy::prelude::*;

use self::{
    attack_animation::AttackAnimationPlugin, attack_indicator::AttackIndicatorPlugin,
    move_indicator::MoveIndicatorPlugin,
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AttackIndicatorPlugin)
            .add_plugin(AttackAnimationPlugin)
            .add_plugin(MoveIndicatorPlugin);
    }
}
