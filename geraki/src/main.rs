#![allow(unused)]

use anyhow::{Context, Ok, Result}; // for error
use clap::Parser; // for parsing the data
use std::process::Command; // for executing command
use std::{fs::read_to_string, io}; // for parsing the input
#[derive(Parser)]
struct Cli {
    /// the name of the project
    name: String,
    /// type of project react/rust-cli/webscrape/web3
    project: String,
}

fn auto_create_reat_app(project_name: &str) {
    let mut name = String::from(project_name);
    if name.len() == 0 {
        name = String::from("Test Project");
    }
    println!("\nCreating React Project Env...............\n");

    // executing create react project
    Command::new("npx")
        .arg("create-react-app")
        .arg(name)
        .spawn()
        .expect("create-react-app not working");
}

fn auto_tailwind_installation() {
    // installing tailwind css
    Command::new("npx")
        .arg("install")
        .arg("-D")
        .arg("tailwindcss")
        .arg("postcss")
        .arg("autoprefixier")
        .spawn()
        .expect("unable to download tailwind css");
}

fn auto_rust_cli(project_name: &str) {
    let mut name = String::from(project_name);
    if name.len() == 0 {
        name = String::from("Test Project");
    }
    println!("\nCreating Rust Env...............\n");

    // executing cargo rust
    Command::new("cargo")
        .arg("new")
        .arg(name)
        .spawn()
        .expect("cargo not working");
}
fn web_scrape(project_name: &str) {
    println!("Working on Rust Cli are we...? ");
}
fn web_3(project_name: &str) {
    
    // creating starter template for truffle project
    Command::new("truffle")
        .arg("init")
        .output()
        .expect("Failed to Start Truffle");

    // creating the contract file
    Command::new("truffle")
        .arg("create")
        .arg("contract")
        .arg(&project_name)
        .output()
        .expect("Failed to create new Contract");

    // Running a Ganache Instance
    Command::new("ganache")
        .spawn()
        .expect("Ganache -cli not working");
}
fn wrong_input() {
    println!("Working on Rust Cli are we...? ");
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();
    let name: &String = &args.name; // name of the project
    let project: &String = &args.project; // project type

    // Checking for project
    match project.to_lowercase().trim() {
        "react" => auto_create_reat_app(&name),
        "rustcli" => auto_rust_cli(&name),
        "webscrape" => web_scrape(&name),
        "web3" => web_3(&name),
        _ => wrong_input(),
    };
    Ok(())
}
