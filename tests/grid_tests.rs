// //! `tests/grid_tests.rs` - Unit tests for grid computation

// use mandart_engine_rust::grid::{
//     get_grid_from_shape_inputs, get_grid_from_mandart_json_string, get_grid_from_mandart_file
// };
// use tempfile::tempdir;
// use std::fs;

// #[test]
// pub fn test_grid_generation() {
//     let shape_json = r#"{
//         "image_width": 10,
//         "image_height": 10,
//         "iterations_max": 100.0,
//         "scale": 1.0,
//         "x_center": 0.0,
//         "y_center": 0.0,
//         "theta": 0.0,
//         "r_sq_limit": 4.0,
//         "mand_power_real": 2
//     }"#;

//     let result = get_grid_from_shape_inputs(shape_json);
//     assert!(result.is_ok(), "Grid computation failed");

//     let grid = result.unwrap(); 
    
//     assert_eq!(grid.len(), 10, "Grid width mismatch");
//     assert_eq!(grid[0].len(), 10, "Grid height mismatch");
// }

// #[test]
// pub fn test_grid_from_mandart_json() {
//     let mandart_json = r#"{
//         "grid": [[0, 1, 2], [3, 4, 5], [6, 7, 8]]
//     }"#;

//     let result = get_grid_from_mandart_json_string(mandart_json);
//     assert!(result.is_ok(), "Failed to compute grid from mandart JSON");

//     let grid = result.unwrap(); 

//     assert_eq!(grid.len(), 3, "Grid width mismatch");
//     assert_eq!(grid[0].len(), 3, "Grid height mismatch");
//     assert_eq!(grid[1][1], 4.0, "Grid value mismatch");
// }


// #[test]
// pub fn test_grid_from_mandart_file() {
//     let dir = tempdir().unwrap();
//     let file_path = dir.path().join("test.mandart");

//     let mandart_data = r#"{
//         "grid": [[0, 2, 4], [6, 8, 10], [12, 14, 16]]
//     }"#;

//     fs::write(&file_path, mandart_data).expect("Failed to write test .mandart file");

//     let result = get_grid_from_mandart_file(file_path.to_str().unwrap());
//     assert!(result.is_ok(), "Failed to compute grid from mandart file");

//     let grid = result.unwrap(); 

//     assert_eq!(grid.len(), 3, "Grid width mismatch");
//     assert_eq!(grid[0].len(), 3, "Grid height mismatch");
//     assert_eq!(grid[2][2], 16.0, "Grid value mismatch");
// }
