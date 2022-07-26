use bevy::prelude::*;
use crate::{player::PlayerMovedEvent, enemy::EnemyUpdateEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState{
    Menu,
    PlayerPhase,
    EnemyPhase,
    AllPhase
}
pub struct GameData{
    timer: Timer,
}

pub fn game_loop(mut player_moved: EventReader<PlayerMovedEvent>, mut enemy_update: EventWriter<EnemyUpdateEvent>){
    for PlayerMovedEvent in player_moved.iter(){
        enemy_update.send(EnemyUpdateEvent);
    }

}

pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(game_loop);

        
    }
}
