//! Holds game state logic related to the current level

use bevy::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::screens::state::ScreenState;

pub struct LevelCleared;
pub struct LevelFailed;

pub struct Level {
    remaining_players: u32,
    remaining_enemies: u32,
    win_lock: bool,
    lose_lock: bool,
}

impl Level {
    pub fn new() -> Self {
        Level {
            remaining_players: 0,
            remaining_enemies: 0,
            win_lock: true,
            lose_lock: false,
        }
    }
    pub fn remaining_players(&self) -> u32 {
        self.remaining_players
    }
    pub fn remaining_enemies(&self) -> u32 {
        self.remaining_enemies
    }
    pub fn register_player(&mut self) {
        self.remaining_players += 1;
    }
    pub fn register_enemy(&mut self) {
        self.remaining_enemies += 1;
    }
    pub fn deregister_player(&mut self) {
        self.remaining_players -= 1;
    }
    pub fn deregister_enemy(&mut self) {
        self.remaining_enemies -= 1;
    }
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Level::new())
            .add_event::<LevelCleared>()
            .add_event::<LevelFailed>()
            .add_system(update.run_in_state(ScreenState::Ingame));
    }
}

fn update(
    mut room_state: ResMut<Level>,
    mut win_writer: EventWriter<LevelCleared>,
    mut lose_writer: EventWriter<LevelFailed>,
) {
    // TODO right now using stupid hack to prevent instantly winning/losing when no
    // players/enemies are loaded from the map yet.
    if room_state.is_changed() {
        if room_state.remaining_enemies() == 0 {
            if room_state.win_lock {
                room_state.win_lock = false;
            } else {
                println!("player win");
                win_writer.send(LevelCleared);
            }
        }
        if room_state.remaining_players() == 0 {
            if room_state.lose_lock {
                room_state.lose_lock = false;
            } else {
                println!("player lost");
                lose_writer.send(LevelFailed);
            }
        }
    }
}
