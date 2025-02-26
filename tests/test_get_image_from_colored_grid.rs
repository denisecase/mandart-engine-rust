#[cfg(test)]
mod tests {
    use mandart_core::get_image_from_colored_grid;

    #[test]
    fn test_colored_grid_to_image() {
        // Create a test grid in the new 3D format
        let mut test_grid = vec![vec![vec![0.0, 0.0, 0.0]; 2]; 2];
        
        // Add some colors
        test_grid[0][0] = vec![255.0, 0.0, 0.0];      // Red
        test_grid[0][1] = vec![0.0, 255.0, 0.0];      // Green
        test_grid[1][0] = vec![0.0, 0.0, 255.0];      // Blue
        test_grid[1][1] = vec![255.0, 255.0, 255.0];  // White
        
        let image = get_image_from_colored_grid(&test_grid);
        
        // Assertions remain the same
        assert_eq!(image.width(), 2);
        assert_eq!(image.height(), 2);
        // Add color checks as needed
    }
}
