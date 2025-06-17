use crate::cli::RemoveArgs;
use crate::config;
use crate::error::AppError;
use crate::output::OutputConfig;
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

    let mut manifest = config::load_manifest()?;
    output.verbose(format!("\t\t[VERBOSE] Manifest loaded. Contains {} templates before removal.", manifest.templates.len()));

    let template_entry_to_remove = match manifest.get_template(&args.template_name) {
        Some(entry) => entry.clone(), // Clone to avoid borrowing issues with manifest later
        None => return Err(AppError::TemplateNotFound(args.template_name.clone())),
    };

    let filename_in_storage = template_entry_to_remove.filename_in_storage;
    let templates_dir = config::get_templates_dir()?;
    let template_file_path = templates_dir.join(&filename_in_storage);

    output.verbose(format!(
        "\t\t[VERBOSE] Template '{}' corresponds to file: {:?}",
        args.template_name, template_file_path
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
            return Err(AppError::ConfirmationNeededInQuietMode{
                action: "remove".to_string(),
                template_name: args.template_name.clone()
            });
        }
    } else {
        output.info(format!(
            // Use output.info
            "\t\t{} {}",
            ">".magenta(),
            "Force flag active, skipping confirmation.".bright_yellow()
        ));
    }

    // Delete the actual template file
    if template_file_path.exists() {
        fs::remove_file(&template_file_path).map_err(|e| AppError::FileRemove {
            path: template_file_path.clone(),
            source_error: e,
        })?;
        output.verbose(format!("\t\t[VERBOSE] Deleted file: {:?}", template_file_path));
    } else {
        output.warn(format!(
            "\t\tWarning: File '{}' not found in storage, but removing from manifest.",
            filename_in_storage
        ));
    }

    // Remove from the manifest object
    manifest.remove_template(&args.template_name);
    output.verbose(format!("\t\t[VERBOSE] Entry for '{}' removed from manifest object.", args.template_name));

    // Save the updated manifest
    config::save_manifest(&manifest)?;
    output.verbose(format!("\t\t[VERBOSE] Manifest saved. Total templates: {}.", manifest.templates.len()));


    output.success(
        format!("\n\t{} Template '{}' removed successfully.",
        "✓".green().bold(),
        args.template_name.cyan()
    ));

    Ok(())
}
