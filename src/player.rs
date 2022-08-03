use std::time::Duration;

use bevy::prelude::*;
use bevy_bobs::component::health::*;
use leafwing_input_manager::prelude::*;

use crate::{
    animation::Animation,
    assets::{BeingPrefab, PrefabData, SpriteSheets},
    attack::AttackPattern,
    camera::CameraFollow,
    grid::{to_world_coords, CellType, Grid, GridEntity},
    movement::Movement,
    ui::attack_indicator::AttackIndicator,
    utils::Dir,
};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub enum PlayerState {
    Move,
    Attack,
}

//TODO add direction vectors to PlayerAction definition
#[derive(Actionlike, Clone)]
pub enum PlayerAction {
    Left,
    Right,
    Up,
    Down,
    Attack,
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
            .add_system(move_controller)
            .add_system(attack_controller)
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
            (KeyCode::Space, PlayerAction::Attack),
        ]);

        // let handle = prefab_data.get("player").unwrap();
        // let player = beings.get(handle).unwrap();

        let id = cmd.spawn().id();

        cmd.entity(id)
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
            .insert(GridEntity::new(*spawn_pos, CellType::Player(id)))
            .insert(Health::new(10))
            .insert_bundle(InputManagerBundle::<PlayerAction> {
                action_state: ActionState::default(),
                input_map,
            })
            .insert(CameraFollow)
            .insert(AttackIndicator {
                dir: Dir::North,
                pattern: AttackPattern::Hammer,
                hidden: true,
            })
            .insert(Children::default())
            .insert(Movement::new());
    }
}

//TODO check collision with tiled map
fn move_controller(
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

fn attack_controller(
    mut cmd: Commands,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut AttackIndicator), With<Player>>,
) {
    if let Ok((entity, mut attack_indicator)) = query.get_single_mut() {
        if keys.just_pressed(KeyCode::Up) {
            attack_indicator.dir = Dir::North;
        }
        if keys.just_pressed(KeyCode::Left) {
            attack_indicator.dir = Dir::West;
        }
        if keys.just_pressed(KeyCode::Down) {
            attack_indicator.dir = Dir::South;
        }
        if keys.just_pressed(KeyCode::Right) {
            attack_indicator.dir = Dir::East;
        }
        if keys.just_pressed(KeyCode::Space) {
            attack_indicator.hidden = !attack_indicator.hidden;
        }
    }
}
