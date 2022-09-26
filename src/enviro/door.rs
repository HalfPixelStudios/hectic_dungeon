use bevy::prelude::*;

#[derive(Component)]
pub struct Door {
    pub open: bool,
}

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, _app: &mut App) {}
}

// fn spawn_from_ldtk(
//     mut cmd: Commands,
//     query: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
//     asset_sheet: Res<SpriteSheet>,
// ) {
// }
