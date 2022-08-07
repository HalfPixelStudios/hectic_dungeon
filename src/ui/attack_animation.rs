use std::time::Duration;

use bevy::{core::Stopwatch, prelude::*};

// TODO this is simply a short lived sprite animation, should be replaced by general animation
// system in future

#[derive(Component)]
pub struct AttackAnimation {
    frame: usize,
    frame_indices: Vec<usize>,
    /// Seconds between each frame
    animation_speed: f32,
    timer: Stopwatch,
}

impl AttackAnimation {
    pub fn new(frame_indices: Vec<usize>, animation_speed: f32) -> Self {
        AttackAnimation {
            frame: 0,
            frame_indices,
            animation_speed,
            timer: Stopwatch::default(),
        }
    }
}

pub struct AttackAnimationPlugin;

impl Plugin for AttackAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate);
    }
}

fn animate(
    mut cmd: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut AttackAnimation, &mut TextureAtlasSprite)>,
) {
    for (entity, mut attack_anim, mut sprite) in query.iter_mut() {
        sprite.index = *attack_anim.frame_indices.get(attack_anim.frame).unwrap();

        if attack_anim.timer.elapsed_secs() > attack_anim.animation_speed {
            attack_anim.frame += 1;
            attack_anim.timer.reset();
        }

        attack_anim.timer.tick(time.delta());

        // self destruct
        if attack_anim.frame >= attack_anim.frame_indices.len() {
            cmd.entity(entity).despawn_recursive();
        }
    }
}
