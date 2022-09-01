//! Holds game state logic related to the current level

use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, IntoConditionalSystem};

use crate::screens::state::ScreenState;

pub struct LevelCleared;
pub struct LevelFailed;

pub struct Level {
    remaining_players: u32,
    remaining_enemies: u32,
    /// Lock to prevent instantly winning or losing the level while entites are still being loaded
    game_end_lock: bool,
}

impl Level {
    pub fn new() -> Self {
        Level {
            remaining_players: 0,
            remaining_enemies: 0,
            game_end_lock: true,
        }
    }
    pub fn reset(&mut self) {
        println!("level reset");
        self.remaining_players = 0;
        self.remaining_enemies = 0;
        self.game_end_lock = true;
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
        // careful of underflow
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
            .add_system(update.run_in_state(ScreenState::Ingame))
            .add_enter_system(ScreenState::Ingame, reset);
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
        println!(
            "players: {}, enemies: {}",
            room_state.remaining_players(),
            room_state.remaining_enemies()
        );

        if room_state.remaining_players() != 0 && room_state.remaining_enemies() != 0 {
            room_state.game_end_lock = false;
        }

        if !room_state.game_end_lock {
            if room_state.remaining_enemies() == 0 {
                println!("player win");
                win_writer.send(LevelCleared);
            }
            if room_state.remaining_players() == 0 {
                println!("player lost");
                lose_writer.send(LevelFailed);
            }
        }
    }
}

fn reset(mut room_state: ResMut<Level>) {
    room_state.reset();
}
