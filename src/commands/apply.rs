use crate::cli::ApplyArgs;
use crate::commands::error::AppError;
use crate::config;
use colored::*;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

// Helper function to find a template file by its base name
// Returns the full path to the template file if found.
fn find_template_path(templates_dir: &Path, template_name: &str) -> Result<PathBuf, AppError> {
    let entries = fs::read_dir(templates_dir).map_err(|e| AppError::ReadDir {
        source_path: templates_dir.to_path_buf(),
        source_error: e,
    })?;

    for entry_result in entries {
        let entry = entry_result.map_err(|e| AppError::ReadDir {
            source_path: templates_dir.to_path_buf(), // Or be more specific about entry path if possible
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

/// Handles the `tempo new` command.
pub fn run(args: &ApplyArgs, force: bool) -> Result<(), AppError> {
    println!(
        "\n\t{} template {} to {}...",
        "→ Applying".blue().bold(),
        args.template_name.yellow().bold(),
        format!("{:?}", args.destination_file_path).cyan()
    );

    // 1. Find the template
    let templates_dir = config::get_templates_dir()?;
    let template_file_path = find_template_path(&templates_dir, &args.template_name)?;
    println!(
        "\t\t{} Using template file: {}",
        ">".magenta(),
        format!("{template_file_path:?}").cyan()
    );

    // 2. Read template content
    let template_content = fs::read_to_string(&template_file_path).map_err(|e| AppError::Io(e))?;
    // Consider adding a specific AppError variant for template read failure if needed

    let dest_path = &args.destination_file_path;

    // 3. Handle destination file
    if dest_path.exists() {
        if dest_path.is_dir() {
            return Err(AppError::DestinationIsDirectory {
                action: "apply template".to_string(),
                dest: dest_path.to_path_buf(),
            });
        }

        // Destination file exists, apply strategy
        if args.overwrite {
            println!("\t\t{} Overwriting existing file.", ">".magenta());
            fs::write(dest_path, &template_content).map_err(|e| AppError::Io(e))?;
        } else if args.append {
            println!("\t\t{} Appending to existing file.", ">".magenta());
            let mut file = OpenOptions::new().append(true).open(dest_path)?; // AppError::Io handles error
            file.write_all(template_content.as_bytes())?;
        } else if args.prepend {
            println!("\t\t{} Prepending to existing file.", ">".magenta());
            let mut original_content = String::new();
            File::open(dest_path)?.read_to_string(&mut original_content)?;

            let new_content = format!("{}\n{}", template_content, original_content);
            fs::write(dest_path, new_content)?;
        } else if force {
            // No specific strategy flag, but --force is active
            println!(
                "\t\t{} Overwriting existing file (due to --force).",
                ">".magenta()
            );
            fs::write(dest_path, &template_content)?;
        } else {
            // No strategy flag, no --force, and file exists
            return Err(AppError::DestinationFileExists(dest_path.to_path_buf()));
        }
    } else {
        // Destination file does not exist, create it
        println!("\t\t{} Creating new file.", ">".magenta());
        if let Some(parent_dir) = dest_path.parent() {
            if !parent_dir.exists() {
                fs::create_dir_all(parent_dir)?; // AppError::Io handles error
                println!(
                    "\t\t{} Created parent directory: {:?}",
                    ">".magenta(),
                    parent_dir
                );
            }
        }
        fs::write(dest_path, &template_content)?;
    }

    println!(
        "\n\t{} Template '{}' applied to {}",
        "✓ Successfully".green().bold(),
        args.template_name.yellow(),
        format!("{:?}", dest_path).cyan()
    );

    Ok(())
}
