use crate::PollenError;
use seahorse::{App, Command, Context, Flag, FlagType};
use std::env;

use super::actions::*;
use super::commands;

/// Main CLI application entry point
pub fn run() -> Result<(), PollenError> {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("pollen [COMMAND] [OPTIONS]")
        .flag(
            Flag::new("config", FlagType::String)
                .description("Configuration file to use")
                .alias("c")
        )
        .flag(
            Flag::new("verbose", FlagType::Bool)
                .description("Enable verbose output")
                .alias("v")
        )
        .command(
            Command::new("init")
                .description("Initialize Pollen configuration directory")
                .usage("pollen init")
                .action(init_action)
        )
        .command(
            Command::new("parse")
                .description("Parse the configuration file and display entries")
                .usage("pollen parse [OPTIONS]")
                .action(parse_action)
        )
        .command(
            Command::new("list")
                .description("List all entries with their aliases")
                .usage("pollen list [OPTIONS]")
                .flag(
                    Flag::new("paths", FlagType::Bool)
                        .description("Show file paths instead of entry names")
                        .alias("p")
                )
                .action(list_action)
        )
        .command(
            Command::new("validate")
                .description("Validate the configuration file for errors")
                .usage("pollen validate [OPTIONS]")
                .action(validate_action)
        )
        .command(
            Command::new("gather")
                .description("Gather configuration files from the system into the files directory")
                .usage("pollen gather [ENTRY_NAMES...]")
                .action(gather_action)
        )
        .command(
            Command::new("scatter")
                .description("Scatter files from the files directory to their target locations")
                .usage("pollen scatter [ENTRY_NAMES...]")
                .action(scatter_action)
        )
        .command(
            Command::new("undo")
                .description("Undo the last gather or scatter operation")
                .usage("pollen undo")
                .action(undo_action)
        )
        .command(
            Command::new("git")
                .description("Git operations for the files directory")
                .usage("pollen git [SUBCOMMAND]")
                .action(git_action)
        )
        .command(
            Command::new("cd")
                .description("Show command to change to Pollen directory")
                .usage("pollen cd [SUBDIRECTORY]")
                .action(cd_action)
        )
        .command(
            Command::new("config")
                .description("Show current configuration and settings")
                .usage("pollen config")
                .action(config_action)
        )
        .action(default_action);

    app.run(args);
    Ok(())
}

fn default_action(c: &Context) {
    // If no command is specified, default to parse
    if let Err(e) = commands::parse::parse_config(c) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
