use anyhow::Result;
use autodefault::autodefault;
use bevy::prelude::*;
use pino_argparse::{Cli, Command, Flag, FlagParse};

#[autodefault]
pub fn run_cli() -> Result<()> {
    let cli = Cli {
        program_name: "hectic_dungeon",
        root_command: Command {
            command_name: "run",
            handler: handle_run,
            flags: vec![],
        },
    };

    let args = std::env::args().collect();
    cli.run(&args);
    Ok(())
}

fn handle_run(flagparse: FlagParse) -> Result<(), Box<dyn std::error::Error>> {
    crate::app::app();
    Ok(())
}
