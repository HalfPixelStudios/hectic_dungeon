#![allow(unused)]
#![allow(dead_code)]
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
mod layers;
mod map;
mod movement;
mod player;
mod prefab;
mod room;
mod ui;
mod utils;
mod weapon;

fn main() {
    app::app();
}
