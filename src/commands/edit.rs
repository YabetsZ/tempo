use crate::cli::EditArgs;
use crate::config;
use crate::error::AppError;
use crate::utils;
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

    let templates_dir = config::get_templates_dir()?;
    output.verbose(format!("[VERBOSE] Editing template '{}' from: {:?}", args.template_name, templates_dir));
    let template_file_path = utils::find_template_path(&templates_dir, &args.template_name)?;

    output.info(
        format!("\t\t{} Editing file: {}",
        ">".magenta(),
        format!("{:?}", template_file_path).cyan()
    ));

    match edit::edit_file(&template_file_path) {
        Ok(()) => {
            output.success(format!(
                "\n\t{} Finished editing template '{}'.",
                "✓".green().bold(),
                args.template_name.cyan()
            ));
            Ok(())
        }
        Err(io_err) => {
            Err(AppError::EditorFailed {
                path: template_file_path,
                source: io_err,
            })
        }
    }
}
