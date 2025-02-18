// outputs.rs - generate various output files (csv, bmp, png, etc)

use std::fs::File;
use std::io::{self, Write, Result};
use image::{Rgb, RgbImage};
use crate::inputs::{ArtImageShapeInputs, ArtImageColorInputs};  // Ensure both are imported

/// Writes a grid to a CSV file in width-first order.
pub fn write_grid_to_csv(grid: &Vec<Vec<f64>>, output_file: &str) -> io::Result<()> {
    let mut file = File::create(output_file)?;

    for row in grid {
        let line = row.iter()
            .map(|val| val.to_string())
            .collect::<Vec<String>>()
            .join(",");
        writeln!(file, "{}", line)?;
    }

    Ok(())
}



/// Colors the Mandelbrot grid using iteration values and saves as BMP.
pub fn color_grid(
    f_iter: &Vec<Vec<f64>>, 
    shape_inputs: &ArtImageShapeInputs, 
    color_inputs: &ArtImageColorInputs, 
    output_file: &str
) -> io::Result<()> {

    let image_width = f_iter.len();
    let image_height = f_iter[0].len();

    let iterations_max = shape_inputs.iterations_max;
    let spacing_color_far = color_inputs.spacing_color_far;
    let spacing_color_near = color_inputs.spacing_color_near;
    let n_blocks = color_inputs.n_blocks as usize;
    let n_colors = color_inputs.n_colors as usize;
    let hues_list = &color_inputs.hues_list; // Color palette

    // Compute color block boundaries
    let mut block_bound = vec![0.0; n_blocks + 1];
    let spacing_color_mid = (iterations_max - spacing_color_far * n_blocks as f64)
        / (n_blocks as f64).powf(spacing_color_near);

    for i in 0..=n_blocks {
        block_bound[i] = spacing_color_far * i as f64 + spacing_color_mid * (i as f64).powf(spacing_color_near);
    }

    let mut img = RgbImage::new(image_width as u32, image_height as u32);

    for u in 0..image_width {
        for v in 0..image_height {
            let iter_value = f_iter[u][v];

            let mut pixel = Rgb([0, 0, 0]);

            if iter_value < iterations_max {
                let mut h = iter_value;
                for block in 0..n_blocks {
                    if h >= block_bound[block] && h < block_bound[block + 1] {
                        let x_x = (h - block_bound[block]) / (block_bound[block + 1] - block_bound[block]);

                        let block0 = block % n_colors;
                        let block1 = (block0 + 1) % n_colors;

                        let r = hues_list[block0][1] + x_x * (hues_list[block1][1] - hues_list[block0][1]);
                        let g = hues_list[block0][2] + x_x * (hues_list[block1][2] - hues_list[block0][2]);
                        let b = hues_list[block0][3] + x_x * (hues_list[block1][3] - hues_list[block0][3]);

                        pixel = Rgb([r as u8, g as u8, b as u8]); // Assign color
                        break; // Exit loop after assigning color
                    }
                }
            }

            img.put_pixel(u as u32, v as u32, pixel);
        }
    }

    img.save(output_file).expect("Failed to save BMP file");
    println!("✅ BMP saved to {}", output_file);
    Ok(())
}
