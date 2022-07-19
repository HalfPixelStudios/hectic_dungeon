use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct SpawnPlayerEvent;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct GridPosition(pub IVec2);

#[derive(Actionlike, Clone)]
pub enum PlayerAction {
    Left,
    Right,
    Up,
    Down,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_system(spawn_player)
            .add_system(player_controller);
    }
}

fn spawn_player(mut cmd: Commands) {
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

    let e = cmd.spawn().id();

    cmd.entity(e)
        .insert(Player)
        .insert(GridPosition(IVec2::ZERO))
        .insert_bundle(InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map,
        });
}

fn player_controller(mut cmd: Commands, query: Query<&ActionState<PlayerAction>, With<Player>>) {
    let action_state = query.single();
    let mut dir = IVec2::ZERO;

    if action_state.just_pressed(PlayerAction::Left) {
        dir += IVec2::new(-1, 0);
    }
    if action_state.just_pressed(PlayerAction::Right) {
        dir += IVec2::new(1, 0);
    }
    if action_state.just_pressed(PlayerAction::Up) {
        dir += IVec2::new(0, -1);
    }
    if action_state.just_pressed(PlayerAction::Left) {
        dir += IVec2::new(0, 1);
    }
}
