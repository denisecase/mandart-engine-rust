//! `lib.rs` - Core module entry point for the Mandelbrot engine.

pub mod config_settings; // ⚙️ Handles configuration settings
pub mod flatten; // flattens Rust arrays to better match JS arrays
pub mod get_colored_grid; // 🎨 Applies color transformations to a grid.
pub mod get_grid_from_shape_inputs; // 🔢 Computes the Mandelbrot grid from shape inputs.
pub mod get_image_from_colored_grid; // 🖼️ Converts a colored grid into an image.
pub mod get_inputs_from_picdef_string; // 📥 Extracts shape & color inputs from PicDef JSON.
pub mod get_picdef_from_mandart_file; // 📂 Reads `.mandart` files and extracts PicDef JSON.
pub mod io_csv; // 📑 Reads & writes grids in CSV format.
pub mod io_json; // 🖼️ Reads & write JSON to/from disk. 
pub mod io_png; // 🖼️ Handles saving images in PNG format.
pub mod io_utils; // 🔍 Utility functions (file listing, etc.)
pub mod load_or_compute_default_grid; // 🔄 Loads or computes the default Mandelbrot grid.

pub use config_settings::load_config;
pub use flatten::{flatten_colored_grid, flatten_grid, flatten_image_definition, unflatten_grid, unflatten_colored_grid,unflatten_image_definition};
pub use get_colored_grid::get_colored_grid;
pub use get_grid_from_shape_inputs::get_grid_from_shape_inputs;
pub use get_image_from_colored_grid::get_image_from_colored_grid;
pub use get_inputs_from_picdef_string::{
    get_color_inputs_from_picdef_string, get_shape_inputs_from_picdef_string,
};
pub use get_picdef_from_mandart_file::get_picdef_from_mandart_file;
pub use io_csv::{
    read_colored_grid_from_csv, read_grid_from_csv, save_colored_grid_to_csv, save_grid_to_csv,
};
pub use io_json::{read_json_file, write_json_file};
pub use io_png::save_image_to_png;
pub use io_utils::list_files_in_dir;
pub use load_or_compute_default_grid::load_or_compute_default_grid;
