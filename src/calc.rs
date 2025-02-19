// calc.rs - calculate the grid, or recolor, or whatever

use crate::inputs::ArtImageShapeInputs;


/// Calculates the Mandelbrot grid and returns the result as a 2D vector of doubles.
pub fn calculate_grid(shape_inputs: &ArtImageShapeInputs) -> Vec<Vec<f64>> {
    let image_width = shape_inputs.image_width as usize;
    let image_height = shape_inputs.image_height as usize;

    // Initialize the grid
    let mut f_iter = vec![vec![0.0; image_height]; image_width];  //  column-major [width][height]
    // outer loop is rows, inner loop is columns
    // rows correspond to height, columns correspond to width
    // access as r,c like excel or u,v in mandmath

    let iterations_max = shape_inputs.iterations_max;
    let scale = shape_inputs.scale;
    let x_center = shape_inputs.x_center;
    let y_center = shape_inputs.y_center;

    let pi = std::f64::consts::PI;
    let theta_r = pi * shape_inputs.theta / 180.0;

    let r_sq_limit = shape_inputs.r_sq_limit;
    let r_sq_max = (r_sq_limit.sqrt().powi(2) + 2.0).powi(2);
    let g_gml = (r_sq_max.ln().ln()) - (r_sq_limit.ln().ln());
    let g_gl = r_sq_limit.ln().ln();

    println!("rSqMax = {}", r_sq_max);

    for u in 0..image_width {  // Outer loop over columns (width)
        for v in 0..image_height {  // Inner loop over rows (height)
            let d_x = (u as f64 - (image_width / 2) as f64) / scale;
            let d_y = (v as f64 - (image_height / 2) as f64) / scale;

            let x0 = x_center + d_x * theta_r.cos() - d_y * theta_r.sin();
            let y0 = y_center + d_x * theta_r.sin() + d_y * theta_r.cos();

            let mut xx = x0;
            let mut yy = y0;
            let mut r_sq = xx * xx + yy * yy;
            let mut iter = 0.0;

            if shape_inputs.mand_power_real == 2 {
                let p = ((xx - 0.25).powi(2) + yy.powi(2)).sqrt();
                let test1 = p - 2.0 * p.powi(2) + 0.25;
                let test2 = (xx + 1.0).powi(2) + yy.powi(2);

                if xx < test1 || test2 < 0.0625 {
                    f_iter[v][u] = iterations_max;
                    iter = iterations_max;
                } else {
                    for i in 1..=(iterations_max as usize) {
                        if r_sq >= r_sq_limit {
                            break;
                        }

                        let x_temp = xx * xx - yy * yy + x0;
                        yy = 2.0 * xx * yy + y0;
                        xx = x_temp;
                        r_sq = xx * xx + yy * yy;
                        iter = i as f64;
                    }
                }
            } else {
                for i in 1..=(iterations_max as usize) {
                    if r_sq >= r_sq_limit {
                        break;
                    }

                    let (new_x, new_y) = complex_pow(xx, yy, shape_inputs.mand_power_real);
                    xx = new_x + x0;
                    yy = new_y + y0;
                    r_sq = xx * xx + yy * yy;
                    iter = i as f64;
                }
            }

            if iter < iterations_max {
                let safe_r_sq = r_sq.max(1.00001); // Avoid ln of 0 or negative
                let d_iter = -((safe_r_sq.ln().ln() - g_gl) / g_gml);
                f_iter[u][v] = iter + d_iter;  
            } else {
                f_iter[u][v] = iter; 
            }
        }
    }

    f_iter
}

/// Computes the complex power for the Mandelbrot set
fn complex_pow(base_x: f64, base_y: f64, power_real: i32) -> (f64, f64) {
    if power_real == 2 {
        let x_temp = base_x * base_x - base_y * base_y;
        let new_y = 2.0 * base_x * base_y;
        return (x_temp, new_y);
    } else if power_real == 3 {
        let x_squared = base_x * base_x;
        let y_squared = base_y * base_y;

        let x_temp = (x_squared - 3.0 * y_squared) * base_x;
        let new_y = (3.0 * x_squared - y_squared) * base_y;
        return (x_temp, new_y);
    } else {
        let r = (base_x * base_x + base_y * base_y).sqrt();
        let theta = base_y.atan2(base_x);
        let new_r = r.powi(power_real);
        let new_theta = (power_real as f64) * theta;

        let new_x = new_r * new_theta.cos();
        let new_y = new_r * new_theta.sin();
        return (new_x, new_y);
    }
}

