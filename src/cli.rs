use clap::{Parser, Subcommand};

/// `tempo`: The Code Templating Assistant
/// Quickly manage and use code templates for various purposes.
#[derive(Parser, Debug)]
#[command(name = "tempo", version = "0.1.0")]
#[command(about = "A Code Templating Assistant. Use `tempo <SUBCOMMAND> --help` for details.", long_about = None)]
pub struct Cli {
    /// The subcommand to execute
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new template from a source file
    Add, // We'll add arguments to this later (e.g., name, source_file_path)

    /// Create a new file from an existing template
    New, // We'll add arguments to this later (e.g., template_name, destination_file_path)

    /// List all available templates
    List,

    /// Remove a specified template
    Remove, // We'll add arguments (e.g., template_name)

    /// Show the content of a specified template
    Show, // We'll add arguments (e.g., template_name)

    /// Edit a specified template in the default editor
    Edit, // We'll add arguments (e.g., template_name)

    /// Print the full path to a specified template
    Path, // We'll add arguments (e.g., template_name)
          // We can add `Init` later if we decide it's needed
}
