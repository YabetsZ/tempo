use crate::cli::RemoveArgs; // Ensure this matches your CLI definition
use crate::config;
use crate::error::AppError;
use colored::*;
use std::fs;
use std::io::{self, Write}; // For stdin/stdout interaction
use std::path::{Path, PathBuf};

fn find_template_path(templates_dir: &Path, template_name: &str) -> Result<PathBuf, AppError> {
    let entries = fs::read_dir(templates_dir).map_err(|e| AppError::ReadDir {
        source_path: templates_dir.to_path_buf(),
        source_error: e,
    })?;

    for entry_result in entries {
        let entry = entry_result.map_err(|e| AppError::ReadDir {
            source_path: templates_dir.to_path_buf(),
            source_error: e,
        })?;
        let path = entry.path();
        if path.is_file() {
            if let Some(stem) = path.file_stem() {
                if stem == template_name {
                    return Ok(path);
                }
            }
        }
    }
    Err(AppError::TemplateNotFound(template_name.to_string()))
}

/// Handles the `tempo remove` (or `tempo rm`) command.
pub fn run(args: &RemoveArgs, force: bool) -> Result<(), AppError> {
    println!(
        "\n\t{} template {}...",
        "→ Attempting to remove".yellow().bold(),
        args.template_name.cyan().bold()
    );

    let templates_dir = config::get_templates_dir()?;
    let template_file_path = find_template_path(&templates_dir, &args.template_name)?;

    println!(
        "\t\t{} Found template file: {:?}",
        ">".magenta(),
        template_file_path
    );

    if !force {
        print!(
            "\t❓ Are you sure you want to remove template '{}'? [y/N]: ",
            args.template_name.cyan()
        );
        io::stdout().flush()?; // Make sure the prompt is displayed before reading input

        let mut confirmation = String::new();
        io::stdin().read_line(&mut confirmation)?;

        if confirmation.trim().to_lowercase() != "y" {
            println!("\t{} Removal cancelled by user.", "✗".dimmed());
            return Ok(());
        }
    } else {
        println!(
            "\t\t{} {}",
            ">".magenta(),
            "Force flag active, skipping confirmation.".bright_yellow()
        );
    }

    fs::remove_file(&template_file_path).map_err(|e| AppError::FileRemove {
        path: template_file_path.clone(),
        source_error: e,
    })?;

    println!(
        "\n\t{} Template '{}' removed successfully.",
        "✓".green().bold(),
        args.template_name.cyan()
    );

    Ok(())
}
