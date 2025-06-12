use crate::cli::RemoveArgs;
use crate::config;
use crate::error::AppError;
use crate::output::OutputConfig;
use crate::utils;
use colored::*;
use std::fs;
use std::io::{self, Write};

/// Handles the `tempo remove` (or `tempo rm`) command.
pub fn run(args: &RemoveArgs, force: bool, output: &OutputConfig) -> Result<(), AppError> {
    output.info(format!(
        "\n\t{} template {}...",
        "→ Attempting to remove".yellow().bold(),
        args.template_name.cyan().bold()
    ));

    let templates_dir = config::get_templates_dir()?;
    output.verbose(format!(
        "[VERBOSE] Searching for template to remove in: {:?}",
        templates_dir
    ));
    let template_file_path = utils::find_template_path(&templates_dir, &args.template_name)?;

    output.info(format!(
        "\t\t{} Found template file: {:?}",
        ">".magenta(),
        template_file_path
    ));

    if !force {
        if !output.quiet {
            print!(
                "\t❓ Are you sure you want to remove template '{}'? [y/N]: ",
                args.template_name.cyan()
            );
            io::stdout().flush()?;
            let mut confirmation = String::new();
            io::stdin().read_line(&mut confirmation)?;
            if confirmation.trim().to_lowercase() != "y" {
                output.info(format!("\t{} Removal cancelled by user.", "✗".dimmed())); // Use output.info
                return Ok(());
            }
        } else {
            output.verbose("[VERBOSE] Quiet mode: cancellation assumed as no interactive confirmation possible.");
            return Ok(());
        }
    } else {
        output.info(format!(
            // Use output.info
            "\t\t{} {}",
            ">".magenta(),
            "Force flag active, skipping confirmation.".bright_yellow()
        ));
    }

    fs::remove_file(&template_file_path).map_err(|e| AppError::FileRemove {
        path: template_file_path.clone(),
        source_error: e,
    })?;

    output.success(
        format!("\n\t{} Template '{}' removed successfully.",
        "✓".green().bold(),
        args.template_name.cyan()
    ));

    Ok(())
}
