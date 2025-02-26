use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

/// **Reads a CSV file and parses it into a `Vec<Vec<f64>>`.**
pub fn read_grid_from_csv(file_path: &str) -> io::Result<Vec<Vec<f64>>> {
    let file = File::open(file_path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("File `{}` not found: {}", file_path, e),
        )
    })?;
    let reader = BufReader::new(file);

    let mut grid = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        } // Skip empty lines
        let row: Vec<f64> = line
            .split(',')
            .filter_map(|s| s.trim().parse::<f64>().ok()) // Parse values into f64
            .collect();
        grid.push(row);
    }

    if grid.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "CSV file is empty or improperly formatted.",
        ));
    }

    Ok(grid)
}

/// **Reads a CSV file and parses it into a `Vec<Vec<[f64; 3]>>`.**
pub fn read_colored_grid_from_csv(file_path: &str) -> io::Result<Vec<Vec<[f64; 3]>>> {
    let file = File::open(file_path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("File `{}` not found: {}", file_path, e),
        )
    })?;
    let reader = BufReader::new(file);

    let mut grid = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        } // Skip empty lines
        let row: Vec<[f64; 3]> = line
            .split(',')
            .collect::<Vec<&str>>()
            .chunks(3) // Group every 3 values into RGB triplets
            .filter_map(|chunk| {
                if chunk.len() == 3 {
                    Some([
                        chunk[0].trim().parse::<f64>().ok()?,
                        chunk[1].trim().parse::<f64>().ok()?,
                        chunk[2].trim().parse::<f64>().ok()?,
                    ])
                } else {
                    None
                }
            })
            .collect();

        grid.push(row);
    }

    if grid.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "CSV file is empty or improperly formatted.",
        ));
    }

    Ok(grid)
}

/// **Saves a grid to a CSV file.**
pub fn save_grid_to_csv(grid: &[Vec<f64>], file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    for row in grid {
        let line = row
            .iter()
            .map(|val| format!("{:.6}", val)) // Ensures consistent float formatting
            .collect::<Vec<String>>()
            .join(",");
        writeln!(file, "{}", line)?;
    }

    println!("✅ 2D Grid saved to `{}`", file_path);
    Ok(())
}

/// **Saves a 3D colored grid to a CSV file with RGB values.**
pub fn save_colored_grid_to_csv(grid: &[Vec<[f64; 3]>], file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    for row in grid {
        let line = row
            .iter()
            .map(|color| format!("{:.3},{:.3},{:.3}", color[0], color[1], color[2])) // R,G,B
            .collect::<Vec<String>>()
            .join(",");
        writeln!(file, "{}", line)?;
    }

    println!("✅ 3D Colored Grid saved to `{}`", file_path);
    Ok(())
}
