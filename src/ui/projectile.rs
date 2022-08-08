use bevy::prelude::*;
use bevy_bobs::{
    component::lifetime::*,
    physics_2d::{PhysicsPlugin, RigidBody},
};

use crate::{assets::SpriteSheet, grid::to_world_coords, utils::Dir};

const CELL_TYPE: u32 = 8;

#[derive(Component)]
pub struct Projectile;

pub struct SpawnProjectileEvent {
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
            .add_system(duration_lifetime_system)
            .add_system(distance_lifetime_system)
            .add_system(spawn)
            .add_system(despawn)
            .add_system(debug);
    }
}

fn spawn(
    mut cmd: Commands,
    mut events: EventReader<SpawnProjectileEvent>,
    asset_sheet: Res<SpriteSheet>,
) {
    for SpawnProjectileEvent {
        spawn_pos,
        dir,
        distance,
        speed,
    } in events.iter()
    {
        cmd.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 144,
                ..default()
            },
            texture_atlas: asset_sheet.clone(),
            transform: Transform {
                translation: to_world_coords(spawn_pos).extend(2.),
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
    for (entity, distance_lifetime) in query.iter() {
        if distance_lifetime.is_expired() {
            cmd.entity(entity).despawn();
        }
    }
}

fn debug(mut writer: EventWriter<SpawnProjectileEvent>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::E) {
        writer.send(SpawnProjectileEvent {
            spawn_pos: IVec2::new(4, 4),
            dir: Dir::North,
            distance: 5 * CELL_TYPE,
            speed: 200.,
        });
    }
}
