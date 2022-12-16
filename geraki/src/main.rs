#![allow(unused)]
use anyhow::{Context, Ok, Result}; // for error
use clap::Parser; // for parsing the data
use std::process::Command; // for executing command
use std::{fs::read_to_string, io}; // for parsing the input

/// Welcome to Geraki!
/// Sets up the inital folder for your sexy project ideas
///
/// Currently supports -
/// 1. React
/// 2. Rust
/// 3. WebScrape
/// 4. Web3
///
/// React - 
///     Executes :
///         create-react-app <projectname>
/// 
/// Rust - 
///     Executes : 
///          cargo new <projectname>
/// 
/// WebScrape - 
///     Executes : 
///         Wishes you best of luck :) 
///    
/// Web3 - 
///     Executes : 
///         initiliazes truffle  
///         creates a contract 
///         executes ganache-cli 

#[derive(Parser)]
struct Cli {
    /// the name of the project
    name: String,
    /// type of project react/rust/webscrape/web3
    project: String,
}

fn auto_create_reat_app(project_name: &str) {
    let mut name = String::from(project_name);
    if name.len() == 0 {
        name = String::from("Test Project");
    }
    println!("\nCreating React Project Env...............\n");
    open_vs_code();
    // executing create react project
    Command::new("npx")
        .arg("create-react-app")
        .arg(name)
        .spawn()
        .expect("create-react-app not working");
}
fn open_vs_code() {
    // Opening VS code
    Command::new("code")
        .arg(".")
        .spawn()
        .expect("VS code not Opening");
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
        .expect("unable to install, tailwind css");
}
fn auto_rust_cli(project_name: &str) {
    let mut name = String::from(project_name);
    if name.len() == 0 {
        name = String::from("Test Project");
    }
    println!("\nCreating Rust Env...............\n");
    open_vs_code();
    // executing cargo rust
    Command::new("cargo")
        .arg("new")
        .arg(name)
        .spawn()
        .expect("cargo not working");
}
fn web_scrape(project_name: &str) {
    println!("Working on WebScrape are we...? ");
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
    open_vs_code();
    // Running a Ganache Instance
    Command::new("ganache")
        .spawn()
        .expect("Ganache -cli not working");
}
fn wrong_input() {
    println!("Input not recognized");
}
fn welcome() {
    println!(
        "\n
    ░██████╗░███████╗██████╗░░█████╗░██╗░░██╗██╗██╗██╗██╗██╗
    ██╔════╝░██╔════╝██╔══██╗██╔══██╗██║░██╔╝██║██║██║██║██║
    ██║░░██╗░█████╗░░██████╔╝███████║█████═╝░██║██║██║██║██║
    ██║░░╚██╗██╔══╝░░██╔══██╗██╔══██║██╔═██╗░██║╚═╝╚═╝╚═╝╚═╝
    ╚██████╔╝███████╗██║░░██║██║░░██║██║░╚██╗██║██╗██╗██╗██╗
    ░╚═════╝░╚══════╝╚═╝░░╚═╝╚═╝░░╚═╝╚═╝░░╚═╝╚═╝╚═╝╚═╝╚═╝╚═╝\n\n\n\n"
    );
}
fn main() -> Result<()> {
    welcome();

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
