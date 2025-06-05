use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// `tempo`: The Code Templating Assistant
/// Quickly manage and use code templates for various purposes.
#[derive(Parser, Debug)]
#[command(name = "tempo", version = "0.1.0")]
#[command(about = "A Code Templating Assistant. Use `tempo <SUBCOMMAND> --help` for details.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, global = true, default_value_t = false)]
    pub force: bool,
    // TODO: We can add --verbose and --quiet later as global options
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new template from a source file
    #[command(visible_alias = "a")]
    Add(AddArgs),

    /// Create a new file from an existing template
    New, // TODO: We'll add arguments to this later (e.g., template_name, destination_file_path)

    /// List all available templates
    List,

    /// Remove a specified template
    Remove, // TODO: We'll add arguments (e.g., template_name)

    /// Show the content of a specified template
    Show, // TODO: We'll add arguments (e.g., template_name)

    /// Edit a specified template in the default editor
    Edit, // TODO: We'll add arguments (e.g., template_name)

    /// Print the full path to a specified template
    Path, // TODO: We'll add arguments (e.g., template_name)
          // TODO: We can add `Init` later if we decide it's needed
}

#[derive(Args, Debug)]
pub struct AddArgs {
    /// The name to assign to the new template
    pub name: String,
    /// The path to the source file to the template
    pub source_file_path: PathBuf,
}
