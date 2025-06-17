use crate::cli::AddArgs;
use crate::config;
use crate::error::AppError;
use colored::*;
use std::fs;
use crate::manifest::TemplateEntry;
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

    let mut manifest = config::load_manifest()?;
    output.verbose(format!("\t\t[VERBOSE] Manifest loaded. {} templates.", manifest.templates.len()));

    if manifest.get_template(&args.name).is_some() && !force {
        return Err(AppError::TemplateAlreadyExists(args.name.clone()));
    }
    
    // > Get the templates storage directory
    let templates_dir = config::get_templates_dir()?;

    // > Construct the destination path
    //    We want to store it as `<name>.<original_extension>`
    let original_extension = args
        .source_file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")// Fallback to empty string if no extension
        .to_lowercase(); 

    let mut dest_filename = args.name.clone();
    if !original_extension.is_empty() {
        dest_filename.push('.');
        dest_filename.push_str(&original_extension);
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

    if force && manifest.get_template(&args.name).is_some() {
        if let Some(existing_entry) = manifest.get_template(&args.name) {
            let old_file_path = templates_dir.join(&existing_entry.filename_in_storage);
            if old_file_path.exists() && old_file_path != dest_path { 
                output.verbose(format!("\t\t[VERBOSE] Removing old file due to overwrite: {:?}", old_file_path));
                fs::remove_file(&old_file_path).map_err(|e| AppError::FileRemove {
                    path: old_file_path,
                    source_error: e,
                })?;
            }
        }
    }

    fs::copy(&args.source_file_path, &dest_path).map_err(|e| AppError::FileCopy {
        from: args.source_file_path.clone(),
        to: dest_path.clone(),
        source_error: e,
    })?;

    // --- Create and add TemplateEntry to Manifest ---
    let mut new_entry = TemplateEntry::new(dest_filename.clone(), original_extension.clone());
    new_entry.original_source_path = Some(args.source_file_path.clone().canonicalize().unwrap_or_else(|_| args.source_file_path.clone()));
    // TODO: any other metadata like description, tags could be set here 

    manifest.add_template(args.name.clone(), new_entry);
    output.verbose(format!("\t\t[VERBOSE] Template entry for '{}' added/updated in manifest.", args.name));
    
    // --- Save Manifest ---
    config::save_manifest(&manifest)?;
    output.verbose(format!("\t\t[VERBOSE] Manifest saved. Total templates: {}.", manifest.templates.len()));

    output.success(format!(
        "\t{} Template '{}' added successfully.",
        "✓".green().bold(),
        args.name.yellow().bold()
    ));

    Ok(())
}
