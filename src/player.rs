use bevy::prelude::*;

use crate::{assets::{SpriteSheets, PrefabData, BeingPrefab}, animation::Animation};

#[derive(Component)]
pub struct Player;


pub struct SpawnPlayerEvent {
    pub spawn_pos: Vec2,
}
fn spawn_player_system(
    mut cmd: Commands,
    mut events: EventReader<SpawnPlayerEvent>,
    asset_sheet: Res<SpriteSheets>,
    prefab_data: Res<PrefabData>,
    beings: Res<Assets<BeingPrefab>>
) {
    for SpawnPlayerEvent { spawn_pos } in events.iter() {
        info!("spawning");

        let player = beings.get(prefab_data.get("archer").unwrap()).unwrap();
        cmd.spawn()
            .insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index:0,
                    ..default()
                },
                texture_atlas: asset_sheet.get("archer").unwrap().clone(),
                transform: Transform {
                    translation: spawn_pos.extend(0.),
                    ..default()
                },
                ..default()
            })
            .insert(Animation::new(&player.anim));

    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnPlayerEvent>()
            .add_system(spawn_player_system);
    }
}
