//! `tests/image_tests.rs` - Unit tests for image processing

use mandart_engine_rust::file_io::{read_image_as_base64, save_image_to_png};
// use mandart_engine_rust::image::get_image_from_mandart_json_string;
use tempfile::tempdir;


// #[test]
// pub fn test_generate_image_from_mandart_json() {
//     let mandart_json = r#"{
//         "grid": [[0, 1, 2], [3, 4, 5], [6, 7, 8]],
//         "colors": [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 1.0, 0.0]]
//     }"#;

//     let result = get_image_from_mandart_json_string(mandart_json);
//     assert!(result.is_ok(), "Failed to generate image from mandart JSON");

//     let image_grid = result.unwrap();
//     assert!(!image_grid.is_empty(), "Generated image grid is empty");
// }

#[test]
pub fn test_save_and_read_image_grid() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_image.png");

    // Create a simple 2x2 image grid with RGB values in [0.0, 1.0] range
    let image_grid: Vec<Vec<[f64; 3]>> = vec![
        vec![[0.0, 1.0, 0.0], [1.0, 1.0, 0.0]], // Green, Yellow
        vec![[1.0, 0.0, 0.0], [0.0, 0.0, 1.0]], // Red, Blue
    ];

    assert!(
        save_image_to_png(&image_grid, file_path.to_str().unwrap()).is_ok(),
        "Failed to save image"
    );

    assert!(file_path.exists(), "PNG file was not created");

    let result = read_image_as_base64(file_path.to_str().unwrap());
    assert!(result.is_ok(), "Failed to read image as base64");

    let encoded_image = result.unwrap();
    assert!(!encoded_image.is_empty(), "Base64 image string is empty");
}
