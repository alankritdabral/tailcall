#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! regex = "1.10.2"
//! ```

use regex::Regex;
use std::io;

fn suggest_pr_title(pr_title: &str) -> Option<String> {
    // Define the regex pattern
    let regex_pattern = r"^[a-zA-Z]+(\([a-zA-Z]+\))?: .+";
    let regex = Regex::new(regex_pattern).unwrap();

    if regex.is_match(pr_title) {
        // PR title is valid, no suggestion needed
        None
    } else {
        // PR title is invalid, suggest a new title
        let suggested_title = "Your suggested PR title here".to_string();
        Some(suggested_title)
    }
}

fn main() {
    // Example usage
    println!("Enter the PR title:");
    let mut pr_title = String::new();
    io::stdin().read_line(&mut pr_title).expect("Failed to read line");

    // Trim newline characters from input
    pr_title = pr_title.trim().to_string();

    if let Some(suggested_title) = suggest_pr_title(&pr_title) {
        println!("Suggested PR title: {}", suggested_title);
    } else {
        println!("PR title is valid.");
    }
}
