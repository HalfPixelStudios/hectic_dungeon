use bevy::prelude::*;

use std::collections::HashMap;


pub enum ZLayer {
    Ground = -1,
    Being = 50,
}

pub enum GridLayer{
    Wall,
    Player,
    Enemy,
    Spike
}
    
