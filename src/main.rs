mod cli;
mod commands;
mod config;

use clap::Parser;
use cli::{Cli, Commands};
use commands::add;
// use config::{get_app_config_dir, get_templates_dir};
use colored::*;

fn main() {
    let cli_args = Cli::parse();

    // Match on the subcommand provided
    let command_result = match cli_args.command {
        Commands::Add(add_args) => add::run(&add_args, cli_args.force),
        Commands::New => {
            println!("Executing 'new' command...");
            Ok(())
        }
        Commands::List => {
            println!("Executing 'list' command...");
            Ok(())
        }
        Commands::Remove => {
            println!("Executing 'remove' command...");
            Ok(())
        }
        Commands::Show => {
            println!("Executing 'show' command...");
            Ok(())
        }
        Commands::Edit => {
            println!("Executing 'edit' command...");
            Ok(())
        }
        Commands::Path => {
            println!("Executing 'path' command...");
            Ok(())
        }
    };

    if let Err(err) = command_result {
        eprintln!("\t{} {}", "✖ Error:".red().bold(), err.to_string().red());
        std::process::exit(1)
    }
}
