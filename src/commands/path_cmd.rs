use crate::cli::PathArgs; // Ensure this matches your CLI definition
use crate::config;
use crate::error::AppError;
use crate::output::OutputConfig;
use crate::utils;

/// Handles the `tempo path` command.
pub fn run(args: &PathArgs, output: &OutputConfig) -> Result<(), AppError> {
    let templates_dir = config::get_templates_dir()?;
    output.verbose(format!("[VERBOSE] Getting path for template '{}' from: {:?}", args.template_name, templates_dir));
    let template_file_path = utils::find_template_path(&templates_dir, &args.template_name)?;
    output.verbose(format!("[VERBOSE] Template file found at: {:?}", template_file_path));

    output.data(template_file_path.display().to_string()); 

    Ok(())
}
