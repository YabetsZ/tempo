use crate::config;
use crate::error::AppError;
use colored::*;
use crate::output::OutputConfig;

/// Handles the `tempo list` (or `tempo ls`) command.
///
/// # Returns
/// * `Ok(())` if the templates were listed successfully or if no templates exist.
/// * `Err(ListError)` if an error occurred.
pub fn run(output: &OutputConfig) -> Result<(), AppError> {
    let manifest = config::load_manifest()?;
    output.verbose(format!("\t\t[VERBOSE] Manifest loaded. Contains {} templates.", manifest.templates.len()));
    
    output.info(format!("\t{}", "Available templates:".blue().bold().underline()));

    if manifest.templates.is_empty() {
        output.info(format!(
            "\t\t{}",
            "No templates found. Use 'tempo add <name> <path>' to add one.".yellow()
        ));
        return Ok(());
    }
    // Sort entries by filename for consistent output
    let mut template_names: Vec<_> = manifest.templates.keys().collect();
    template_names.sort_by_key(|key| key.to_lowercase());

    for template_name in template_names {
        if let Some(entry) = manifest.templates.get(template_name){
            if !entry.source_extension.is_empty() {
                output.data(
                    format!("\t\t- {} {}",
                            template_name.cyan().bold(),
                            format!("(.{})", entry.source_extension).dimmed()
                    ));
            } else {
                output.data(format!("\t\t- {}", template_name.cyan().bold()));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*; // Import from outer module (list.rs)
    use crate::config;
    use std::fs::{self, File};
    use std::path::Path;
    
    // Helper to create a dummy template file in the actual templates directory
    fn create_template_in_actual_dir(templates_dir: &Path, filename: &str) {
        File::create(templates_dir.join(filename))
            .unwrap_or_else(|e| panic!("Failed to create dummy template {:?}: {}", filename, e));
    }

    // Helper to clear the templates directory (use with caution, targeted removal is better)
    fn clear_templates_dir(templates_dir: &Path) {
        if templates_dir.exists() {
            for entry in fs::read_dir(templates_dir).unwrap() {
                let entry = entry.unwrap();
                fs::remove_file(entry.path()).unwrap();
            }
        }
    }

    #[test]
    fn test_list_no_templates() {
        let templates_dir = config::get_templates_dir().unwrap();
        clear_templates_dir(&templates_dir); // Ensure it's empty
        let output = OutputConfig::new(true, false);

        // We need to capture stdout to verify the printed output.
        // This is a bit more involved. For now, let's just check Ok status.
        // TODO: Implement stdout capturing for more thorough list tests.
        let result = run(&output);
        assert!(result.is_ok());
        // Manual verification: run `cargo test -- --nocapture` and check output,
        // or implement stdout capturing (e.g. using 'gag' crate or std::io::set_output_capture).
    }

    #[test]
    fn test_list_with_templates() {
        let templates_dir = config::get_templates_dir().unwrap();
        let output = OutputConfig::new(true, false);
        clear_templates_dir(&templates_dir); // Start clean

        create_template_in_actual_dir(&templates_dir, "alpha.txt");
        create_template_in_actual_dir(&templates_dir, "beta.rs");
        create_template_in_actual_dir(&templates_dir, "gamma_tpl"); // No extension

        // TODO: Capture and assert stdout content.
        let result = run(&output);
        assert!(result.is_ok());

        // If we had stdout capture, we'd assert:
        // captured_stdout.contains("- alpha (.txt)");
        // captured_stdout.contains("- beta (.rs)");
        // captured_stdout.contains("- gamma_tpl");

        // Clean up
        fs::remove_file(templates_dir.join("alpha.txt")).unwrap();
        fs::remove_file(templates_dir.join("beta.rs")).unwrap();
        fs::remove_file(templates_dir.join("gamma_tpl")).unwrap();
    }

    // Test for what happens if templates_dir doesn't exist initially
    // This is harder to test because config::get_templates_dir() *creates* it.
    // We'd need to mock that or test a lower-level function.
    // For now, we trust that config::get_templates_dir handles creation.
}
