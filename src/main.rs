#![allow(unused)]
#![allow(dead_code)]
mod ability;
mod ai;
mod animation;
mod app;
mod assets;
mod attack;
mod camera;
mod constants;
mod enemy;
mod enviro;
mod game;
mod grid;
mod item;
mod layers;
mod map;
mod material;
mod movement;
mod player;
mod prefab;
mod room;
mod screens;
mod spritesheet_constants;
mod ui;
mod utils;
mod weapon;

fn main() {
    app::app();
}
