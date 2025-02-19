//! `tests/file_tests.rs` - Unit tests for file I/O operations

use mandart_engine_rust::file_io::{
    read_grid_from_csv, read_image_as_base64, save_grid_to_csv, save_image_to_png,
};

use tempfile::tempdir;

#[test]
pub fn test_save_and_read_grid_csv() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_grid.csv");

    let grid: Vec<Vec<f64>> = vec![vec![0.0, 1.0, 2.0], vec![3.0, 4.0, 5.0]];

    assert!(
        save_grid_to_csv(&grid, file_path.to_str().unwrap()).is_ok(),
        "Failed to save CSV"
    );

    let read_result = read_grid_from_csv(file_path.to_str().unwrap());
    assert!(read_result.is_ok(), "Failed to read CSV");

    let read_csv = read_result.unwrap();
    let parsed_read_result: Vec<Vec<f64>> = read_csv
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<f64>().expect("Failed to parse CSV float"))
                .collect()
        })
        .collect();

    assert_eq!(parsed_read_result, grid, "Grid CSV content mismatch");
}

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
