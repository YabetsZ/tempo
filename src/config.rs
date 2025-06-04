use std::fs;
use std::io;
use std::path::PathBuf;
#[allow(dead_code)]
#[derive(Debug)]
pub enum ConfigError {
    DirectoryCreationError(PathBuf, io::Error),
    NoConfigDirectory,
}

// Implement std::fmt::Display for ConfigError to make it printable
impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::DirectoryCreationError(path, err) => {
                write!(f, "Failed to create directory {:?}: {}", path, err)
            }
            ConfigError::NoConfigDirectory => {
                write!(
                    f,
                    "Could not determine a valid configuration directory for tempo."
                )
            }
        }
    }
}

// Implement std::error::Error for ConfigError
impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::DirectoryCreationError(_, err) => Some(err),
            ConfigError::NoConfigDirectory => None,
        }
    }
}

const APP_NAME: &str = "tempo";

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

// Optional: A function to get the manifest file path (we'll use this later)
// pub fn get_manifest_path() -> Result<PathBuf, ConfigError> {
//     let mut path = get_app_config_dir()?;
//     path.push("manifest.json"); // Or manifest.toml
//     Ok(path)
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir; // For creating temporary directories for tests

    // Helper to set up a temporary isolated environment for config tests
    #[allow(dead_code)]
    fn setup_test_env() -> (tempfile::TempDir, PathBuf) {
        let temp_home = tempdir().expect("Failed to create temp dir for home");
        let fake_config_path = temp_home.path().join(".config"); // Simulate ~/.config

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
