//! `get_picdef_from_mandart_file.rs` - Extracts PicDef JSON from a `.mandart` file.

use std::fs;
use std::io::{self, Error, ErrorKind};

/// **Reads a `.mandart` file and extracts the PicDef JSON string.**
///
/// - **`file_path`**: Path to the `.mandart` file.
///
/// **Returns**:
/// - `Ok(String)`: The PicDef JSON string.
/// - `Err(io::Error)`: If the file cannot be read or is not valid JSON.
///
/// **Example Usage:**
/// ```rust
/// let picdef_json = get_picdef_from_mandart_file("example.mandart")?;
/// ```
pub fn get_picdef_from_mandart_file(file_path: &str) -> io::Result<String> {
    let file_contents = fs::read_to_string(file_path)
        .map_err(|e| Error::new(ErrorKind::NotFound, format!("File not found: {}", e)))?;

    // ✅ The `.mandart` file **must** contain valid JSON, otherwise return an error.
    if serde_json::from_str::<serde_json::Value>(&file_contents).is_err() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Invalid JSON format in .mandart file",
        ));
    }

    Ok(file_contents)
}
