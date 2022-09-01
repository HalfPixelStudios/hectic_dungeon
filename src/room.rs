use bevy::prelude::*;

pub struct LevelCleared;
pub struct LevelFailed;

pub struct Room {
    remaining_players: u32,
    remaining_enemies: u32,
}

impl Room {
    pub fn new() -> Self {
        Room {
            remaining_players: 0,
            remaining_enemies: 0,
        }
    }
    pub fn remaining_players(&self) -> u32 {
        self.remaining_players
    }
    pub fn remaining_enemies(&self) -> u32 {
        self.remaining_enemies
    }
    pub fn did_lose(&self) -> bool {
        self.remaining_players == 0
    }
    pub fn did_win(&self) -> bool {
        self.remaining_enemies == 0
    }
    pub fn register_player(&mut self) {
        self.remaining_players += 1;
    }
    pub fn register_enemy(&mut self) {
        self.remaining_enemies += 1;
    }
    pub fn deregister_player(&mut self) {
        self.remaining_players -= 1;
    }
    pub fn deregister_enemy(&mut self) {
        self.remaining_enemies -= 1;
    }
}

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Room::new())
            .add_event::<LevelCleared>()
            .add_event::<LevelFailed>()
            .add_system(update);
    }
}

fn update(
    room_state: Res<Room>,
    mut win_writer: EventWriter<LevelCleared>,
    mut lose_writer: EventWriter<LevelFailed>,
) {
    if room_state.is_changed() {
        println!(
            "players: {}, enemies: {}",
            room_state.remaining_players(),
            room_state.remaining_enemies()
        );
        if room_state.did_win() {
            println!("player win");
            win_writer.send(LevelCleared);
        }
        if room_state.did_lose() {
            println!("player lost");
            lose_writer.send(LevelFailed);
        }
    }
}
