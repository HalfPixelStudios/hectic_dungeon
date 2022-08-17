use bevy::prelude::*;
use bevy_bobs::prefab::{PrefabId, PrefabLib};

use crate::{
    assets::SpriteSheet,
    constants::BEING_LAYER,
    grid::{to_world_coords, CellType, Grid, GridEntity},
    item::ItemPrefab,
    utils::some_or_return,
};

#[derive(Component)]
pub struct DroppedItem {
    pub prefab_id: PrefabId,
}

pub struct SpawnDroppedItemEvent {
    pub spawn_pos: IVec2,
    pub prefab_id: PrefabId,
}

pub struct DroppedItemPlugin;

impl Plugin for DroppedItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnDroppedItemEvent>()
            .add_system(spawn)
            .add_system(debug);
    }
}

fn spawn(
    mut cmd: Commands,
    mut events: EventReader<SpawnDroppedItemEvent>,
    grid: Res<Grid>,
    asset_sheet: Res<SpriteSheet>,
    prefab_lib: Res<PrefabLib<ItemPrefab>>,
) {
    for SpawnDroppedItemEvent {
        spawn_pos,
        prefab_id,
    } in events.iter()
    {
        let prefab = some_or_return!(prefab_lib.get(prefab_id));
        let id = cmd.spawn().id();

        cmd.entity(id)
            .insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: prefab.sprite_index,
                    ..default()
                },
                texture_atlas: asset_sheet.clone(),
                transform: Transform {
                    translation: to_world_coords(spawn_pos).extend(BEING_LAYER),
                    ..default()
                },
                ..default()
            })
            .insert(DroppedItem {
                prefab_id: prefab_id.to_owned(),
            })
            .insert(GridEntity::new(*spawn_pos, CellType::DroppedItem(id)));
    }
}

fn debug(keys: Res<Input<KeyCode>>, mut writer: EventWriter<SpawnDroppedItemEvent>) {
    if keys.just_pressed(KeyCode::P) {
        writer.send(SpawnDroppedItemEvent {
            spawn_pos: IVec2::new(4, 4),
            prefab_id: "steel_sword".into(),
        });
    }
}
