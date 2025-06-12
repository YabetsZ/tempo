use crate::cli::ApplyArgs;
use crate::config;
use crate::error::AppError;
use crate::utils;
use colored::*;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use crate::output::OutputConfig;

/// Handles the `tempo new` command.
pub fn run(args: &ApplyArgs, force: bool, output: &OutputConfig) -> Result<(), AppError> {
    output.info(
        format!("\n\t{} template {} to {}...",
        "→ Applying".blue().bold(),
        args.template_name.yellow().bold(),
        format!("{:?}", args.destination_file_path).cyan()
    ));

    // 1. Find the template
    let templates_dir = config::get_templates_dir()?;
    let template_file_path = utils::find_template_path(&templates_dir, &args.template_name)?;
    output.verbose(
        format!("\t\t{} Using template file: {}",
                "[VERBOSE]".magenta(),
        format!("{template_file_path:?}").cyan()
    ));

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
            output.info(format!("\t\t{} Overwriting existing file.", ">".magenta()));
            fs::write(dest_path, &template_content).map_err(|e| AppError::Io(e))?;
        } else if args.append {
            output.info(format!("\t\t{} Appending to existing file.", ">".magenta()));
            let mut file = OpenOptions::new().append(true).open(dest_path)?; // AppError::Io handles error
            file.write_all(template_content.as_bytes())?;
        } else if args.prepend {
            output.info(format!("\t\t{} Prepending to existing file.", ">".magenta()));
            let mut original_content = String::new();
            File::open(dest_path)?.read_to_string(&mut original_content)?;

            let new_content = format!("{}\n{}", template_content, original_content);
            fs::write(dest_path, new_content)?;
        } else if force {
            // No specific strategy flag, but --force is active
            output.info(
                format!("\t\t{} Overwriting existing file (due to --force).",
                ">".magenta()
            ));
            fs::write(dest_path, &template_content)?;
        } else {
            // No strategy flag, no --force, and file exists
            return Err(AppError::DestinationFileExists(dest_path.to_path_buf()));
        }
    } else {
        // Destination file does not exist, create it
        output.info(format!("\t\t{} Creating new file.", ">".magenta()));
        if let Some(parent_dir) = dest_path.parent() {
            if !parent_dir.exists() {
                fs::create_dir_all(parent_dir)?; // AppError::Io handles error
                output.info(
                    format!("\t\t{} Created parent directory: {:?}",
                    ">".magenta(),
                    parent_dir
                ));
            }
        }
        fs::write(dest_path, &template_content)?;
    }

    output.success(
        format!("\n\t{} Template '{}' applied to {}",
        "✓ Successfully".green().bold(),
        args.template_name.yellow(),
        format!("{:?}", dest_path).cyan()
    ));

    Ok(())
}
