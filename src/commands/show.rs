use crate::{cli::ShowArgs, config, error::AppError};
use atty;
use std::fs;
use crate::output::OutputConfig;

/// Handles the `tempo show` command.
pub fn run(args: &ShowArgs, output: &OutputConfig) -> Result<(), AppError> {
    let manifest = config::load_manifest()?;
    output.verbose("[VERBOSE] Manifest loaded for 'show' command.");
    
    let template_entry = match manifest.get_template(&args.template_name) {
        Some(entry) => entry,
        None => return Err(AppError::TemplateNotFound(args.template_name.clone())),
    };

    let filename_in_storage = &template_entry.filename_in_storage;
    let templates_dir = config::get_templates_dir()?; // Ensure templates dir path is available
    output.verbose(format!("[VERBOSE] Showing template '{}' from: {:?}", args.template_name, templates_dir));
    let template_file_path = templates_dir.join(filename_in_storage);
    output.verbose(format!("[VERBOSE] Template file found at: {:?}", template_file_path));

    let content = fs::read_to_string(&template_file_path).map_err(|io_err| {
        // If file not found here, it implies inconsistency between manifest and filesystem
        if io_err.kind() == std::io::ErrorKind::NotFound {
            AppError::TemplateFileMissing {
                name: args.template_name.clone(),
                path: template_file_path, 
            }
        } else {
            AppError::Io(io_err) 
        }
    })?;
    
    output.data_no_nl(format!("{}", content));

    if atty::is(atty::Stream::Stdout) && !content.ends_with('\n') {
        println!(); // Add a newline if outputting to terminal and content doesn't have one
    }

    Ok(())
}
