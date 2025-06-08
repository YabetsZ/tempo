use crate::cli::EditArgs;
use crate::config;
use crate::error::AppError;
use crate::utils;
use colored::*;
use edit;

/// Handles the `tempo edit` command.
pub fn run(args: &EditArgs) -> Result<(), AppError> {
    println!(
        "\n\t{} template {}...",
        "→ Opening".blue().bold(),
        args.template_name.cyan().bold()
    );

    let templates_dir = config::get_templates_dir()?;
    let template_file_path = utils::find_template_path(&templates_dir, &args.template_name)?;

    println!(
        "\t\t{} Editing file: {}",
        ">".magenta(),
        format!("{:?}", template_file_path).cyan()
    );

    match edit::edit_file(&template_file_path) {
        Ok(()) => {
            println!(
                "\n\t{} Finished editing template '{}'.",
                "✓".green().bold(),
                args.template_name.cyan()
            );
            Ok(())
        }
        Err(io_err) => {
            eprintln!(
                "{}",
                format!(
                    "\n\t{} Failed to open editor for {:?}.",
                    "✗".red().bold(),
                    template_file_path
                )
                .red()
            );
            eprintln!("{}", format!("\tDetails: {}", io_err).dimmed());

            Err(AppError::EditorFailed {
                path: template_file_path,
                source: io_err,
            })
        }
    }
}
