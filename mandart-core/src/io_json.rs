use std::fs;
use std::io::{self};

/// Reads a JSON file and returns its contents as a string.
pub fn read_json_file(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
}

/// Writes a JSON string to a file.
pub fn write_json_file(file_path: &str, json_content: &str) -> io::Result<()> {
    fs::write(file_path, json_content)
}
