// main.rs

mod calc;
mod inputs;
mod outputs;

use std::fs;
use std::io::{self};
use std::path::PathBuf;

use calc::calculate_grid;
use inputs::{get_color_inputs, get_shape_inputs};
use outputs::{color_grid, write_grid_to_csv};

/// Generates a CSV filename based on the `.mandart` filename.
fn get_csv_name_from_mandart_name(output_folder: &str, mandart_name: &str) -> String {
    format!("{}/{}.csv", output_folder, mandart_name)
}

/// Generates an image filename based on the `.mandart` filename.
fn get_image_name_from_mandart_name(output_folder: &str, mandart_name: &str, ext: &str) -> String {
    format!("{}/{}.{}", output_folder, mandart_name, ext)
}

fn main() -> io::Result<()> {
    let input_folder = "input";
    let output_folder = "output";

    // Ensure output folder exists
    if !std::path::Path::new(output_folder).exists() {
        fs::create_dir(output_folder)?;
    }

    let mut mandart_files: Vec<PathBuf> = fs::read_dir(input_folder)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| path.extension().map_or(false, |ext| ext == "mandart"))
        .collect();

    mandart_files.sort(); // Process in order

    for path in &mandart_files {
        let file_name = path.file_stem().unwrap().to_str().unwrap();
        let mandart_file = path.to_str().unwrap();
        let output_csv = get_csv_name_from_mandart_name(output_folder, file_name);
        let output_bmp = get_image_name_from_mandart_name(output_folder, file_name, "bmp");

        println!("📂 Processing `{}`...", file_name);

        // Read input parameters from .mandart
        let shape_inputs = get_shape_inputs(&mandart_file)?;
        let color_inputs = get_color_inputs(&mandart_file)?;

        println!("✅ Shape Inputs: {:?}", shape_inputs);
        println!("🎨 Color Inputs: {:?}", color_inputs);

        // Generate the grid
        let grid = calculate_grid(&shape_inputs, &color_inputs);

        // Save as CSV
        write_grid_to_csv(&grid, &output_csv)?;
        println!("✅ CSV saved to {}", output_csv);

        // Save as BMP (image)
        color_grid(&grid, &shape_inputs, &color_inputs, &output_bmp)?;
        println!("🖼️ BMP image saved to {}", output_bmp);
    }

    Ok(())
}
