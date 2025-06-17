use chrono::Utc;
use crate::cli::EditArgs;
use crate::config;
use crate::error::AppError;
use colored::*;
use edit;
use crate::output::OutputConfig;

/// Handles the `tempo edit` command.
pub fn run(args: &EditArgs, output: &OutputConfig) -> Result<(), AppError> {
    output.info(
        format!("\n\t{} template {}...",
        "→ Opening".blue().bold(),
        args.template_name.cyan().bold()
    ));

    // --- Load Manifest ---
    let mut manifest = config::load_manifest()?; // mutable because we might update it
    output.verbose("VERBOSE] Manifest loaded for 'edit' command.");

    // --- Find template entry in manifest ---
    let template_entry = match manifest.get_template(&args.template_name) {
        Some(entry) => entry,
        None => return Err(AppError::TemplateNotFound(args.template_name.clone())),
    };

    // Get the actual filename and construct the path
    let filename_in_storage = template_entry.filename_in_storage.clone();
    let templates_dir = config::get_templates_dir()?;
    let template_file_path = templates_dir.join(&filename_in_storage);

    output.info(
        format!("\t\t{} Editing file: {}",
        ">".magenta(),
        format!("{:?}", template_file_path).cyan()
    ));

    // Check if the template file actually exists (manifest consistency)
    if !template_file_path.exists() {
        return Err(AppError::TemplateFileMissing {
            name: args.template_name.clone(),
            path: template_file_path,
        });
    }

    // Use the `edit` crate to open the file 
    match edit::edit_file(&template_file_path) {
        Ok(()) => {
            output.verbose(format!(
                "\t\t[VERBOSE] Editor closed for template '{}'. Updating manifest.",
                args.template_name
            ));

            // Update `updated_at` timestamp in manifest 
            if let Some(entry_to_update) = manifest.get_template_mut(&args.template_name) {
                entry_to_update.updated_at = Utc::now();
                config::save_manifest(&manifest)?; // Save the updated manifest
                output.verbose(format!("[VERBOSE] Manifest saved with updated timestamp for '{}'.", args.template_name));
            } else {
                // This case should be rare if get_template succeeded earlier,
                // but handle defensively.
                output.warn(format!(
                    "Warning: Could not find template '{}' in manifest to update timestamp after edit.",
                    args.template_name
                ));
            }

            output.success(format!(
                "\n\t{} Finished editing template '{}'.",
                "✓".green().bold(),
                args.template_name.cyan()
            ));
            Ok(())
        }
        Err(io_err) => {
            // Using a more specific error type from AppError for editor failures
            Err(AppError::EditorFailed {
                path: template_file_path, // PathBuf is expected by this error variant
                source: io_err,
            })
        }
    }
}
