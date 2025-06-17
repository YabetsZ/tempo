use chrono::{DateTime, Utc}; // For timestamps
use serde::{Deserialize, Serialize}; // For SerDe
use std::collections::HashMap;
use std::path::PathBuf; 

/// Represents a single template entry in the manifest.
#[derive(Serialize, Deserialize, Debug, Clone)] 
pub struct TemplateEntry {
    /// The actual filename as stored in the `templates` directory.
    /// e.g., "my_template_v1.rs"
    pub filename_in_storage: String,

    /// The original extension of the source file (e.g., "rs", "txt").
    /// This helps in reconstructing the typical display or for language hinting.
    pub source_extension: String,

    /// Timestamp of when the template was added.
    #[serde(with = "chrono::serde::ts_seconds")] // Serialize as Unix timestamp (seconds)
    pub created_at: DateTime<Utc>,

    /// Timestamp of when the template was last modified (e.g., via `tempo edit`).
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,

    // --- Optional fields we can add later ---
    /// An optional description for the template.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional: The original path from where the template was sourced.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_source_path: Option<PathBuf>, // PathBuf might need custom serde if not careful

    /// Optional: Tags for categorizing templates.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    /// Optional: detected or specified language
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl TemplateEntry {
    /// Creates a new template entry.
    pub fn new(filename_in_storage: String, source_extension: String) -> Self {
        let now = Utc::now();
        TemplateEntry {
            filename_in_storage,
            source_extension,
            created_at: now,
            updated_at: now,
            description: None,
            original_source_path: None, // Can be set during 'add'
            tags: Vec::new(),
            language: None,
        }
    }
}

/// Represents the entire manifest.
/// The HashMap key is the user-facing `template_name`.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(transparent)] // Treat this struct as its inner field for SerDe
pub struct Manifest {
    pub templates: HashMap<String, TemplateEntry>,
}

impl Manifest {
    /// Creates an empty manifest.
    pub fn new() -> Self {
        Manifest {
            templates: HashMap::new(),
        }
    }

    /// Adds or updates a template entry in the manifest.
    /// If the name already exists, it updates the entry and bumps `updated_at`.
    pub fn add_template(&mut self, name: String, mut entry: TemplateEntry) {
        if self.templates.contains_key(&name) {
            // If template with this name exists, update its `updated_at`
            entry.created_at = self.templates.get(&name).unwrap().created_at; // Keep original creation
            entry.updated_at = Utc::now();
        }
        self.templates.insert(name, entry);
    }

    /// Removes a template entry from the manifest.
    /// Returns the removed entry if it existed.
    pub fn remove_template(&mut self, name: &str) -> Option<TemplateEntry> {
        self.templates.remove(name)
    }

    /// Gets a reference to a template entry.
    pub fn get_template(&self, name: &str) -> Option<&TemplateEntry> {
        self.templates.get(name)
    }

    /// Gets a mutable reference to a template entry for updates (like `tempo edit`).
    pub fn get_template_mut(&mut self, name: &str) -> Option<&mut TemplateEntry> {
        self.templates.get_mut(name)
    }
}