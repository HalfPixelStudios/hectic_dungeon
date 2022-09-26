use autodefault::autodefault;
use bevy::prelude::*;
use bevy_hanabi::*;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HanabiPlugin);
    }
}

#[autodefault]
pub fn player_move(cmd: &mut Commands, effects: &mut ResMut<Assets<EffectAsset>>) -> Entity {
    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(0.5, 0.5, 0.5, 1.0));
    color_gradient.add_key(1.0, Vec4::new(0.5, 0.5, 0.5, 0.0));

    let effect = effects.add(
        EffectAsset {
            name: "Player Move".to_string(),
            capacity: 10,
            spawner: Spawner::rate(5.0.into()),
        }
        .init(PositionCircleModifier {
            center: Vec3::ZERO,
            axis: Vec3::Z,
            speed: 1.0.into(),
            radius: 0.1,
            dimension: ShapeDimension::Surface,
        })
        .init(ParticleLifetimeModifier { lifetime: 1. })
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient,
        }),
    );

    cmd.spawn_bundle(ParticleEffectBundle {
        effect: ParticleEffect::new(effect).with_z_layer_2d(Some(10.)),
    })
    .id()
}

#[autodefault]
pub fn hurt(cmd: &mut Commands, effects: &mut ResMut<Assets<EffectAsset>>) -> Entity {
    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(1.0, 0.0, 0.0, 1.0));
    color_gradient.add_key(1.0, Vec4::new(1.0, 0.0, 0.0, 0.0));

    let effect = effects.add(
        EffectAsset {
            name: "Hurt".to_string(),
            capacity: 100,
            spawner: Spawner::once(Value::Uniform((5., 10.)), true),
        }
        .init(PositionCircleModifier {
            center: Vec3::ZERO,
            axis: Vec3::Z,
            speed: 6.0.into(),
            radius: 1.,
            dimension: ShapeDimension::Surface,
        })
        .init(ParticleLifetimeModifier { lifetime: 1. })
        .update(AccelModifier {
            accel: Vec3::new(0., -10., 0.),
        })
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient,
        }),
    );

    cmd.spawn_bundle(ParticleEffectBundle {
        effect: ParticleEffect::new(effect).with_z_layer_2d(Some(10.)),
    })
    .id()
}
