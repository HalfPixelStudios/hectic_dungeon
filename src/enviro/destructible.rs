use bevy::prelude::*;
use bevy_bobs::component::health::Health;
use bevy_ecs_ldtk::EntityInstance;
use iyes_loopless::prelude::*;

use crate::{
    constants::BEING_LAYER,
    grid::{to_world_coords, CellType, GridEntity},
    map::ldtk_to_bevy,
    screens::state::ScreenState,
    spritesheet::{SpriteIndex, SpriteSheet},
    utils::cleanup,
};

#[derive(Component)]
pub struct Destructible;

pub struct DestructiblePlugin;

impl Plugin for DestructiblePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_from_ldtk)
            .add_exit_system(ScreenState::Ingame, cleanup::<Destructible>);
    }
}

fn spawn_from_ldtk(
    mut cmd: Commands,
    query: Query<&EntityInstance, Added<EntityInstance>>,
    asset_sheet: Res<SpriteSheet>,
) {
    for entity_instance in query.iter().filter(|e| e.identifier == "Destructible") {
        if let Some(field) = entity_instance
            .field_instances
            .iter()
            .find(|field| field.identifier == "id")
        {
            let id = cmd.spawn().id();
            let grid_coords = ldtk_to_bevy(&entity_instance.grid);

            cmd.entity(id)
                .insert_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: SpriteIndex::Barrel as usize,
                        ..default()
                    },
                    texture_atlas: asset_sheet.clone(),
                    transform: Transform {
                        translation: to_world_coords(&grid_coords).extend(BEING_LAYER),
                        ..default()
                    },
                    ..default()
                })
                .insert(Destructible)
                .insert(Health::new(1))
                .insert(GridEntity {
                    pos: grid_coords,
                    value: CellType::Enemy(id),
                });
        }
    }
}
