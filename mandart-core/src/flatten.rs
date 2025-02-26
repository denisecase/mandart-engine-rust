
/// Flattens a `Vec<Vec<f64>>` iteration grid into a `Vec<f64>` for WASM.
pub fn flatten_grid(grid: Vec<Vec<f64>>) -> Vec<f64> {
    grid.into_iter().flatten().collect()
}

/// Flattens a `Vec<Vec<String>>` colored grid into a `Vec<String>` for WASM.
/// Each color is stored as `"#RRGGBB"`.
pub fn flatten_colored_grid(grid: Vec<Vec<String>>) -> Vec<String> {
    grid.into_iter().flatten().collect()
}


/// Flattens a `Vec<Vec<[f64; 3]>>` into a `Vec<f64>` for WASM.
/// Each pixel is stored as 3 sequential values: `[r, g, b]`.
pub fn flatten_image_definition(colored_grid: Vec<Vec<[f64; 3]>>) -> Vec<f64> {
    let mut flat_vec = Vec::with_capacity(colored_grid.len() * colored_grid[0].len() * 3);
    for row in colored_grid {
        for pixel in row {
            flat_vec.extend_from_slice(&pixel); // Push R, G, B floats
        }
    }
    flat_vec
}

/// Unflattens a 1D Vec<f64> back into a 2D Vec<Vec<f64>>.
/// 
/// - `flat_grid`: Linear array of values
/// - `width`: Number of columns
/// - `height`: Number of rows
pub fn unflatten_grid(flat_grid: Vec<f64>, width: usize, height: usize) -> Vec<Vec<f64>> {
    if flat_grid.is_empty() || width == 0 || height == 0 {
        return vec![vec![0.0; 0]; 0];
    }
    
    let mut grid = vec![vec![0.0; height]; width];
    
    for idx in 0..flat_grid.len() {
        let x = idx % width;
        let y = idx / width;
        
        if y < height {
            grid[x][y] = flat_grid[idx];
        }
    }
    
    grid
}

/// Unflattens a Vec<String> back into a Vec<Vec<String>>.
/// 
/// - `flat_colored_grid`: Linear array of color strings
/// - `width`: Number of columns
/// - `height`: Number of rows
pub fn unflatten_colored_grid(flat_colored_grid: Vec<String>, width: usize, height: usize) -> Vec<Vec<String>> {
    if flat_colored_grid.is_empty() || width == 0 || height == 0 {
        return vec![vec![String::new(); 0]; 0];
    }
    
    let mut grid = vec![vec![String::new(); height]; width];
    
    for idx in 0..flat_colored_grid.len() {
        let x = idx % width;
        let y = idx / width;
        
        if y < height {
            grid[x][y] = flat_colored_grid[idx].clone();
        }
    }
    
    grid
}

/// Unflattens a Vec<f64> back into a Vec<Vec<[f64; 3]>>.
/// Each pixel is represented by 3 sequential values: [r, g, b].
/// 
/// - `flat_image`: Linear array of RGB values
/// - `width`: Number of columns
/// - `height`: Number of rows
pub fn unflatten_image_definition(flat_image: Vec<f64>, width: usize, height: usize) -> Vec<Vec<[f64; 3]>> {
    if flat_image.len() < 3 || width == 0 || height == 0 {
        return vec![vec![[0.0, 0.0, 0.0]; 0]; 0];
    }
    
    let mut grid = vec![vec![[0.0, 0.0, 0.0]; height]; width];
    
    for pixel_idx in 0..(flat_image.len() / 3) {
        let x = pixel_idx % width;
        let y = pixel_idx / width;
        
        if y < height && pixel_idx * 3 + 2 < flat_image.len() {
            grid[x][y] = [
                flat_image[pixel_idx * 3],
                flat_image[pixel_idx * 3 + 1],
                flat_image[pixel_idx * 3 + 2]
            ];
        }
    }
    
    grid
}