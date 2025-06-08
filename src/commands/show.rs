use crate::{cli::ShowArgs, config, error::AppError, utils};
use atty;
use std::fs;
/// Handles the `tempo show` command.
pub fn run(args: &ShowArgs) -> Result<(), AppError> {
    let templates_dir = config::get_templates_dir()?;
    let template_file_path = utils::find_template_path(&templates_dir, &args.template_name)?;

    let content = fs::read_to_string(&template_file_path).map_err(|io_err| AppError::Io(io_err))?; // Could be more specific, e.g., AppError::TemplateReadError

    print!("{}", content);

    if atty::is(atty::Stream::Stdout) && !content.ends_with('\n') {
        println!(); // Add a newline if outputting to terminal and content doesn't have one
    }

    Ok(())
}
