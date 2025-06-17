use crate::cli::PathArgs; // Ensure this matches your CLI definition
use crate::config;
use crate::error::AppError;
use crate::output::OutputConfig;

/// Handles the `tempo path` command.
pub fn run(args: &PathArgs, output: &OutputConfig) -> Result<(), AppError> {
    let manifest = config::load_manifest()?;
    output.verbose("[VERBOSE] Manifest loaded for 'path' command.");
    // Find template entry in manifest
    let template_entry = match manifest.get_template(&args.template_name) {
        Some(entry) => entry,
        None => return Err(AppError::TemplateNotFound(args.template_name.clone())),
    };

    // Get the actual filename and construct the path
    let filename_in_storage = &template_entry.filename_in_storage;
    let templates_dir = config::get_templates_dir()?;
    let template_file_path = templates_dir.join(filename_in_storage);

    output.verbose(format!(
        "\t\t[VERBOSE] Path for template '{}' is: {:?}",
        args.template_name, template_file_path
    ));

    // Check if the file exists, if not, return an error
    if !template_file_path.exists() {
        return Err(AppError::TemplateFileMissing {
            name: args.template_name.clone(),
            path: template_file_path,
        });
    }
    
    output.data(format!("{}", template_file_path.display()));

    Ok(())
}
