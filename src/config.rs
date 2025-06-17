use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use thiserror::Error;
use crate::manifest::Manifest;
use toml;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to create directory {0:?}: {1}")]
    DirectoryCreationError(PathBuf, #[source] io::Error),

    #[error("Could not determine a valid configuration directory for tempo.")]
    NoConfigDirectory,

    #[error("Failed to read manifest file at {path:?}: {source_error}")]
    ManifestReadError {
        path: PathBuf,
        #[source]
        source_error: io::Error,
    },

    #[error("Failed to write manifest file at {path:?}: {source_error}")]
    ManifestWriteError {
        path: PathBuf,
        #[source]
        source_error: io::Error,
    },

    #[error("Failed to parse manifest file at {path:?}: {source_error}")]
    ManifestParseError {
        path: PathBuf,
        #[source]
        source_error: toml::de::Error, // Error from toml deserialization
    },

    #[error("Failed to serialize manifest data: {source_error}")]
    ManifestSerializeError {
        #[source]
        source_error: toml::ser::Error, // Error from toml serialization
    },
}

const APP_NAME: &str = "tempo";
const MANIFEST_FILENAME: &str = "manifest.toml";

/// Gets the application's base configuration directory.
/// This is typically ~/.config/tempo/ on Linux/macOS
/// or %APPDATA%\tempo\ on Windows.
///
/// It will attempt to create this directory if it doesn't exist.
///
/// # Errors
///
/// Returns `ConfigError::NoConfigDirectory` if the system's config directory cannot be determined.
/// Returns `ConfigError::DirectoryCreationError` if the directory cannot be created.
pub fn get_app_config_dir() -> Result<PathBuf, ConfigError> {
    match dirs::config_dir() {
        Some(mut path) => {
            path.push(APP_NAME);
            if !path.exists() {
                fs::create_dir_all(&path)
                    .map_err(|e| ConfigError::DirectoryCreationError(path.clone(), e))?;
            }
            Ok(path)
        }
        None => Err(ConfigError::NoConfigDirectory),
    }
}

/// Gets the directory where templates will be stored.
/// This is typically ~/.config/tempo/templates/
///
/// It will attempt to create this directory (and its parent) if it doesn't exist.
///
/// # Errors
///
/// Returns `ConfigError` if the base config directory cannot be determined or
/// if the templates directory cannot be created.
pub fn get_templates_dir() -> Result<PathBuf, ConfigError> {
    let mut path = get_app_config_dir()?;
    path.push("templates");
    if !path.exists() {
        fs::create_dir_all(&path)
            .map_err(|e| ConfigError::DirectoryCreationError(path.clone(), e))?;
    }
    Ok(path)
}

/// Gets the full path to the manifest file (e.g., manifest.toml).
pub fn get_manifest_path() -> Result<PathBuf, ConfigError> {
    let mut path = get_app_config_dir()?;
    path.push(MANIFEST_FILENAME);
    Ok(path)
}

/// Loads the manifest from the manifest file.
/// If the file doesn't exist, returns a new empty Manifest.
pub fn load_manifest() -> Result<Manifest, ConfigError> {
    let manifest_path = get_manifest_path()?;

    if !manifest_path.exists() {
        return Ok(Manifest::new());
    }

    let mut file_content = String::new();
    File::open(&manifest_path)
        .map_err(|e| ConfigError::ManifestReadError {
            path: manifest_path.clone(),
            source_error: e,
        })?
        .read_to_string(&mut file_content)
        .map_err(|e| ConfigError::ManifestReadError {
            path: manifest_path.clone(),
            source_error: e,
        })?;

    if file_content.trim().is_empty() {
        return Ok(Manifest::new());
    }

    toml::from_str(&file_content).map_err(|e| ConfigError::ManifestParseError {
        path: manifest_path, // No clone needed here as it's the last use
        source_error: e,
    })
}

/// Saves the given Manifest data to the manifest file.
/// This will overwrite the existing manifest file.
pub fn save_manifest(manifest: &Manifest) -> Result<(), ConfigError> {
    let manifest_path = get_manifest_path()?;

    let toml_string =
        toml::to_string_pretty(manifest).map_err(|e| ConfigError::ManifestSerializeError {
            source_error: e,
        })?;

    // Write to the file, creating it if it doesn't exist, truncating if it does.
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // Overwrite existing content
        .open(&manifest_path)
        .map_err(|e| ConfigError::ManifestWriteError {
            path: manifest_path.clone(),
            source_error: e,
        })?;

    file.write_all(toml_string.as_bytes())
        .map_err(|e| ConfigError::ManifestWriteError {
            path: manifest_path, // No clone needed here
            source_error: e,
        })?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir; // For creating temporary directories for tests

    // Helper to set up a temporary isolated environment for config tests
    #[allow(dead_code)]
    fn setup_test_env() -> (tempfile::TempDir, PathBuf) {
        let temp_home = tempdir().expect("Failed to create temp dir for home");
        // let fake_config_path = temp_home.path().join(".config"); // Simulate ~/.config

        let app_base_dir = temp_home.path().join(APP_NAME);
        fs::create_dir_all(&app_base_dir).expect("Failed to create temp app base dir");
        (temp_home, app_base_dir)
    }

    #[test]
    fn test_get_templates_dir_creates_if_not_exists() {
        // A simpler test: if we manually create the structure, does it find it?
        let temp_root = tempdir().unwrap();
        let app_config = temp_root.path().join(APP_NAME);
        fs::create_dir_all(&app_config).unwrap();
        let templates_dir_expected = app_config.join("templates");

        // We can't directly call `get_templates_dir()` and have it use `temp_root`
        // without mocking `dirs::config_dir()`.

        // Let's test a slightly refactored idea: a function that takes a base path.
        fn get_templates_dir_with_base(base_path: &PathBuf) -> Result<PathBuf, ConfigError> {
            let mut path = base_path.clone();
            path.push("templates");
            if !path.exists() {
                fs::create_dir_all(&path)
                    .map_err(|e| ConfigError::DirectoryCreationError(path.clone(), e))?;
            }
            Ok(path)
        }

        let result = get_templates_dir_with_base(&app_config);
        assert!(result.is_ok());
        let templates_path = result.unwrap();
        assert_eq!(templates_path, templates_dir_expected);
        assert!(templates_path.exists());
        assert!(templates_path.is_dir());

        // Clean up (temp_root does this automatically when it goes out of scope)
    }

    #[test]
    fn test_app_name_constant() {
        assert_eq!(APP_NAME, "tempo");
    }
}
