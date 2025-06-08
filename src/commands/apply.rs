use crate::cli::ApplyArgs;
use crate::config;
use crate::error::AppError;
use colored::*;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

// Helper function to find a template file by its base name
// Returns the full path to the template file if found.
fn find_template_path(templates_dir: &Path, template_name: &str) -> Result<PathBuf, AppError> {
    let entries = fs::read_dir(templates_dir).map_err(|e| AppError::ReadDir {
        source_path: templates_dir.to_path_buf(),
        source_error: e,
    })?;

    for entry_result in entries {
        let entry = entry_result.map_err(|e| AppError::ReadDir {
            source_path: templates_dir.to_path_buf(), // Or be more specific about entry path if possible
            source_error: e,
        })?;
        let path = entry.path();
        if path.is_file() {
            if let Some(stem) = path.file_stem() {
                if stem == template_name {
                    return Ok(path);
                }
            }
        }
    }
    Err(AppError::TemplateNotFound(template_name.to_string()))
}

/// Handles the `tempo new` command.
pub fn run(args: &ApplyArgs, force: bool) -> Result<(), AppError> {
    println!(
        "\n\t{} template {} to {}...",
        "→ Applying".blue().bold(),
        args.template_name.yellow().bold(),
        format!("{:?}", args.destination_file_path).cyan()
    );

    // 1. Find the template
    let templates_dir = config::get_templates_dir()?;
    let template_file_path = find_template_path(&templates_dir, &args.template_name)?;
    println!(
        "\t\t{} Using template file: {}",
        ">".magenta(),
        format!("{template_file_path:?}").cyan()
    );

    // 2. Read template content
    let template_content = fs::read_to_string(&template_file_path).map_err(|e| AppError::Io(e))?;
    // Consider adding a specific AppError variant for template read failure if needed

    let dest_path = &args.destination_file_path;

    // 3. Handle destination file
    if dest_path.exists() {
        if dest_path.is_dir() {
            return Err(AppError::DestinationIsDirectory {
                action: "apply template".to_string(),
                dest: dest_path.to_path_buf(),
            });
        }

        // Destination file exists, apply strategy
        if args.overwrite {
            println!("\t\t{} Overwriting existing file.", ">".magenta());
            fs::write(dest_path, &template_content).map_err(|e| AppError::Io(e))?;
        } else if args.append {
            println!("\t\t{} Appending to existing file.", ">".magenta());
            let mut file = OpenOptions::new().append(true).open(dest_path)?; // AppError::Io handles error
            file.write_all(template_content.as_bytes())?;
        } else if args.prepend {
            println!("\t\t{} Prepending to existing file.", ">".magenta());
            let mut original_content = String::new();
            File::open(dest_path)?.read_to_string(&mut original_content)?;

            let new_content = format!("{}\n{}", template_content, original_content);
            fs::write(dest_path, new_content)?;
        } else if force {
            // No specific strategy flag, but --force is active
            println!(
                "\t\t{} Overwriting existing file (due to --force).",
                ">".magenta()
            );
            fs::write(dest_path, &template_content)?;
        } else {
            // No strategy flag, no --force, and file exists
            return Err(AppError::DestinationFileExists(dest_path.to_path_buf()));
        }
    } else {
        // Destination file does not exist, create it
        println!("\t\t{} Creating new file.", ">".magenta());
        if let Some(parent_dir) = dest_path.parent() {
            if !parent_dir.exists() {
                fs::create_dir_all(parent_dir)?; // AppError::Io handles error
                println!(
                    "\t\t{} Created parent directory: {:?}",
                    ">".magenta(),
                    parent_dir
                );
            }
        }
        fs::write(dest_path, &template_content)?;
    }

    println!(
        "\n\t{} Template '{}' applied to {}",
        "✓ Successfully".green().bold(),
        args.template_name.yellow(),
        format!("{:?}", dest_path).cyan()
    );

    Ok(())
}

// At the bottom of src/commands/apply.rs

#[cfg(test)]
mod tests {
    use super::*; // Imports run, find_template_path from apply.rs
    use crate::cli::ApplyArgs; // Or whatever your renamed struct is
    use crate::config; // To get the real templates_dir for setup
    use crate::error::AppError;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir; // For destination files

    // Helper to create a dummy template file in the *actual* templates directory
    // managed by config::get_templates_dir().
    // Needs to be cleaned up afterwards.
    fn setup_actual_template(template_name: &str, content: &str) -> PathBuf {
        let templates_dir =
            config::get_templates_dir().expect("Failed to get actual templates dir for test setup");
        // Ensure templates_dir exists (get_templates_dir should do this)
        // fs::create_dir_all(&templates_dir).expect("Failed to create templates dir for test");

        let template_file_path = templates_dir.join(template_name); // e.g., my_tpl.txt
        let mut file = File::create(&template_file_path).unwrap_or_else(|e| {
            panic!(
                "Failed to create dummy template {:?}: {}",
                &template_file_path, e
            )
        });
        writeln!(file, "{}", content).unwrap_or_else(|e| {
            panic!(
                "Failed to write to dummy template {:?}: {}",
                &template_file_path, e
            )
        });
        template_file_path
    }

    fn cleanup_actual_template(template_file_path: &Path) {
        if template_file_path.exists() {
            fs::remove_file(template_file_path).unwrap_or_else(|e| {
                panic!(
                    "Failed to clean up template {:?}: {}",
                    template_file_path, e
                )
            });
        }
    }

    #[test]
    fn test_apply_to_new_file_successful() {
        let template_filename = "test_tpl_new.txt";
        let template_content = "Template for new file.";
        let actual_template_path = setup_actual_template(template_filename, template_content);

        let dest_dir = tempdir().unwrap();
        let dest_file_path = dest_dir.path().join("output.txt");

        let args = ApplyArgs {
            template_name: "test_tpl_new".to_string(), // Name without extension
            destination_file_path: dest_file_path.clone(),
            overwrite: false,
            append: false,
            prepend: false,
        };

        let result = run(&args, false); // force = false
        assert!(result.is_ok(), "apply run failed: {:?}", result.err());
        assert!(dest_file_path.exists());
        let content = fs::read_to_string(&dest_file_path).unwrap();
        assert_eq!(content.trim(), template_content);

        cleanup_actual_template(&actual_template_path);
    }

    #[test]
    fn test_apply_template_not_found() {
        let dest_dir = tempdir().unwrap();
        let dest_file_path = dest_dir.path().join("output.txt");

        let args = ApplyArgs {
            template_name: "non_existent_template_for_apply".to_string(),
            destination_file_path: dest_file_path.clone(),
            overwrite: false,
            append: false,
            prepend: false,
        };

        let result = run(&args, false);
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::TemplateNotFound(name) => assert_eq!(name, "non_existent_template_for_apply"),
            e => panic!("Expected TemplateNotFound, got {:?}", e),
        }
    }

    #[test]
    fn test_apply_destination_exists_no_flags_error() {
        let template_filename = "test_tpl_exists_err.txt";
        let actual_template_path = setup_actual_template(template_filename, "content");

        let dest_dir = tempdir().unwrap();
        let dest_file_path = dest_dir.path().join("output.txt");
        File::create(&dest_file_path)
            .unwrap()
            .write_all(b"Original")
            .unwrap();

        let args = ApplyArgs {
            template_name: "test_tpl_exists_err".to_string(),
            destination_file_path: dest_file_path.clone(),
            overwrite: false,
            append: false,
            prepend: false,
        };

        let result = run(&args, false); // No force
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::DestinationFileExists(path) => assert_eq!(path, dest_file_path),
            e => panic!("Expected DestinationFileExists, got {:?}", e),
        }
        cleanup_actual_template(&actual_template_path);
    }

    #[test]
    fn test_apply_destination_exists_overwrite_flag() {
        let template_filename = "test_tpl_overwrite.txt";
        let template_content = "Template overwrite";
        let actual_template_path = setup_actual_template(template_filename, template_content);

        let dest_dir = tempdir().unwrap();
        let dest_file_path = dest_dir.path().join("output.txt");
        File::create(&dest_file_path)
            .unwrap()
            .write_all(b"Original")
            .unwrap();

        let args = ApplyArgs {
            template_name: "test_tpl_overwrite".to_string(),
            destination_file_path: dest_file_path.clone(),
            overwrite: true, // -o flag
            append: false,
            prepend: false,
        };

        let result = run(&args, false);
        assert!(
            result.is_ok(),
            "apply run failed for overwrite: {:?}",
            result.err()
        );
        let content = fs::read_to_string(&dest_file_path).unwrap();
        assert_eq!(content.trim(), template_content);

        cleanup_actual_template(&actual_template_path);
    }

    #[test]
    fn test_apply_destination_exists_force_global_flag() {
        let template_filename = "test_tpl_force.txt";
        let template_content = "Template force global";
        let actual_template_path = setup_actual_template(template_filename, template_content);

        let dest_dir = tempdir().unwrap();
        let dest_file_path = dest_dir.path().join("output.txt");
        File::create(&dest_file_path)
            .unwrap()
            .write_all(b"Original")
            .unwrap();

        let args = ApplyArgs {
            template_name: "test_tpl_force".to_string(),
            destination_file_path: dest_file_path.clone(),
            overwrite: false, // No -o flag specifically
            append: false,
            prepend: false,
        };

        let result = run(&args, true); // Global force = true
        assert!(
            result.is_ok(),
            "apply run failed for global force: {:?}",
            result.err()
        );
        let content = fs::read_to_string(&dest_file_path).unwrap();
        assert_eq!(content.trim(), template_content);

        cleanup_actual_template(&actual_template_path);
    }

    #[test]
    fn test_apply_destination_exists_append_flag() {
        let template_filename = "test_tpl_append.txt";
        let template_content = "Appended Content";
        let actual_template_path = setup_actual_template(template_filename, template_content);

        let dest_dir = tempdir().unwrap();
        let dest_file_path = dest_dir.path().join("output_append.txt");
        let original_content = "Original Content.\n"; // Ensure newline for clean append
        File::create(&dest_file_path)
            .unwrap()
            .write_all(original_content.as_bytes())
            .unwrap();

        let args = ApplyArgs {
            template_name: "test_tpl_append".to_string(),
            destination_file_path: dest_file_path.clone(),
            overwrite: false,
            append: true, // -a flag
            prepend: false,
        };

        let result = run(&args, false);
        assert!(
            result.is_ok(),
            "apply run failed for append: {:?}",
            result.err()
        );
        let content = fs::read_to_string(&dest_file_path).unwrap();
        // Template content includes a newline from writeln! in setup_actual_template
        let expected_content = format!("{}{}\n", original_content, template_content);
        assert_eq!(content, expected_content);

        cleanup_actual_template(&actual_template_path);
    }

    #[test]
    fn test_apply_destination_exists_prepend_flag() {
        let template_filename = "test_tpl_prepend.txt";
        let template_content = "Prepended Content";
        let actual_template_path = setup_actual_template(template_filename, template_content);

        let dest_dir = tempdir().unwrap();
        let dest_file_path = dest_dir.path().join("output_prepend.txt");
        let original_content = "Original Content.";
        File::create(&dest_file_path)
            .unwrap()
            .write_all(original_content.as_bytes())
            .unwrap();

        let args = ApplyArgs {
            template_name: "test_tpl_prepend".to_string(),
            destination_file_path: dest_file_path.clone(),
            overwrite: false,
            append: false,
            prepend: true, // -p flag
        };

        let result = run(&args, false);
        assert!(result.is_ok());
        let content = fs::read_to_string(&dest_file_path).unwrap();
        // Template content includes a newline from writeln! in setup_actual_template
        let expected_content = format!("{}\n\n{}", template_content, original_content);
        assert_eq!(content, expected_content);

        cleanup_actual_template(&actual_template_path);
    }

    #[test]
    fn test_apply_destination_is_directory() {
        let template_filename = "test_tpl_dest_is_dir.txt";
        let actual_template_path = setup_actual_template(template_filename, "content");

        let dest_dir_for_target = tempdir().unwrap(); // This directory will be the target

        let args = ApplyArgs {
            template_name: "test_tpl_dest_is_dir".to_string(),
            destination_file_path: dest_dir_for_target.path().to_path_buf(), // Path to directory
            overwrite: false,
            append: false,
            prepend: false,
        };

        let result = run(&args, false);
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::DestinationIsDirectory { dest, .. } => {
                assert_eq!(dest, dest_dir_for_target.path())
            }
            e => panic!("Expected DestinationIsDirectory, got {:?}", e),
        }
        cleanup_actual_template(&actual_template_path);
    }

    #[test]
    fn test_apply_creates_parent_directories() {
        let template_filename = "test_tpl_parent_dir.txt";
        let template_content = "Content for subdir";
        let actual_template_path = setup_actual_template(template_filename, template_content);

        let base_dest_dir = tempdir().unwrap();
        let dest_file_path = base_dest_dir.path().join("new_parent").join("output.txt");
        // new_parent directory does not exist yet

        let args = ApplyArgs {
            template_name: "test_tpl_parent_dir".to_string(),
            destination_file_path: dest_file_path.clone(),
            overwrite: false,
            append: false,
            prepend: false,
        };

        assert!(
            !dest_file_path.parent().unwrap().exists(),
            "Parent directory should not exist before test"
        );
        let result = run(&args, false);
        assert!(
            result.is_ok(),
            "apply run failed when creating parent: {:?}",
            result.err()
        );
        assert!(dest_file_path.exists());
        assert!(
            dest_file_path.parent().unwrap().exists(),
            "Parent directory was not created"
        );
        let content = fs::read_to_string(&dest_file_path).unwrap();
        assert_eq!(content.trim(), template_content);

        cleanup_actual_template(&actual_template_path);
    }
}
