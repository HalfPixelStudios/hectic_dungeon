use std::collections::HashMap;

use bevy::prelude::*;

pub enum ZLayer {
    Ground = -1,
    Being = 50,
}

pub enum GridLayer {
    Wall,
    Player,
    Enemy,
    Spike,
}
