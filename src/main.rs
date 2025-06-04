mod cli;
mod config;

use clap::Parser;
use cli::{Cli, Commands};
use config::{get_app_config_dir, get_templates_dir};

fn main() {
    match get_app_config_dir() {
        Ok(path) => {
            println!("The config dir for our app is {path:?}.");
        }
        Err(err) => {
            eprintln!("Getting for config file is failed: {err}");
            return;
        }
    }

    match get_templates_dir() {
        Ok(path) => {
            println!("The template dir for our app is {path:?}.");
        }
        Err(err) => {
            eprintln!("Getting for template file is failed: {err}");
            return;
        }
    }

    let cli_args = Cli::parse();

    // Match on the subcommand provided
    match cli_args.command {
        Commands::Add => {
            println!("Executing 'add' command...");
            // Later, we'll call a function like: commands::add::run(args_for_add)
        }
        Commands::New => {
            println!("Executing 'new' command...");
        }
        Commands::List => {
            println!("Executing 'list' command...");
        }
        Commands::Remove => {
            println!("Executing 'remove' command...");
        }
        Commands::Show => {
            println!("Executing 'show' command...");
        }
        Commands::Edit => {
            println!("Executing 'edit' command...");
        }
        Commands::Path => {
            println!("Executing 'path' command...");
        }
    }
}
