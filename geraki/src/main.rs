#![allow(unused)]

use std::{fs::read_to_string, io};

use anyhow::{Context, Result}; // for error
use clap::Parser; // for parsing the input
/// Search for a pattern in a file and display the line that contain it.
#[derive(Parser)]
struct Cli {
    /// the name of the project
    name: String,
    /// type of project react/rust-cli/webscrape/web3
    project: String,
}

fn auto_react() {
    println!("Working on react are we...? ");
}
fn auto_rust_cli() {
    println!("Working on Rust Cli are we...? ");
}
fn web_scrape() {
    println!("Working on Rust Cli are we...? ");
}
fn web_3() {
    println!("Working on Rust Cli are we...? ");
}
fn wrong_input() {
    println!("Working on Rust Cli are we...? ");
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();
    let name: &String = &args.name; // name of the project
    let project: &String = &args.project; // project type

    // Checking for project
    match project.trim() {
        "react" => auto_react(),
        "rustcli" => auto_rust_cli(),
        "webscrape" => web_scrape(),
        "web3" => web_3(),
        _ => wrong_input(),
    };

    Ok(())
}
