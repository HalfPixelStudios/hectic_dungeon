use autodefault::autodefault;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use iyes_loopless::{
    prelude::{AppLooplessStateExt, ConditionSet},
    state::NextState,
};

use crate::{
    constants::{GROUND_LAYER, MAP_HEIGHT, TILE_SIZE},
    grid::snap_to_grid,
    screens::state::ScreenState,
};

pub struct SwitchLevelEvent {
    level_index: usize,
}

pub struct MapPlugin;

/// List of all collisions
#[derive(Deref, DerefMut)]
pub struct CollisionMap(pub Vec<IVec2>);

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .add_event::<SwitchLevelEvent>()
            .insert_resource(CollisionMap(Vec::new()))
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(ScreenState::Ingame)
                    .with_system(register_collision_int_cell)
                    .with_system(debug)
                    .with_system(switch_level)
                    .into(),
            )
            .add_enter_system(ScreenState::Ingame, setup);
    }
}

#[autodefault]
fn setup(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn_bundle(LdtkWorldBundle {
        transform: Transform {
            translation: Vec3::new(
                (-TILE_SIZE / 2) as f32,
                (-TILE_SIZE / 2) as f32,
                GROUND_LAYER as f32,
            ),
        },
        ldtk_handle: asset_server.load("maps/dungeon.ldtk"),
    });
}

fn register_collision_int_cell(
    mut collision_map: ResMut<CollisionMap>,
    query: Query<(&Transform, &IntGridCell), Added<IntGridCell>>,
) {
    for (transform, int_cell) in &query {
        // TODO magic number
        if int_cell.value == 1 {
            collision_map.push(snap_to_grid(&transform.translation.truncate()));
        }
    }
}

/// Converts ldtk coordinates to bevy coordinates
///
/// Ldtk uses down position, right positive whereas bevy uses up positive, right positive
pub fn ldtk_to_bevy(v: &IVec2) -> IVec2 {
    IVec2::new(v.x, (MAP_HEIGHT as i32) - v.y - 1)
}

fn switch_level(
    mut cmd: Commands,
    mut events: EventReader<SwitchLevelEvent>,
    mut collision_map: ResMut<CollisionMap>,
) {
    for SwitchLevelEvent { level_index } in events.iter() {
        cmd.insert_resource(NextState(ScreenState::Ingame));
        cmd.insert_resource(LevelSelection::Index(*level_index));
        collision_map.clear();
    }
}

fn debug(keys: Res<Input<KeyCode>>, mut writer: EventWriter<SwitchLevelEvent>) {
    if keys.just_pressed(KeyCode::Key1) {
        writer.send(SwitchLevelEvent { level_index: 0 });
    }
    if keys.just_pressed(KeyCode::Key2) {
        writer.send(SwitchLevelEvent { level_index: 1 });
    }
}
