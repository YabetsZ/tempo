use crate::commands::error::AppError;
use crate::config;
use colored::*;
use std::fs;
use std::path::Path;

/// Handles the `tempo list` (or `tempo ls`) command.
///
/// # Returns
/// * `Ok(())` if the templates were listed successfully or if no templates exist.
/// * `Err(ListError)` if an error occurred.
pub fn run() -> Result<(), AppError> {
    println!("\n{}\n", "Available templates:".blue().bold().underline());

    let templates_dir = config::get_templates_dir()?;

    // if !templates_dir.exists() || !templates_dir.is_dir() {
    //     // This case should ideally be handled by get_templates_dir creating it,
    //     // but as a safeguard or if permissions change.
    //     return Err(ListError::TemplatesDirNotFound(templates_dir));
    // }

    let mut entries: Vec<_> = fs::read_dir(&templates_dir)?
        .filter_map(|entry_result| entry_result.ok()) // Ignore entries that cause an error during iteration
        .filter(|entry| entry.path().is_file()) // We only care about files
        .collect();

    if entries.is_empty() {
        println!(
            "\t{}",
            "No templates found. Use 'tempo add <name> <path>' to add one.".yellow()
        );
        return Ok(());
    }

    // Sort entries by filename for consistent output
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let path = entry.path();
        let file_name_osstr = path.file_name().unwrap_or_default(); // Should always have a filename here
        let file_name_str = file_name_osstr.to_string_lossy(); // Convert OsStr to String (lossy)

        let template_name = Path::new(&*file_name_str)
            .file_stem() // Gets filename without final extension
            .unwrap_or_default() // Should have a stem
            .to_string_lossy();

        let extension = Path::new(&*file_name_str)
            .extension()
            .unwrap_or_default()
            .to_string_lossy();

        if !extension.is_empty() {
            println!(
                "\t- {} {}",
                template_name.cyan().bold(),
                format!("(.{})", extension).dimmed()
            );
        } else {
            println!("\t- {}", template_name.cyan().bold());
        }
    }
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*; // Import from outer module (list.rs)
    use crate::config;
    use std::fs::{self, File};
    use tempfile::tempdir;

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

        // We need to capture stdout to verify the printed output.
        // This is a bit more involved. For now, let's just check Ok status.
        // TODO: Implement stdout capturing for more thorough list tests.
        let result = run();
        assert!(result.is_ok());
        // Manual verification: run `cargo test -- --nocapture` and check output,
        // or implement stdout capturing (e.g. using 'gag' crate or std::io::set_output_capture).
    }

    #[test]
    fn test_list_with_templates() {
        let templates_dir = config::get_templates_dir().unwrap();
        clear_templates_dir(&templates_dir); // Start clean

        create_template_in_actual_dir(&templates_dir, "alpha.txt");
        create_template_in_actual_dir(&templates_dir, "beta.rs");
        create_template_in_actual_dir(&templates_dir, "gamma_tpl"); // No extension

        // TODO: Capture and assert stdout content.
        let result = run();
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
