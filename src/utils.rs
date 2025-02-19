//! `utils.rs` - General Helper Functions

use std::fs;
use std::path::Path;

/// Lists files in a directory with a specific extension.
pub fn list_files_in_dir(directory: &str, extension: &str) -> Vec<String> {
    fs::read_dir(directory)
        .unwrap_or_else(|_| panic!("Failed to read directory `{}`", directory))
        .filter_map(|entry| entry.ok().map(|e| e.path().to_string_lossy().into_owned()))
        .filter(|path| Path::new(path).extension().map_or(false, |ext| ext == extension.trim_start_matches(".")))
        .collect()
}

/// Ensures a directory exists, creating it if necessary.
pub fn ensure_directory_exists(directory: &str) -> Result<(), String> {
    if !Path::new(directory).exists() {
        fs::create_dir_all(directory).map_err(|e| format!("Failed to create directory `{}`: {}", directory, e))?
    }
    Ok(())
}