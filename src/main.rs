// #![allow(unused)]
// #![warn(unused_imports)]
#![allow(dead_code)]
mod ai;
mod app;
mod attack;
mod buffs;
mod camera;
mod cli;
mod constants;
mod enemy;
mod enviro;
mod game;
mod grid;
mod item;
mod level;
mod map;
mod material;
mod movement;
mod player;
mod screens;
mod spritesheet;
mod ui;
mod utils;
mod weapon;

pub mod prelude {
    pub use crate::{
        attack::{AttackEvent, AttackPattern},
        constants::*,
        enemy::{DamageEnemyEvent, DropTable, Enemy, SpawnEnemyEvent},
        game::{GameState, PauseGame},
        grid::{snap_to_grid, to_world_coords, CellType, Grid, GridEntity},
        level::{Level, LevelCleared, LevelFailed},
        map::{ldtk_to_bevy, CollisionMap, CurrentLevel, SwitchLevelEvent},
        movement::Movement,
        player::{DamagePlayerEvent, Player, PlayerMovedEvent, SelectedPlayer, SpawnPlayerEvent},
        screens::state::ScreenState,
        spritesheet::*,
        utils::*,
        weapon::{CurrentWeapon, Damage, WeaponPrefab},
    };
}

fn main() {
    cli::run_cli().unwrap();
}
