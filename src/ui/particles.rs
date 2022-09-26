use autodefault::autodefault;
use bevy::prelude::*;
use bevy_hanabi::*;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HanabiPlugin);
        // .add_startup_system(debug);
    }
}

#[autodefault]
fn debug(mut cmd: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.5, 0.5, 1.0, 1.0));
    gradient.add_key(1.0, Vec4::new(0.5, 0.5, 1.0, 0.0));

    let effect = effects.add(
        EffectAsset {
            name: "Player Damage".to_string(),
            capacity: 1000,
            spawner: Spawner::rate(5.0.into()),
        }
        .init(PositionCircleModifier {
            center: Vec3::ZERO,
            axis: Vec3::Z,
            speed: 6.0.into(),
            radius: 1.,
            dimension: ShapeDimension::Surface,
        })
        .render(ColorOverLifetimeModifier { gradient }),
    );

    cmd.spawn_bundle(ParticleEffectBundle {
        effect: ParticleEffect::new(effect).with_z_layer_2d(Some(10.)),
    });
}
