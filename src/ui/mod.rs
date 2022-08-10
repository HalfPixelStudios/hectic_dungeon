pub mod attack_animation;
pub mod attack_indicator;
pub mod floating_text;
pub mod move_indicator;
pub mod projectile;

use bevy::prelude::*;

use self::{
    attack_animation::AttackAnimationPlugin, attack_indicator::AttackIndicatorPlugin,
    floating_text::FloatingTextPlugin, move_indicator::MoveIndicatorPlugin,
    projectile::ProjectilePlugin,
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AttackIndicatorPlugin)
            .add_plugin(AttackAnimationPlugin)
            .add_plugin(MoveIndicatorPlugin)
            .add_plugin(ProjectilePlugin)
            .add_plugin(FloatingTextPlugin);
    }
}
