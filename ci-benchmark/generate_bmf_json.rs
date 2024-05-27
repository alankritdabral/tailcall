#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! regex = "1.5"
//! ```

use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use regex::Regex;

// Struct to represent the benchmark data
#[derive(Debug, Serialize, Deserialize)]
struct BenchmarkData {
    test_name: String,
    threads: u32,
    connections: u32,
    stats: Stats,
    total_requests: u32,
    data_transferred: String,
    requests_per_second_overall: f64,
    min_latency: String,
}

// Struct to represent latency and requests per second statistics
#[derive(Debug, Serialize, Deserialize)]
struct Stats {
    latency: LatencyStats,
    requests_per_second: RequestStats,
}

#[derive(Debug, Serialize, Deserialize)]
struct LatencyStats {
    avg: String,
    stdev: String,
    max: String,
    deviation: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RequestStats {
    avg: f64,
    stdev: f64,
    max: f64,
    deviation: String,
}

// Function to convert text file data to JSON
fn convert_to_json(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Open the file
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Regular expressions to extract data
    let threads_connections_regex = Regex::new(r"(\d+) threads and (\d+) connections").unwrap();
    let latency_regex = Regex::new(r"Latency +(\S+) +(\S+) +(\S+) +(\S+)").unwrap();
    let requests_per_sec_regex = Regex::new(r"Req/Sec +(\S+) +(\S+) +(\S+) +(\S+)").unwrap();
    let total_requests_regex = Regex::new(r"(\d+) requests in (\S+), (\S+) read").unwrap();
    let requests_per_sec_overall_regex = Regex::new(r"Requests/sec: +(\S+)").unwrap();

    let mut benchmark_data = BenchmarkData {
        test_name: "ci-benchmark".to_string(),
        threads: 0,
        connections: 0,
        stats: Stats {
            latency: LatencyStats {
                avg: String::new(),
                stdev: String::new(),
                max: String::new(),
                deviation: String::new(),
            },
            requests_per_second: RequestStats {
                avg: 0.0,
                stdev: 0.0,
                max: 0.0,
                deviation: String::new(),
            },
        },
        total_requests: 0,
        data_transferred: String::new(),
        requests_per_second_overall: 0.0,
        min_latency: String::new(),
    };

    // Process each line in the file
    for line in reader.lines() {
        let line = line?;
        if let Some(captures) = threads_connections_regex.captures(&line) {
            benchmark_data.threads = captures[1].parse().unwrap_or_default();
            benchmark_data.connections = captures[2].parse().unwrap_or_default();
        } else if let Some(captures) = latency_regex.captures(&line) {
            benchmark_data.stats.latency.avg = captures[1].to_string();
            benchmark_data.stats.latency.stdev = captures[2].to_string();
            benchmark_data.stats.latency.max = captures[3].to_string();
            benchmark_data.stats.latency.deviation = captures[4].to_string();
        } else if let Some(captures) = requests_per_sec_regex.captures(&line) {
            benchmark_data.stats.requests_per_second.avg = parse_requests_per_second(&captures[1]);
            benchmark_data.stats.requests_per_second.stdev = captures[2].parse().unwrap_or_default();
            benchmark_data.stats.requests_per_second.max = parse_requests_per_second(&captures[3]);
            benchmark_data.stats.requests_per_second.deviation = captures[4].to_string();
        } else if let Some(captures) = total_requests_regex.captures(&line) {
            benchmark_data.total_requests = captures[1].parse().unwrap_or_default();
            benchmark_data.data_transferred = captures[3].to_string();
        } else if let Some(captures) = requests_per_sec_overall_regex.captures(&line) {
            benchmark_data.requests_per_second_overall = captures[1].parse().unwrap_or_default();
        }
    }

    // Find minimum latency
    let min_latency = find_min_latency(&benchmark_data.stats.latency)?;
    benchmark_data.min_latency = min_latency;

    // Serialize to JSON with indentation
    let json_data = serde_json::to_string_pretty(&benchmark_data)?;

    // Print JSON data
    println!("{}", json_data);

    Ok(())
}

// Function to parse floating-point numbers
fn parse_requests_per_second(s: &str) -> f64 {
    if s.ends_with("k") {
        let value: f64 = s.trim_end_matches("k").parse().unwrap_or(0.0);
        return value * 1000.0;
    } else {
        return s.parse().unwrap_or(0.0);
    }
}

// Function to find minimum latency
fn find_min_latency(latency_stats: &LatencyStats) -> Result<String, Box<dyn std::error::Error>> {
    let avg = parse_float(&latency_stats.avg);
    let stdev = parse_float(&latency_stats.stdev);
    let max = parse_float(&latency_stats.max);

    let min_latency = if avg < stdev && avg < max {
        latency_stats.avg.clone()
    } else if stdev < avg && stdev < max {
        latency_stats.stdev.clone()
    } else {
        latency_stats.max.clone()
    };

    Ok(min_latency)
}

// Function to parse floating-point numbers
fn parse_float(s: &str) -> f64 {
    match s.parse() {
        Ok(num) => num,
        Err(_) => 0.0, // Return default value if parsing fails
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    if let Err(err) = convert_to_json(filename) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
