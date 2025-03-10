# Rotation

Challenge:

Given the Bhj1.mandart inputs, the SwiftUI MandArt program shows the needle pointing to the left. But in the Rust WASM version, the needle is pointing up - 90 degrees off. 

Source: get_grid_from_shape_inputs.rs


## 1. Coordinate System Differences

- SwiftUI uses a coordinate system where the origin (0,0) is at the top-left corner of the view, with the y-axis increasing downward.

- Rust (Canvas/WebGL) typically uses a coordinate system where the origin (0,0) is at the bottom-left corner, with the y-axis increasing upward.

## 2. Angle Interpretation Difference
- SwiftUI rotations are often measured clockwise from the top.
- Rust/WebGL usually measure counterclockwise from the right or left.

## Corrected Theta in Rust

let corrected_theta = -theta + std::f64::consts::FRAC_PI_2;



## Also in source file

> Why is theta only mentioned in > 3 power?

/// **Computes complex power for Mandelbrot iterations.**
fn complex_pow(base_x: f64, base_y: f64, power_real: i32) -> (f64, f64) {
    if power_real == 2 {
        let x_temp = base_x * base_x - base_y * base_y;        let new_y = 2.0 * base_x * base_y;
        (x_temp, new_y)
    } else if power_real == 3 {
        let x_squared = base_x * base_x;
        let y_squared = base_y * base_y;

        let x_temp = (x_squared - 3.0 * y_squared) * base_x;
        let new_y = (3.0 * x_squared - y_squared) * base_y;
        (x_temp, new_y)
    } else {
        let r = (base_x * base_x + base_y * base_y).sqrt();
        let theta = base_y.atan2(base_x);
        let new_r = r.powi(power_real);
        let new_theta = (power_real as f64) * theta;

        let new_x = new_r * new_theta.cos();
        let new_y = new_r * new_theta.sin();
        (new_x, new_y)
    }
}