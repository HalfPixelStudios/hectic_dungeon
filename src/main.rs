#![allow(unused)]
#![allow(dead_code)]
mod ability;
mod ai;
mod animation;
mod app;
mod assets;
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
mod layers;
mod level;
mod map;
mod material;
mod movement;
mod player;
mod prefab;
mod screens;
mod spritesheet_constants;
mod ui;
mod utils;
mod weapon;

fn main() {
    cli::run_cli().unwrap();
}
