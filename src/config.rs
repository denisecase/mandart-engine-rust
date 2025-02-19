//! `config.rs` - Reads and manages configuration settings

use std::collections::HashMap;
use std::fs;
use toml::Value;

/// Loads configuration settings from a given file, or defaults to `config.toml` if None is provided.
pub fn load_config(config_path: Option<&str>) -> HashMap<String, String> {
    let mut config = HashMap::new();
    let config_file = config_path.unwrap_or("config.toml"); // Use the provided file or default

    let config_str = fs::read_to_string(config_file).unwrap_or_else(|_| {
        eprintln!("⚠️ Warning: Could not read `{}`, using defaults.", config_file);
        String::new()
    });

    let parsed: Value = toml::from_str(&config_str).unwrap_or_else(|_| {
        eprintln!("⚠️ Warning: Failed to parse `{}`, using defaults.", config_file);
        Value::Table(Default::default())
    });

    // Load values with defaults
    config.insert("input_folder".to_string(), parsed.get("input_folder").and_then(Value::as_str).unwrap_or("input").to_string());
    config.insert("output_folder".to_string(), parsed.get("output_folder").and_then(Value::as_str).unwrap_or("output").to_string());
    config.insert("check_folder".to_string(), parsed.get("check_folder").and_then(Value::as_str).unwrap_or("input_swift").to_string());
    config.insert("save_grid".to_string(), parsed.get("save_grid").and_then(Value::as_bool).unwrap_or(true).to_string());

    config
}
