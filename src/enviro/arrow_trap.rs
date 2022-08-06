use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::FieldValue, EntityInstance};

use crate::{assets::SpriteSheet, utils::Dir};

pub struct ArrowTrapPlugin;

impl Plugin for ArrowTrapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_from_ldtk);
    }
}

fn spawn_from_ldtk(
    mut cmd: Commands,
    query: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
    asset_sheet: Res<SpriteSheet>,
) {
    for (entity, entity_instance) in query.iter().filter(|(_, t)| t.identifier == "ArrowTrap") {
        // TODO this code is sorta cringe
        let dir: Dir = entity_instance
            .field_instances
            .iter()
            .find(|field| field.identifier.to_lowercase() == "dir")
            .map_or(Dir::East, |field_inst| {
                if let FieldValue::String(Some(dir)) = &field_inst.value {
                    Dir::from(dir.to_owned())
                } else {
                    Dir::East
                }
            });

        info!("arrow trap facing {:?}", dir);
    }
}
