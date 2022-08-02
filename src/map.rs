use autodefault::autodefault;
use bevy::{ecs::query, prelude::*};
use bevy_ecs_ldtk::prelude::*;

use crate::grid::snap_to_grid;

pub const MAPWIDTH: f32 = 16.;
pub const MAPHEIGHT: f32 = 16.;
pub const TILEWIDTH: f32 = 8.;

pub struct MapPlugin;

/// List of all collisions
#[derive(Deref, DerefMut)]
pub struct CollisionMap(pub Vec<IVec2>);

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::Index(0))
            .insert_resource(CollisionMap(Vec::new()))
            .add_startup_system(setup)
            .add_system(register_collision_int_cell)
            .add_system(register_spawn_points);
    }
}

#[autodefault]
fn setup(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn_bundle(LdtkWorldBundle {
        transform: Transform {
            translation: Vec3::new(-TILEWIDTH / 2., -TILEWIDTH / 2., -1.),
        },
        ldtk_handle: asset_server.load("maps/testing.ldtk"),
    });
}

fn register_collision_int_cell(
    mut collision_map: ResMut<CollisionMap>,
    query: Query<(&Transform, &IntGridCell), Added<IntGridCell>>,
) {
    for (transform, int_cell) in query.iter() {
        collision_map.push(snap_to_grid(&transform.translation.truncate()));
    }
}

fn register_spawn_points(query: Query<(&EntityInstance), Added<EntityInstance>>) {
    for entity_instance in query.iter() {
        // TODO handle not found
        if let Some(field) = entity_instance
            .field_instances
            .iter()
            .find(|field| field.identifier == "id")
        {
            if let FieldValue::String(Some(v)) = &field.value {
                info!("field instance {:?} {:?}", v, entity_instance.grid);
            }
        }
    }
}
