use clap::Parser;
use gitsc::cli::Cli;
use gitsc::config::load_config;
use gitsc::error::Error;
use gitsc::git::{get_staged_diff, is_git_repository};

fn main() {
    let cli = Cli::parse();

    println!("--- CLI Arguments ---");
    println!("{:#?}", cli);

    match load_config() {
        Ok(config) => {
            println!("\n--- Configuration Loaded ---");
            println!("{:#?}", config);
        }
        Err(e) => {
            eprintln!("Error loading configuration: {}", e);
            std::process::exit(1);
        }
    }

    if !is_git_repository() {
        eprintln!("Error: Not a Git repository.");
        std::process::exit(1);
    }

    println!("\nThis is a Git repository.");

    match get_staged_diff() {
        Ok(diff) => {
            println!("\n--- Staged Changes ---");
            println!("{}", diff);
        }
        Err(Error::NoStagedChanges) => {
            println!("\nNo staged changes found.");
        }
        Err(e) => {
            eprintln!("\nAn error occurred: {}", e);
            std::process::exit(1);
        }
    }
}
