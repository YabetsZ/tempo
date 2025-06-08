mod cli;
mod commands;
mod config;
mod error;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};
use colored::*;
use commands::{add, apply, list, remove};

use crate::commands::{edit, path_cmd, show};

fn main() {
    let cli_args = Cli::parse();

    // Match on the subcommand provided
    let command_result = match cli_args.command {
        Commands::Add(add_args) => add::run(&add_args, cli_args.force),
        Commands::Apply(apply_args) => apply::run(&apply_args, cli_args.force),
        Commands::List => list::run(),
        Commands::Remove(remove_args) => remove::run(&remove_args, cli_args.force),
        Commands::Show(show_args) => show::run(&show_args),
        Commands::Edit(edit_args) => edit::run(&edit_args),
        Commands::Path(path_args) => path_cmd::run(&path_args),
    };

    if let Err(err) = command_result {
        eprintln!("\t{} {}", "✖ Error:".red().bold(), err.to_string().red());
        std::process::exit(1)
    }
}
