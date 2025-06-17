use crate::config; // For config::ConfigError
use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum AppError {
    // --- Configuration Errors ---
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError), // Allows easy conversion from config::ConfigError

    // --- I/O Errors ---
    #[error("I/O error: {0}")]
    Io(#[from] io::Error), // General I/O errors

    #[error("Failed to read directory: {source_path:?}, due to: {source_error}")]
    ReadDir {
        source_path: PathBuf,
        #[source] // Marks the underlying source of the error for 'thiserror'
        source_error: io::Error,
    },

    #[error("Failed to copy file from {from:?} to {to:?}, due to: {source_error}")]
    FileCopy {
        from: PathBuf,
        to: PathBuf,
        #[source]
        source_error: io::Error,
    },

    #[error("Failed to remove file: {path:?}, due to: {source_error}")]
    FileRemove {
        path: PathBuf,
        #[source]
        source_error: io::Error,
    },

    // --- Command Specific Logic Errors ---
    // 'add' command related
    #[error("Source file does not exist: {0:?}")]
    SourceFileDoesNotExist(PathBuf),

    #[error("Source path is not a file: {0:?}")]
    SourcePathIsNotAFile(PathBuf),

    #[error("Template name '{0}' is invalid: {1}")]
    TemplateNameInvalid(String, String), // name, reason

    #[error("Template '{0}' already exists. Use --force to overwrite.")]
    TemplateAlreadyExists(String), // name

    // 'list' command related (could also be generic)
    #[error("Templates directory not found at: {0:?}")] // Could be part of Config if critical
    TemplatesDirNotFound(PathBuf),

    // 'new' command related (placeholders for now)
    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    #[error("Template file missing from storage: Manifest indicates template '{name}' should be at {path:?}, but the file was not found.")]
    TemplateFileMissing {
        name: String,
        path: PathBuf,
    },

    #[error(
        "Destination file '{0:?}' already exists. Specify a write strategy (-o, -a, -p) or use --force to overwrite."
    )]
    DestinationFileExists(PathBuf),

    #[error("Cannot {action} to destination '{dest:?}' because it's a directory.")]
    DestinationIsDirectory { action: String, dest: PathBuf },

    #[error("Confirmation required for '{action}' on template '{template_name}', but running in quiet mode. Use --force.")]
    ConfirmationNeededInQuietMode {
        action: String,
        template_name: String
    },
    
    #[error("Failed to open or run editor for {path:?}: {source}")]
    EditorFailed {
        path: std::path::PathBuf,
        #[source]
        source: std::io::Error,
    },

    // General / Other
    #[error("An unexpected error occurred: {0}")]
    Unexpected(String),
}
