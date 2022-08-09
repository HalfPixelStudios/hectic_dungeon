use bevy::prelude::*;
use bevy_bobs::prefab::PrefabId;

use crate::grid::{CellType, Grid};

#[derive(Component)]
pub struct DroppedItem;

pub struct SpawnDroppedItemEvent {
    pub pos: IVec2,
    pub id: PrefabId,
}

pub struct DroppedItemPlugin;

impl Plugin for DroppedItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnDroppedItemEvent>();
    }
}

fn spawn(
    mut cmd: Commands,
    mut events: EventReader<SpawnDroppedItemEvent>,
    grid: Res<Grid<CellType>>,
) {
    for SpawnDroppedItemEvent { pos, id } in events.iter() {
        let id = cmd.spawn().id();
    }
}
