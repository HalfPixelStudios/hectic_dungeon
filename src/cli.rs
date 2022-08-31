use anyhow::Result;
use autodefault::autodefault;
use bevy::prelude::*;
use pino_argparse::{Cli, Command, Flag, FlagParse};

use crate::{app::AppConfig, screens::state::ScreenState};

#[autodefault]
pub fn run_cli() -> Result<()> {
    let cli = Cli {
        program_name: "hectic_dungeon",
        root_command: Command {
            command_name: "run",
            handler: handle_run,
            flags: vec![
                Flag::new("appstate")
                    .short('s')
                    .desc("which app state to start in (mainmenu|ingame)")
                    .parameter(),
                Flag::new("fullscreen").short('f'),
                Flag::new("egui").short('e'),
            ],
        },
    };

    let args = std::env::args().collect();
    cli.run(&args);
    Ok(())
}

fn handle_run(flagparse: FlagParse) -> Result<(), Box<dyn std::error::Error>> {
    let start_state = flagparse
        .get_flag_value::<String>("appstate")
        .and_then(|f| ScreenState::try_from(f).ok())
        .unwrap_or_else(|| {
            warn!("invalid appstate, defaulting to 'ingame'");
            ScreenState::Ingame
        });

    let fullscreen = flagparse.get_flag("fullscreen");
    let egui_enabled = flagparse.get_flag("egui");

    let config = AppConfig {
        fullscreen,
        egui_enabled,
        start_state,
    };

    crate::app::app(config);
    Ok(())
}
