use clap::{ArgAction, Args, Parser, Subcommand};
use std::path::PathBuf;

/// `tempo`: The Code Templating Assistant
/// Quickly manage and use code templates for various purposes.
#[derive(Parser, Debug)]
#[command(author = "Yabets Zekaryas", name = "tempo", version = "0.1.0-alpha.1")]
#[command(about = "A Code Templating Assistant. Use `tempo <SUBCOMMAND> --help` for details.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, global = true, default_value_t = false)]
    pub force: bool,

    /// Enable verbose output
    #[arg(short, long, global = true, default_value_t = false, group = "verbosity", action = ArgAction::SetTrue)]
    pub verbose: bool,

    /// Suppress all output except for errors and essential data (like list/show output)
    #[arg(short, long, global = true, default_value_t = false, group = "verbosity", action = ArgAction::SetTrue)]
    pub quiet: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new template from a source file
    #[command(visible_alias = "a")]
    Add(AddArgs),

    /// Apply append, prepend or override operations onto the destination file
    Apply(ApplyArgs),

    /// List all available templates
    #[command(alias = "ls")]
    List,

    /// Remove a specified template
    #[command(alias = "rm")]
    Remove(RemoveArgs),
    /// Show the content of a specified template
    Show(ShowArgs),

    /// Edit a specified template in the default editor
    Edit(EditArgs),

    /// Print the full path to a specified template
    Path(PathArgs),
}

#[derive(Args, Debug)]
pub struct AddArgs {
    /// The name to assign to the new template
    pub name: String,
    /// The path to the source file to the template
    pub source_file_path: PathBuf,
}

/// Arguments for the `new` command
///
///  - If -o, -a, or -p is specified, --force might not have an additional effect for these actions.
///  - If none of -o, -a, -p are specified, AND destination exists:
///    - If --force is given: overwrite.
///    - If --force is NOT given: error or skip (we'll define this default behavior).
///  > **This means 'overwrite' can be triggered by -o OR by (--force AND no -a/-p).**
#[derive(Args, Debug)]
#[group(args(&["overwrite", "append", "prepend"]))]
pub struct ApplyArgs {
    /// Name of the template to use
    pub template_name: String,

    /// Path to the destination file to be created/modified
    pub destination_file_path: PathBuf,

    /// Overwrite the destination file if it exists
    #[arg(short = 'o', long, group = "write_strategy")]
    pub overwrite: bool,

    /// Append template content to the destination file if it exists
    #[arg(short = 'a', long, group = "write_strategy")]
    pub append: bool,

    /// Prepend template content to the destination file if it exists
    #[arg(short = 'p', long, group = "write_strategy")]
    pub prepend: bool,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// Name of the template to be deleted
    pub template_name: String,
}

/// Arguments for the `show` command
#[derive(Args, Debug)]
pub struct ShowArgs {
    /// Name of the template to show
    pub template_name: String,
}

/// Arguments for the `edit` command
#[derive(Args, Debug)]
pub struct EditArgs {
    /// Name of the template to edit
    pub template_name: String,
}

/// Arguments for the `path` command
#[derive(Args, Debug)]
pub struct PathArgs {
    /// Name of the template whose path is to be shown
    pub template_name: String,
}
