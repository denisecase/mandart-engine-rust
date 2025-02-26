use image::RgbImage;
use std::io;
use std::path::Path;

/// Saves an `RgbImage` as a PNG file.
pub fn save_image_to_png(image: &RgbImage, file_path: &str) -> io::Result<()> {
    image
        .save(&Path::new(file_path))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to save PNG: {}", e)))?;

    println!("✅ PNG saved to `{}`", file_path);
    Ok(())
}
