#!/usr/bin/env cargo-script

//! ```cargo
//! [dependencies]
//! prettytable-rs = "^0.10"
//! serde_json = "1.0"
//! serde = { version = "1", features = ["derive"] }
//! ```

use std::{env, fs};

use prettytable::row;
use serde::Deserialize;
use serde_json::from_str;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Benchmark {
  id: String,
  mean: Mean,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Mean {
  estimate: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Get command-line arguments
  let args: Vec<String> = env::args().collect();

  // Check if two file paths are provided
  if args.len() != 3 {
    eprintln!("Usage: {} <old_file_path> <new_file_path>", args[0]);
    std::process::exit(1);
  }

  // Extract file paths from command-line arguments
  let old_file_path = &args[1];
  let new_file_path = &args[2];

  let old_content = fs::read_to_string(old_file_path)?;
  let old_benchmarks: Vec<Benchmark> = old_content.lines().filter_map(|line| from_str(line).ok()).collect();

  let new_content = fs::read_to_string(new_file_path)?;
  let new_benchmarks: Vec<Benchmark> = new_content.lines().filter_map(|line| from_str(line).ok()).collect();

  if old_benchmarks.len() != new_benchmarks.len() {
    return Err("Mismatch in the number of benchmarks between old and new files".into());
  }

  // Specify the output file path
  let output_file_path = "benches/benchmark.md";

  // Generate the comparison table and write it to the output file
  let comparison_table = generate_comparison_table(&old_benchmarks, &new_benchmarks)?;
  fs::write(output_file_path, comparison_table)?;

  // Check for benchmarks exceeding the 10% change threshold
  let benchmarks_exceeding_threshold: Vec<_> = old_benchmarks
    .iter()
    .zip(new_benchmarks.iter())
    .filter_map(|(old, new)| {
      let percentage_change = calculate_percentage_change(old.mean.estimate, new.mean.estimate);
      if percentage_change.abs() > 10.0 {
        Some(old.id.clone())
      } else {
        None
      }
    })
    .collect();

  // If there are benchmarks exceeding the threshold, print their names
  if !benchmarks_exceeding_threshold.is_empty() {
    let exceeding_benchmarks_str = benchmarks_exceeding_threshold.join(", ");
    println!(
      "Benchmarks exceeding the 10% change threshold: {}",
      exceeding_benchmarks_str
    );
  }

  Ok(())
}

fn generate_comparison_table(
  old_benchmarks: &[Benchmark],
  new_benchmarks: &[Benchmark],
) -> Result<String, Box<dyn std::error::Error>> {
  let mut comparison_table = prettytable::Table::new();
  comparison_table.add_row(row!["Benchmark", "Base ", "Change", "Percentage Change"]);

  for (old, new) in old_benchmarks.iter().zip(new_benchmarks) {
    let old_estimate = old.mean.estimate;
    let new_estimate = new.mean.estimate;
    let percentage_change = calculate_percentage_change(old_estimate, new_estimate);

    comparison_table.add_row(row![
      old.id,
      format!("{:.2}", old_estimate),
      format!("{:.2}", new_estimate),
      format!("{:.2}%", percentage_change),
    ]);
  }

  // Use `to_string` to convert the table to a string
  Ok(comparison_table.to_string())
}

fn calculate_percentage_change(old_value: f64, new_value: f64) -> f64 {
  if old_value == 0.0 {
    0.0
  } else {
    ((new_value - old_value) / old_value) * 100.0
  }
}
