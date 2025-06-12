mod cli;
mod commands;
mod config;
mod error;
mod utils;
mod output;

use clap::Parser;
use cli::{Cli, Commands};
use colored::*;
use commands::{add, apply, list, remove};

use crate::commands::{edit, path_cmd, show};
use crate::output::OutputConfig;

fn main() {
    let cli_args = Cli::parse();
    let force_flag = cli_args.force;
    let output = OutputConfig::new(cli_args.verbose, cli_args.quiet);

    let command_result = match cli_args.command {
        Commands::Add(add_args) => add::run(&add_args, force_flag, &output),
        Commands::Apply(apply_args) => apply::run(&apply_args, force_flag, &output),
        Commands::List => list::run(&output),
        Commands::Remove(remove_args) => remove::run(&remove_args, force_flag, &output),
        Commands::Show(show_args) => show::run(&show_args, &output),
        Commands::Edit(edit_args) => edit::run(&edit_args, &output),
        Commands::Path(path_args) => path_cmd::run(&path_args, &output),
    };

    if let Err(err) = command_result {
        eprintln!("\t{} {}", "✖ Error:".red().bold(), err.to_string().red());
        std::process::exit(1)
    }
}
