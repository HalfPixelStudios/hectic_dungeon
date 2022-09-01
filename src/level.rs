//! Holds game state logic related to the current level

use std::collections::HashSet;

use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, IntoConditionalSystem};

use crate::screens::state::ScreenState;

pub struct LevelCleared;
pub struct LevelFailed;

pub struct Level {
    players: HashSet<Entity>,
    enemies: HashSet<Entity>,
    /// Lock to prevent instantly winning or losing the level while entites are still being loaded
    game_end_lock: bool,
}

impl Level {
    pub fn new() -> Self {
        Level {
            players: HashSet::new(),
            enemies: HashSet::new(),
            game_end_lock: true,
        }
    }
    pub fn reset(&mut self) {
        self.players.clear();
        self.enemies.clear();
        self.game_end_lock = true;
    }
    pub fn remaining_players(&self) -> usize {
        self.players.len()
    }
    pub fn remaining_enemies(&self) -> usize {
        self.enemies.len()
    }
    pub fn register_player(&mut self, e: Entity) {
        self.players.insert(e);
    }
    pub fn register_enemy(&mut self, e: Entity) {
        self.enemies.insert(e);
    }
    pub fn deregister_player(&mut self, e: Entity) {
        self.players.remove(&e);
    }
    pub fn deregister_enemy(&mut self, e: Entity) {
        self.enemies.remove(&e);
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
