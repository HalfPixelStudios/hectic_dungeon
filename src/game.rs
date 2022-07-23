use bevy::core::Timer;

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


