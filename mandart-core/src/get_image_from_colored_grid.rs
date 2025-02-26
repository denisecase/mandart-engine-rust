//! `get_image_from_colored_grid.rs` - Converts a **colored grid** into an image information format

use image::{ImageBuffer, Rgb, RgbImage};
use log::info;

/// **Converts a 3D colored grid (`Vec<Vec<Vec<f64>>>`) into an `RgbImage`.**
///
/// - **`colored_grid`**: A `Vec<Vec<Vec<f64>>>` where each inner `Vec<f64>` represents an RGB color.
/// - **Returns**: An `RgbImage` (which can be saved separately).
///
/// **Example Usage:**
/// ```rust
/// let image = get_image_from_colored_grid(&colored_grid);
/// image.save("output.png").unwrap();
/// ```
pub fn get_image_from_colored_grid(colored_grid: &Vec<Vec<Vec<f64>>>) -> RgbImage {
    if colored_grid.is_empty() {
        info!("Empty colored grid, returning minimal image");
        return ImageBuffer::new(1, 1);
    }

    let width = colored_grid.len() as u32;
    let height = colored_grid[0].len() as u32;

    info!("Creating RgbImage from {}x{} colored grid", width, height);
    
    let mut img = ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            // Make sure we access the grid correctly based on the structure
            // from get_colored_grid()
            let color = &colored_grid[x as usize][y as usize];
            
            // Ensure values are properly scaled & clamped
            let r = (color[0]).clamp(0.0, 255.0) as u8;  // Already in 0-255 range
            let g = (color[1]).clamp(0.0, 255.0) as u8;
            let b = (color[2]).clamp(0.0, 255.0) as u8;
            
            img.put_pixel(x, y, Rgb([r, g, b]));
        }
    }

    info!("RgbImage creation complete");
    img
}

/// **Creates a WebAssembly-compatible representation of an image.**
///
/// This function converts the RgbImage into a format that can be easily 
/// used in JavaScript when exposed via WebAssembly.
///
/// - **`img`**: An `RgbImage` to convert
/// - **Returns**: A tuple with dimensions and a flat RGBA byte array
pub fn get_wasm_compatible_image(img: &RgbImage) -> (u32, u32, Vec<u8>) {
    let width = img.width();
    let height = img.height();
    
    // Create a flat RGBA buffer (with alpha=255)
    let mut rgba_data = Vec::with_capacity((width * height * 4) as usize);
    
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            rgba_data.push(pixel[0]); // R
            rgba_data.push(pixel[1]); // G
            rgba_data.push(pixel[2]); // B
            rgba_data.push(255);      // A (fully opaque)
        }
    }
    
    (width, height, rgba_data)
}