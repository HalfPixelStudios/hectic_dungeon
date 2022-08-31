use anyhow::anyhow;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreenState {
    MainMenu,
    Settings,
    LevelSelect,
    Ingame,
}

impl TryFrom<String> for ScreenState {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "mainmenu" => Ok(Self::MainMenu),
            "settings" => Ok(Self::Settings),
            "levelselect" => Ok(Self::LevelSelect),
            "ingame" => Ok(Self::Ingame),
            _ => Err(anyhow!("could not convert '{}' to ScreenState", value)),
        }
    }
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(ScreenState::MainMenu);
    }
}
