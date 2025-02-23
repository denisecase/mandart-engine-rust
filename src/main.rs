//! `main.rs` - CLI Test Application for MandArt Engine

use mandart_engine_rust::config::load_config;
use mandart_engine_rust::file_io::{save_image_to_bmp, save_image_to_png};
use mandart_engine_rust::image::get_image_from_mandart_file;
use mandart_engine_rust::utils::list_files_in_dir;
use std::env;
use std::fs;
use std::path::Path;
use log::{info, warn, error};

/// Entry point for the CLI application.
fn main() {
    setup_logging();
    process_mandart_files();
}

/// Initialize logging for CLI and WebAssembly environments.
pub fn setup_logging() {
    use std::sync::Once;
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        #[cfg(feature = "wasm")]
        {
            use log::Level;
            console_log::init_with_level(Level::Debug).expect("Failed to initialize console_log.");
            console_error_panic_hook::set_once();
        }

        #[cfg(not(feature = "wasm"))]
        {
            env_logger::init();
        }
    });
}

/// Process all `.mandart` files in a folder and generate images.
pub fn process_mandart_files() {
    let args: Vec<String> = env::args().collect();
    let config_file = args.get(1).map(String::as_str);
    let config = load_config(config_file);

    // Use config values or default to `"input"` and `"output"`
    let input_folder = config.get("input_folder").map_or_else(|| "input".to_string(), Clone::clone);
    let output_folder = config.get("output_folder").map_or_else(|| "output".to_string(), Clone::clone);

    info!("\u{1F4C2} Processing MandArt files from `{}`...", input_folder);

    let mut mandart_files = list_files_in_dir(&input_folder, ".mandart");
  
    if mandart_files.is_empty() {
        warn!("⚠️ No .mandart files found in `{}`.", input_folder);
        return;
    }

    mandart_files.sort();

    if let Ok(metadata) = fs::metadata(&output_folder) {  // ✅ Borrow `output_folder`
        if metadata.is_file() {
            error!("❌ Output path `{}` exists but is a file, not a directory.", output_folder);
            return;
        }
    }
    
    if let Err(e) = fs::create_dir_all(&output_folder) {  // ✅ Borrow `output_folder`
        error!("❌ Failed to create output directory `{}`: {}", output_folder, e);
        return;
    }
    

    for file in &mandart_files {
        info!("📄 Processing file: {}", file);
        let file_path = Path::new(file);

        let file_stem = match file_path.file_stem() {
            Some(f) => f.to_string_lossy().to_string(),
            None => {
                warn!("⚠️ Skipping file `{}` due to missing filename.", file);
                continue;
            }
        };

        match get_image_from_mandart_file(file) {
            Ok(image_grid) => { 
                let bmp_output_path = format!("{}/{}.bmp", output_folder, file_stem);
                let png_output_path = format!("{}/{}.png", output_folder, file_stem);

                if let Err(e) = save_image_to_bmp(&image_grid, &bmp_output_path) {
                    error!("❌ Failed to save BMP {}: {}", bmp_output_path, e);
                } else {
                    info!("✅ BMP saved: {}", bmp_output_path);
                }

                if let Err(e) = save_image_to_png(&image_grid, &png_output_path) {
                    error!("❌ Failed to save PNG {}: {}", png_output_path, e);
                } else {
                    info!("✅ PNG saved: {}", png_output_path);
                }
            }
            Err(e) => error!("❌ Error processing `{}`: {}", file, e),
        }
    }

    info!("🎉 Done! Images saved in `{}`.", output_folder);
}
