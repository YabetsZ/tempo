use crate::{cli::ShowArgs, config, error::AppError, utils};
use atty;
use std::fs;
use crate::output::OutputConfig;

/// Handles the `tempo show` command.
pub fn run(args: &ShowArgs, output: &OutputConfig) -> Result<(), AppError> {
    let templates_dir = config::get_templates_dir()?;
    output.verbose(format!("[VERBOSE] Showing template '{}' from: {:?}", args.template_name, templates_dir));
    let template_file_path = utils::find_template_path(&templates_dir, &args.template_name)?;
    output.verbose(format!("[VERBOSE] Template file found at: {:?}", template_file_path));

    let content = fs::read_to_string(&template_file_path).map_err(|io_err| AppError::Io(io_err))?; // Could be more specific, e.g., AppError::TemplateReadError

    output.data_no_nl(format!("{}", content));

    if atty::is(atty::Stream::Stdout) && !content.ends_with('\n') {
        println!(); // Add a newline if outputting to terminal and content doesn't have one
    }

    Ok(())
}
