#![allow(unused)]
#![warn(unused_imports)]
#![allow(dead_code)]
mod ability;
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

fn main() {
    cli::run_cli().unwrap();
}
