#[cfg(test)]
mod tests {
    use mandart_core::io_csv::{save_grid_to_csv, read_grid_from_csv};
    use std::fs;

    /// **Test CSV I/O operations using a small test grid.**
    #[test]
    fn test_io_csv_operations() {
        let test_grid = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
        ];
        let test_file = "test_grid.csv";

        // ✅ Save the test grid as a CSV
        save_grid_to_csv(&test_grid, test_file).expect("Failed to save CSV");

        // ✅ Load the CSV back into a grid
        let loaded_grid = read_grid_from_csv(test_file).expect("Failed to read CSV");

        // ✅ Ensure the loaded grid matches the original
        assert_eq!(loaded_grid, test_grid, "Loaded grid does not match original data!");

        // ✅ Clean up test file
        fs::remove_file(test_file).expect("Failed to remove test CSV");
    }
}
