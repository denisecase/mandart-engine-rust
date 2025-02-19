//! `main.rs` - CLI Test Application for MandArt Engine

use mandart_engine_rust::config::load_config;
use mandart_engine_rust::file_io::{read_mandart_file, save_image_to_bmp, save_image_to_png};
use mandart_engine_rust::image::get_image_from_mandart_file;
use mandart_engine_rust::utils::list_files_in_dir;
use std::env;
use std::fs;
use std::path::Path;


fn main() {
    process_mandart_files();
}

/// Process all `.mandart` files and generate images.
pub fn process_mandart_files() {
    let args: Vec<String> = env::args().collect();
    let config_file = args.get(1).map(String::as_str);
    let config = load_config(config_file);

    // Fix: Store the default values in variables first
    let default_input_folder = "input".to_string();
    let default_output_folder = "output".to_string();

    // Use variables instead of temporary values
    let input_folder = config.get("input_folder").unwrap_or(&default_input_folder);
    let output_folder = config.get("output_folder").unwrap_or(&default_output_folder);

    println!("\u{1F4C2} Processing MandArt files from `{}`...", input_folder);

    let mut mandart_files = list_files_in_dir(input_folder, ".mandart");
    if mandart_files.is_empty() {
        println!("⚠️ No .mandart files found in `{}`.", input_folder);
        return;
    }

    mandart_files.sort();
    fs::create_dir_all(output_folder).expect("Failed to create output directory.");

    for file in &mandart_files {
        println!("📄 Processing file: {}", file);
        let file_path = Path::new(file);
        let file_stem = file_path.file_stem()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        println!("File stem: `{}`", file_stem);

        match read_mandart_file(file) {
            Ok(_) => {
                match get_image_from_mandart_file(file) {
                    Ok(image_grid) => { 
                        let bmp_output_path = format!("{}/{}.bmp", output_folder, file_stem);
                        let png_output_path = format!("{}/{}.png", output_folder, file_stem);
        
                        match save_image_to_bmp(&image_grid, &bmp_output_path) {
                            Ok(_) => println!("BMP saved: {}", bmp_output_path),
                            Err(e) => println!("Failed to save BMP {}: {}", bmp_output_path, e),
                        }
        
                        match save_image_to_png(&image_grid, &png_output_path) {  
                            Ok(_) => println!("PNG saved: {}", png_output_path),
                            Err(e) => println!("Failed to save PNG {}: {}", png_output_path, e),
                        }
                    }
                    Err(e) => println!("Error processing `{}`: {}", file, e),
                }
            }
            Err(e) => {
                println!("Failed to read `{}`: {}", file, e);
            }
        }
    }

    println!("🎉 Done! Images saved in `{}`.", output_folder);
}
