use std::time::Duration;

use bevy::prelude::*;

use crate::{assets::{SpriteSheets, PrefabData, BeingPrefab}, animation::Animation, grid::GridPosition, movement::Movement};
use leafwing_input_manager::prelude::*;

#[derive(Component)]
pub struct Player;


#[derive(Actionlike, Clone)]
pub enum PlayerAction {
    Left,
    Right,
    Up,
    Down,
}


pub struct SpawnPlayerEvent {
    pub spawn_pos: Vec2,
}

fn spawn(
    mut cmd: Commands,
    mut events: EventReader<SpawnPlayerEvent>,
    asset_sheet: Res<SpriteSheets>,
    prefab_data: Res<PrefabData>,
    beings: Res<Assets<BeingPrefab>>
) {
    for SpawnPlayerEvent { spawn_pos } in events.iter() {
        let input_map = InputMap::new([
            (KeyCode::Left, PlayerAction::Left),
            (KeyCode::A, PlayerAction::Left),
            (KeyCode::Right, PlayerAction::Right),
            (KeyCode::D, PlayerAction::Right),
            (KeyCode::Up, PlayerAction::Up),
            (KeyCode::W, PlayerAction::Up),
            (KeyCode::Down, PlayerAction::Down),
            (KeyCode::S, PlayerAction::Down),
        ]);


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
            .insert(Animation::new(&player.anim))
            .insert(Player)
            .insert(GridPosition(IVec2::ZERO))
            .insert_bundle(InputManagerBundle::<PlayerAction> {
                action_state: ActionState::default(),
                input_map,
            })
            .insert(Movement{
                timer: Timer::from_seconds(0.9, true),
                next_move: IVec2::ZERO
            });

            

    }
}



//TODO check collision with tiled map
fn controller(mut cmd: Commands, mut query: Query<(&mut GridPosition, &mut Movement, &ActionState<PlayerAction>), With<Player>>) {
    
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

        if dir != IVec2::ZERO {
            info!("{}", dir);
        }
        if movement.next_move==IVec2::ZERO{
            movement.next_move=dir;
        }

    }
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_event::<SpawnPlayerEvent>()
            .add_system(controller)
            .add_system(spawn);
    }
}

