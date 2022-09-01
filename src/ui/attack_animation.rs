use bevy::{prelude::*, time::Stopwatch};

use crate::{constants::INGAME_UI_LAYER, grid::to_world_coords, spritesheet::SpriteSheet};

// TODO this is simply a short lived sprite animation, should be replaced by general animation
// system in future

pub struct SpawnAttackAnimEvent {
    pub frames: Vec<usize>,
    pub animation_speed: f32,
    pub spawn_pos: IVec2,
}

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
        app.add_event::<SpawnAttackAnimEvent>()
            .add_system(animate)
            .add_system(spawn)
            .add_startup_system(debug);
    }
}

fn animate(
    mut cmd: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut AttackAnimation, &mut TextureAtlasSprite)>,
) {
    for (entity, mut attack_anim, mut sprite) in &mut query {
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

fn spawn(
    mut cmd: Commands,
    mut events: EventReader<SpawnAttackAnimEvent>,
    asset_sheet: Res<SpriteSheet>,
) {
    for SpawnAttackAnimEvent {
        frames,
        animation_speed,
        spawn_pos,
    } in events.iter()
    {
        let frames = frames.clone();

        if frames.is_empty() {
            continue;
        }

        cmd.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: *frames.get(0).unwrap(),
                ..default()
            },
            texture_atlas: asset_sheet.clone(),
            transform: Transform {
                translation: to_world_coords(spawn_pos).extend(INGAME_UI_LAYER),
                ..default()
            },
            ..default()
        })
        .insert(AttackAnimation::new(frames, *animation_speed));
    }
}

fn debug(mut writer: EventWriter<SpawnAttackAnimEvent>) {
    // writer.send(SpawnAttackAnimEvent { frames: vec![128, 129, 130], animation_speed: 0.2, spawn_pos: IVec2::new(4, 4) });
}
