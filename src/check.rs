// check.rs - compare results

use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let output_folder = "output";
    let check_folder = "input_swift";

    let files_to_check = vec!["AAA1", "AAA2", "AAA3", "AAA5", "AAA7", "AAA10"];

    compare_a_list_of_csv_files(output_folder, check_folder, &files_to_check);
}

/// Reads a CSV file and returns its contents as a vector of vectors of strings.
fn read_csv(file_path: &str) -> io::Result<Vec<Vec<String>>> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut content = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let row: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
        content.push(row);
    }

    Ok(content)
}

/// Compares two CSV files.
fn compare_csv_files(file1: &str, file2: &str) -> io::Result<()> {
    let content1 = read_csv(file1)?;
    let content2 = read_csv(file2)?;

    let row_count1 = content1.len();
    let row_count2 = content2.len();
    let col_count1 = content1.first().map_or(0, |row| row.len());
    let col_count2 = content2.first().map_or(0, |row| row.len());

    let first_five_1 = content1.first().map_or(vec![], |row| row.iter().take(5).cloned().collect());
    let first_five_2 = content2.first().map_or(vec![], |row| row.iter().take(5).cloned().collect());

    if row_count1 == row_count2 && col_count1 == col_count2 && first_five_1 == first_five_2 {
        println!("✅ Files match: {} == {}", file1, file2);
    } else {
        println!("❌ Files DO NOT match: {} != {}", file1, file2);
        println!("---------------------------------------------");
        println!("📊 Image WIDTH:  Row Count:    {} vs {} in Swift", row_count1, row_count2);
        println!("📏 Image HEIGHT: Column Count: {} vs {} in Swift", col_count1, col_count2);
        println!("🔍 First 5 Cells:");
        println!("  {}: {:?}", file1, first_five_1);
        println!("  {}: {:?}", file2, first_five_2);
        println!("---------------------------------------------");
    }

    Ok(())
}

/// Compares a list of specific files by name.
pub fn compare_a_list_of_csv_files(output_folder: &str, check_folder: &str, file_names: &[&str]) {
    for file in file_names {
        let rust_csv = format!("{}/{}.csv", output_folder, file);
        let swift_csv = format!("{}/{}.csv", check_folder, file);

        if Path::new(&rust_csv).exists() && Path::new(&swift_csv).exists() {
            let _ = compare_csv_files(&rust_csv, &swift_csv);
        } else {
            println!("⚠️ Missing file: {} or {}", rust_csv, swift_csv);
        }
    }
}
