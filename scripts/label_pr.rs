#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! hubcaps = "0.11.0"
//! regex = "1.5.4"
//! ```

use hubcaps::pulls::Pulls;
use hubcaps::repositories::Repository;
use regex::Regex;
use std::collections::HashSet;

// Define a structure to represent label rules
#[derive(Debug)]
struct LabelRule<'a> {
    label: &'a str,
    branch: Vec<&'a str>,
    title: Vec<Regex>,
    files: Option<Vec<&'a str>>,
}

// Define a function to add labels to a pull request
fn add_labels_to_pr(pr: &Pulls, labels: Vec<&str>) {
    pr.labels()
        .add(labels)
        .expect("Failed to add labels to pull request");
}

// Define a function to process a pull request based on label rules
fn process_pull_request(pr: &Pulls, label_rules: &[LabelRule]) {
    // Extract relevant information
    let branch_name = pr.base_ref().expect("Failed to get base ref").name;
    let title = pr.title();
    let files_changed = pr
        .files()
        .iter()
        .filter_map(|file| file.filename.clone())
        .collect::<HashSet<_>>();

    // Iterate through label rules
    for rule in label_rules {
        // Check if branch name matches
        if rule.branch.iter().any(|pattern| branch_name.contains(pattern)) {
            add_labels_to_pr(pr, vec![rule.label]);
            return;
        }

        // Check if title matches
        if rule.title.iter().any(|regex| regex.is_match(title)) {
            add_labels_to_pr(pr, vec![rule.label]);
            return;
        }

        // Check if any file matches
        if let Some(file_patterns) = &rule.files {
            if files_changed.iter().any(|file| file_patterns.iter().any(|pattern| file.contains(pattern)))
            {
                add_labels_to_pr(pr, vec![rule.label]);
                return;
            }
        }
    }
}

fn main() {
    // GitHub API token
    let token = "YOUR_GITHUB_TOKEN";

    // Repository information
    let owner = "owner";
    let repo_name = "repository";

    // Create a GitHub client
    let github = hubcaps::Github::new(token).expect("Failed to create GitHub client");

    // Fetch the repository
    let repo = github.repo(owner, repo_name);

    // Define label rules
    let label_rules = vec![
        LabelRule {
            label: "type: chore",
            branch: vec!["/chore/.+", "/refactor/.+", "/maintenance/.+"],
            title: vec![Regex::new(r"chore").unwrap(), Regex::new(r"refactor").unwrap()],
            files: Some(vec!["*.yml", "*.conf", "*.sbt", "*.json", "*.xml", "Dockerfile"]),
        },
        LabelRule {
            label: "type: fix",
            branch: vec!["/fix/.+", "/hotfix/.+", "/bugfix/.+", "/patch/.+"],
            title: vec![Regex::new(r"fix").unwrap(), Regex::new(r"hotfix").unwrap(), Regex::new(r"bugfix").unwrap(), Regex::new(r"patch").unwrap()],
            files: None,
        },
        LabelRule {
            label: "type: feature",
            branch: vec!["/feat/.+", "/feature/.+", "/enhancement/.+", "/new/.+"],
            title: vec![Regex::new(r"feat").unwrap(), Regex::new(r"feature").unwrap(), Regex::new(r"enhancement").unwrap(), Regex::new(r"new").unwrap()],
            files: None,
        },
        LabelRule {
            label: "type: docs",
            branch: vec!["/doc/.+", "/docs/.+"],
            title: vec![Regex::new(r"doc").unwrap(), Regex::new(r"docs").unwrap()],
            files: Some(vec!["*.md", "*.txt", "*.rst", "*.wiki", "*.html"]),
        },
    ];

    // Iterate through open pull requests
    for pr in repo.pulls().iter() {
        process_pull_request(&pr, &label_rules);
    }
}
