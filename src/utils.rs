use crate::error::AppError;
use std::fs;
use std::path::{Path, PathBuf};

/// A helper function to verify if a template exits, then return it's exact path
pub fn find_template_path(templates_dir: &Path, template_name: &str) -> Result<PathBuf, AppError> {
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
