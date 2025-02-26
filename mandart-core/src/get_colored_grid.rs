//! `src/image.rs` - Handles Image Generation and Coloring

use crate::get_inputs_from_picdef_string::{
    ArtImageShapeInputs, ArtImageColorInputs,
};
use log::info;

/// Colors the grid based on `ArtImageColorInputs`.
pub fn get_colored_grid(
    iter_grid: &Vec<Vec<f64>>, 
    shape_inputs: &ArtImageShapeInputs, 
    color_inputs: &ArtImageColorInputs
) -> Vec<Vec<Vec<f64>>> {
    info!("Coloring the iteration grid...");
    
    // First, determine the correct dimensions of the grid
    let grid_height = iter_grid.len();
    if grid_height == 0 {
        info!("Error: iter_grid is empty");
        return vec![vec![vec![0.0; 3]; 1]; 1]; // Return a minimal grid
    }
    
    let grid_width = iter_grid[0].len();
    
    info!("Grid dimensions: {}x{}", grid_width, grid_height);
    
    // Create a 3D grid with RGB color values
    // Use the same dimensions as the input grid for consistency
    let mut color_grid = vec![vec![vec![0.0; 3]; grid_width]; grid_height];
    
    // Calculate the min iteration value
    let mut f_iter_min = f64::INFINITY;
    
    // Find the minimum value in the entire grid
    for u in 0..grid_height {
        for v in 0..grid_width {
            if iter_grid[u][v] < f_iter_min && iter_grid[u][v] > 0.0 {
                f_iter_min = iter_grid[u][v];
            }
        }
    }
    
    // Apply the dFIterMin adjustment
    f_iter_min -= shape_inputs.d_f_iter_min;
    
    info!("Minimum iteration value: {}", f_iter_min);
    
    // Calculate color blocks
    let n_blocks = color_inputs.n_blocks as usize;
    let n_colors = color_inputs.n_colors;
    let spacing_color_far = color_inputs.spacing_color_far;
    let spacing_color_near = color_inputs.spacing_color_near;
    
    let mut y_y = color_inputs.y_y_input;
    if y_y >= 1.0 {
        y_y = 0.999999; // Avoid division by zero
    }
    
    let f_n_blocks = n_blocks as f64;
    let spacing_color_mid = (shape_inputs.iterations_max - f_iter_min - f_n_blocks * spacing_color_far) 
                           / f_n_blocks.powf(spacing_color_near);
    
    let mut block_bound = vec![0.0; n_blocks + 1];
    for i in 0..=n_blocks {
        block_bound[i] = spacing_color_far * (i as f64) + spacing_color_mid * (i as f64).powf(spacing_color_near);
    }
    
    info!("Processing colors for {} x {} grid", grid_height, grid_width);
    
    // Process each pixel
    for u in 0..grid_height {
        for v in 0..grid_width {
            if iter_grid[u][v] >= shape_inputs.iterations_max {
                // Set to mand_color (the color for points inside the Mandelbrot set)
                color_grid[u][v][0] = color_inputs.mand_color[0]; // R
                color_grid[u][v][1] = color_inputs.mand_color[1]; // G
                color_grid[u][v][2] = color_inputs.mand_color[2]; // B
            } else {
                let mut h = iter_grid[u][v] - f_iter_min;
                
                // Find which color block this belongs to
                for block in 0..n_blocks {
                    if block + 1 < block_bound.len() && 
                       h >= block_bound[block] && 
                       h < block_bound[block + 1] {
                        let mut block0 = block;
                        
                        // Apply non-linear mapping based on y_y
                        if (h - block_bound[block]) / (block_bound[block + 1] - block_bound[block]) <= y_y {
                            h = block_bound[block];
                        } else {
                            h = block_bound[block] +
                                ((h - block_bound[block]) - y_y * (block_bound[block + 1] - block_bound[block])) /
                                (1.0 - y_y);
                        }
                        
                        let x_x = (h - block_bound[block]) / (block_bound[block + 1] - block_bound[block]);
                        
                        // Wrap around if needed
                        while block0 >= n_colors {
                            block0 = block0 - n_colors;
                        }
                        
                        let mut block1 = block0 + 1;
                        if block1 >= n_colors {
                            block1 = block1 - n_colors;
                        }
                        
                        // Linear interpolation between colors
                        if block0 < color_inputs.colors.len() && block1 < color_inputs.colors.len() {
                            for c in 0..3 {
                                color_grid[u][v][c] = color_inputs.colors[block0][c] + 
                                                     x_x * (color_inputs.colors[block1][c] - color_inputs.colors[block0][c]);
                            }
                        }
                        
                        break; // Exit the block search loop once we've found our block
                    }
                }
            }
        }
    }
    
    info!("Grid coloring complete");
    color_grid
}