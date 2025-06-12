use crate::cli::AddArgs;
use crate::config;
use crate::error::AppError;
use colored::*;
use std::fs;
use crate::output::OutputConfig;

/// Handles the `tempo add` command.
///
/// # Arguments
/// * `args` - The parsed arguments from `clap` for the `add` command.
/// * `force` - Global force flag.
///
/// # Returns
/// * `Ok(())` if the template was added successfully.
/// * `Err(AppError)` if an error occurred.
pub fn run(args: &AddArgs, force: bool, output: &OutputConfig) -> Result<(), AppError> {
    output.info(
            format!("\t{} {} from {}...",
            "→ Adding template".blue().bold(),
            args.name.yellow().bold(),
            format!("{:?}", args.source_file_path).cyan()
        ));

    if force  {
        output.info(
            format!("\t\t{} {}",
            ">".magenta(),
            "Force flag is active.".bright_yellow().underline()
        ));
    }

    // > Validate source file path
    if !args.source_file_path.exists() {
        return Err(AppError::SourceFileDoesNotExist(
            args.source_file_path.clone(),
        ));
    }
    if !args.source_file_path.is_file() {
        return Err(AppError::SourcePathIsNotAFile(
            args.source_file_path.clone(),
        ));
    }

    // > Validate template name (basic validation for now)
    //    TODO: A more robust validation might involve regex or checking for reserved names.
    if args.name.contains('/') || args.name.contains('\\') {
        return Err(AppError::TemplateNameInvalid(
            args.name.clone(),
            "Name cannot contain path separators".to_string(),
        ));
    }
    if args.name.trim().is_empty() {
        return Err(AppError::TemplateNameInvalid(
            args.name.clone(),
            "Name cannot be empty".to_string(),
        ));
    }

    // > Get the templates storage directory
    let templates_dir = config::get_templates_dir()?;

    // > Construct the destination path
    //    We want to store it as `<name>.<original_extension>`
    let original_extension = args
        .source_file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or(""); // Fallback to empty string if no extension

    let mut dest_filename = args.name.clone();
    if !original_extension.is_empty() {
        dest_filename.push('.');
        dest_filename.push_str(original_extension);
    }

    let dest_path = templates_dir.join(dest_filename.clone()); // Use `dest_filename` here as it's now owned

    output.info(
        format!(
            "\t\t{} {} {}",
            ">".magenta(),
            "Target path:".blue(),
            format!("{:?}", dest_path).cyan().bold()
        ));

    // > Check if template already exists (unless --force is used)
    if dest_path.exists() && !force {
        return Err(AppError::TemplateAlreadyExists(args.name.clone()));
    }

    // > Copy the file
    //    `fs::copy` overwrites if the destination exists. The check above handles the `--force` logic.
    match fs::copy(&args.source_file_path, &dest_path) {
        Ok(bytes_copied) => {
            output.success(
                format!("\t{} '{}' ({} bytes → {})",
                "✓ Successfully added".bold(),
                args.name.yellow().bold(),
                bytes_copied,
                format!("{:?}", dest_path).cyan()
            ));
            Ok(())
        }
        Err(e) => {
            // Attempt to remove partially written file if copy failed, though fs::copy is usually atomic or fails early.
            if dest_path.exists() {
                let _ = fs::remove_file(&dest_path); // Ignore error on cleanup
            }
            Err(AppError::Io(e))
        }
    }
}
