use bevy::prelude::*;
use bevy_bobs::{
    component::lifetime::{LifetimePlugin, *},
    physics_2d::{PhysicsPlugin, RigidBody},
};

use crate::{
    constants::TILE_SIZE,
    grid::to_world_coords,
    spritesheet::SpriteSheet,
    utils::{to_rotation, Dir},
};

#[derive(Component)]
pub struct Projectile;

pub struct SpawnProjectileEvent {
    pub sprite_index: usize,
    pub spawn_pos: IVec2,
    pub dir: Dir,
    pub distance: u32,
    pub speed: f32,
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PhysicsPlugin)
            .add_event::<SpawnProjectileEvent>()
            .add_plugin(LifetimePlugin)
            .add_system(spawn)
            .add_system(despawn);
        // .add_system(debug);
    }
}

fn spawn(
    mut cmd: Commands,
    mut events: EventReader<SpawnProjectileEvent>,
    asset_sheet: Res<SpriteSheet>,
) {
    use std::f32::consts::PI;

    for SpawnProjectileEvent {
        sprite_index,
        spawn_pos,
        dir,
        distance,
        speed,
    } in events.iter()
    {
        cmd.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: *sprite_index,
                ..default()
            },
            texture_atlas: asset_sheet.clone(),
            transform: Transform {
                translation: to_world_coords(spawn_pos).extend(2.),
                rotation: Quat::from_rotation_z(to_rotation(*dir) - PI / 2.),
                ..default()
            },
            ..default()
        })
        .insert(DistanceLifetime::new(*distance as f32))
        .insert(RigidBody {
            mass: 1.,
            velocity: *speed * IVec2::from(*dir).as_vec2(),
            ..default()
        })
        .insert(Projectile);
    }
}

fn despawn(mut cmd: Commands, query: Query<(Entity, &DistanceLifetime), With<Projectile>>) {
    for (entity, distance_lifetime) in &query {
        if distance_lifetime.is_expired() {
            cmd.entity(entity).despawn();
        }
    }
}

fn debug(mut writer: EventWriter<SpawnProjectileEvent>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::E) {
        writer.send(SpawnProjectileEvent {
            sprite_index: 39,
            spawn_pos: IVec2::new(4, 4),
            dir: Dir::North,
            distance: 5 * (TILE_SIZE as u32),
            speed: 200.,
        });
    }
}
