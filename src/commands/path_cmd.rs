use crate::cli::PathArgs; // Ensure this matches your CLI definition
use crate::config;
use crate::error::AppError;
use crate::utils;

/// Handles the `tempo path` command.
pub fn run(args: &PathArgs) -> Result<(), AppError> {
    let templates_dir = config::get_templates_dir()?;
    let template_file_path = utils::find_template_path(&templates_dir, &args.template_name)?;

    println!("{}", template_file_path.display());

    Ok(())
}
