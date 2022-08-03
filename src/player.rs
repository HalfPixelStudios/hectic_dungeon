use std::time::Duration;

use bevy::prelude::*;
use bevy_bobs::component::health::*;
use leafwing_input_manager::prelude::*;

use crate::{
    animation::Animation,
    assets::{BeingPrefab, PrefabData, SpriteSheets},
    camera::CameraFollow,
    grid::{to_world_coords, CellType, Grid, GridEntity},
    movement::Movement,
};

#[derive(Component)]
pub struct Player;

//TODO add direction vectors to PlayerAction definition
#[derive(Actionlike, Clone)]
pub enum PlayerAction {
    Left,
    Right,
    Up,
    Down,
}
pub struct PlayerMovedEvent;

pub struct SpawnPlayerEvent {
    pub spawn_pos: IVec2,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_event::<SpawnPlayerEvent>()
            .add_event::<PlayerMovedEvent>()
            .add_system(controller)
            .add_system(spawn);
    }
}

fn spawn(
    mut cmd: Commands,
    mut events: EventReader<SpawnPlayerEvent>,
    asset_sheet: Res<SpriteSheets>,
    prefab_data: Res<PrefabData>,
    beings: Res<Assets<BeingPrefab>>,
) {
    for SpawnPlayerEvent { spawn_pos } in events.iter() {
        let input_map = InputMap::new([
            // (KeyCode::Left, PlayerAction::Left),
            (KeyCode::A, PlayerAction::Left),
            // (KeyCode::Right, PlayerAction::Right),
            (KeyCode::D, PlayerAction::Right),
            // (KeyCode::Up, PlayerAction::Up),
            (KeyCode::W, PlayerAction::Up),
            // (KeyCode::Down, PlayerAction::Down),
            (KeyCode::S, PlayerAction::Down),
        ]);

        // let handle = prefab_data.get("player").unwrap();
        // let player = beings.get(handle).unwrap();

        cmd.spawn()
            .insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..default()
                },
                texture_atlas: asset_sheet.get("player").unwrap().clone(),
                transform: Transform {
                    translation: to_world_coords(spawn_pos).extend(1.),
                    ..default()
                },
                ..default()
            })
            .insert(Player)
            .insert(Health::new(10))
            .insert(GridEntity::new(*spawn_pos, CellType::Player))
            .insert_bundle(InputManagerBundle::<PlayerAction> {
                action_state: ActionState::default(),
                input_map,
            })
            .insert(CameraFollow)
            .insert(Movement::new());
    }
}

//TODO check collision with tiled map
fn controller(
    mut cmd: Commands,
    mut query: Query<(&mut GridEntity, &mut Movement, &ActionState<PlayerAction>), With<Player>>,
    mut player_moved: EventWriter<PlayerMovedEvent>,
    grid: Res<Grid<CellType>>,
) {
    if let Ok((mut grid_position, mut movement, action_state)) = query.get_single_mut() {
        let mut dir = IVec2::ZERO;

        if action_state.just_pressed(PlayerAction::Left) {
            dir += IVec2::new(-1, 0);
        }
        if action_state.just_pressed(PlayerAction::Right) {
            dir += IVec2::new(1, 0);
        }
        if action_state.just_pressed(PlayerAction::Up) {
            dir += IVec2::new(0, 1);
        }
        if action_state.just_pressed(PlayerAction::Down) {
            dir += IVec2::new(0, -1);
        }

        if movement.next_move == IVec2::ZERO {
            let next_pos = grid_position.pos + dir;
            if dir != IVec2::ZERO
                && grid.bounds_check(&next_pos)
                && !grid.contains_at(&next_pos, CellType::Wall).unwrap()
            {
                player_moved.send(PlayerMovedEvent);
                info!("player move {}", dir);
                movement.next_move = dir;
            }
        }
    }
}
