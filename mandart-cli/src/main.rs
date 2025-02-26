//! `main.rs` - CLI Test Application for MandArt Engine

use log::{error, info, warn};
use mandart_core::config_settings::load_config;
use mandart_core::get_colored_grid::get_colored_grid;
use mandart_core::get_grid_from_shape_inputs::get_grid_from_shape_inputs;
use mandart_core::get_image_from_colored_grid::get_image_from_colored_grid;
use mandart_core::get_inputs_from_picdef_string::{
    get_color_inputs_from_picdef_string, get_shape_inputs_from_picdef_string,
};
use mandart_core::get_picdef_from_mandart_file::get_picdef_from_mandart_file;
use mandart_core::io_csv::save_grid_to_csv; 
use mandart_core::io_png::save_image_to_png;
use mandart_core::io_utils::list_files_in_dir;
use std::env;
use std::fs;
use std::path::Path;

/// **Entry point for the CLI application.**
fn main() {
    setup_logging();
    process_mandart_files();
}

/// **Initialize logging for CLI applications.**
pub fn setup_logging() {
    use std::sync::Once;
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        env_logger::init();
    });
}

/// **Process all `.mandart` files in a folder and generate images & CSV grids.**
pub fn process_mandart_files() {
    let args: Vec<String> = env::args().collect();
    let config_file = args.get(1).map(String::as_str);
    let config = load_config(config_file);

    // Use config values or default to `"input"` and `"output"`
    let input_folder = config
        .get("input_folder")
        .map_or_else(|| "input".to_string(), Clone::clone);
    let output_folder = config
        .get("output_folder")
        .map_or_else(|| "output".to_string(), Clone::clone);

    info!("📂 Processing MandArt files from `{}`...", input_folder);

    let mut mandart_files = list_files_in_dir(&input_folder, ".mandart");
    if mandart_files.is_empty() {
        warn!("⚠️ No `.mandart` files found in `{}`.", input_folder);
        return;
    }

    // 🔹 **Sort files alphabetically**
    mandart_files.sort();

    if let Err(e) = fs::create_dir_all(&output_folder) {
        error!(
            "❌ Failed to create output directory `{}`: {}",
            output_folder, e
        );
        return;
    }

    for file in mandart_files {
        process_single_mandart_file(&file, &output_folder);
    }

    info!("🎉 Done! Files saved in `{}`.", output_folder);
}

/// **Process a single `.mandart` file, generate a grid, and save it.**
fn process_single_mandart_file(file: &str, output_folder: &str) {
    info!("📄 Processing `{}`...", file);
    let file_path = Path::new(file);

    let file_stem = match file_path.file_stem() {
        Some(f) => f.to_string_lossy().to_string(),
        None => {
            warn!("⚠️ Skipping `{}` due to missing filename.", file);
            return;
        }
    };

    // 1️⃣ Extract the **PicDef JSON** from the `.mandart` file
    let picdef_string = match get_picdef_from_mandart_file(file) {
        Ok(picdef) => picdef,
        Err(e) => {
            error!(
                "❌ Failed to extract **PicDef JSON** from `{}`: {}",
                file, e
            );
            return;
        }
    };

    // 2️⃣ Extract **shape inputs** from the PicDef JSON
    let shape_inputs = match get_shape_inputs_from_picdef_string(&picdef_string) {
        Ok(inputs) => inputs,
        Err(e) => {
            error!(
                "❌ Failed to extract **shape inputs** from `{}`: {}",
                file, e
            );
            return;
        }
    };

    // 3️⃣ Compute the **Mandelbrot iteration grid** from shape inputs
    info!("🔢 Calculating Mandelbrot grid for `{}`...", file);
    let grid = get_grid_from_shape_inputs(&shape_inputs);

    // 4️⃣ Save **2D iteration grid** to CSV
    let csv_grid_output_path = format!("{}/{}.csv", output_folder, file_stem);
    if let Err(e) = save_grid_to_csv(&grid, &csv_grid_output_path) {
        error!(
            "❌ Failed to save grid CSV `{}`: {}",
            csv_grid_output_path, e
        );
    } else {
        info!("✅ Grid CSV saved: `{}`", csv_grid_output_path);
    }

    // 5️⃣ Extract **color inputs** from the PicDef JSON
    let color_inputs = match get_color_inputs_from_picdef_string(&picdef_string) {
        Ok(inputs) => inputs,
        Err(e) => {
            error!(
                "❌ Failed to extract **color inputs** from `{}`: {}",
                file, e
            );
            return;
        }
    };

    info!("🎨 Applying color mapping for `{}`...", file);
    let colored_grid = get_colored_grid(&grid, &shape_inputs, &color_inputs);
    info!("✅ Colored grid generated for `{}`.", file);

    let image = get_image_from_colored_grid(&colored_grid);
    let png_output_path = format!("{}/{}.png", output_folder, file_stem);
    if let Err(e) = save_image_to_png(&image, &png_output_path) {
        error!("❌ Failed to save PNG `{}`: {}", png_output_path, e);
    } else {
        info!("✅ PNG saved: `{}`", png_output_path);
    }
}