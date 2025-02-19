// artist.rs - given a .mandart file, calculate the grid, save it, and color it and make an image
// this will become a wasm file to be called by mandart-web


mod calc;
mod inputs;
mod outputs;

use std::fs;
use std::io::{self};
use std::path::PathBuf;

use calc::calculate_grid;
use inputs::{get_color_inputs, get_shape_inputs};
use outputs::{color_grid, write_grid_to_csv};

// function calc_and_color given .mandart file



// function recolor given .mandart file and grid



// function to accept a mandart file and know if you need to calc_and_color or just recolor
// this is the main logic
// we have the following functions:



