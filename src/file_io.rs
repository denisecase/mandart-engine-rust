// file_io.rs - Handles file input/output operations.

use std::fs;
use std::fs::File;
use std::io::Write;
use base64::engine::general_purpose;
use base64::Engine;
use image::{Rgb, RgbImage};

/// Reads a `.mandart` file and returns its contents as a JSON string.
pub fn read_mandart_file(file_path: &str) -> Result<String, String> {
    println!("🔍 Attempting to read file: `{}`", file_path);

    fs::read_to_string(file_path)
        .map_err(|_| format!("Failed to read .mandart file: {}", file_path))
}

/// Reads a CSV file containing a grid and returns its contents as a string.
pub fn read_grid_from_csv(file_path: &str) -> Result<String, String> {
    fs::read_to_string(file_path)
        .map_err(|_| format!("Failed to read CSV file: {}", file_path))
}

/// Reads an image file and returns its base64-encoded contents.
pub fn read_image_as_base64(file_path: &str) -> Result<String, String> {
    fs::read(file_path)
        .map(|img_data| general_purpose::STANDARD.encode(img_data))
        .map_err(|_| format!("Failed to read image file: {}", file_path))
}

/// Saves an image grid as a BMP file.
pub fn save_image_to_bmp(image_grid: &Vec<Vec<[f64; 3]>>, file_path: &str) -> Result<(), String> {
    let img_width = image_grid[0].len();
    let img_height = image_grid.len();

    let mut img = RgbImage::new(img_width as u32, img_height as u32);

    for v in 0..img_height {
        for u in 0..img_width {
            let color = image_grid[v][u];
            let pixel = Rgb([
                (color[0] * 255.0) as u8,  // Convert float [0.0-1.0] to u8 [0-255]
                (color[1] * 255.0) as u8,
                (color[2] * 255.0) as u8,
            ]);
            img.put_pixel(u as u32, v as u32, pixel);
        }
    }

    img.save(file_path).map_err(|e| format!("Failed to save BMP: {}", e))?;
    println!("BMP saved to `{}`", file_path);
    Ok(())
}


/// Saves an image grid as a PNG file.
pub fn save_image_to_png(image_grid: &Vec<Vec<[f64; 3]>>, file_path: &str) -> Result<(), String> {
    let img_width = image_grid[0].len();
    let img_height = image_grid.len();

    let mut img = RgbImage::new(img_width as u32, img_height as u32);

    for v in 0..img_height {
        for u in 0..img_width {
            let color = image_grid[v][u];
            let pixel = Rgb([
                (color[0] * 255.0) as u8,  
                (color[1] * 255.0) as u8,
                (color[2] * 255.0) as u8,
            ]);
            img.put_pixel(u as u32, v as u32, pixel);
        }
    }

    img.save(file_path).map_err(|e| format!("Failed to save PNG: {}", e))?;
    println!("PNG saved to `{}`", file_path);
    Ok(())
}

/// Saves a grid to a CSV file.
pub fn save_grid_to_csv(grid: &[Vec<f64>], file_path: &str) -> Result<(), String> {
    let mut file = File::create(file_path)
        .map_err(|_| format!("Failed to create CSV file: {}", file_path))?;

    for row in grid {
        let line = row.iter()
            .map(|val| format!("{:.6}", val))  // Ensures consistent float formatting
            .collect::<Vec<String>>()
            .join(",");
        writeln!(file, "{}", line)
            .map_err(|_| format!("Failed to write to CSV file: {}", file_path))?;
    }

    println!("Grid saved to `{}`", file_path);
    Ok(())
}

