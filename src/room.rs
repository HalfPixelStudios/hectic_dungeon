use bevy::prelude::*;

pub struct Room {
    pub remaining_enemies: u32,
}

impl Room {
    pub fn is_completed(&self) -> bool {
        self.remaining_enemies == 0
    }
}

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {}
}
