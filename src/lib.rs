
//!
//!       ____   _____   _____  ____   __
//!/ __ \ |  __ \ |_   _|/ __ \ | \ | |
//!| |  | || |__) |  | | | |  | ||  \| |
//!| |  | ||  _  /   | | | |  | || . ` |
//!| |__| || | \ \  _| |_| |__| || |\  |
//!\____/ |_|  \_\|_____|\____/ |_| \_|
//!_____  _       _____
//! / ____|| |     |_   _|
//!| |     | |       | |
//!| |     | |       | |
//!| |____ | |____  _| |_
//! \_____||______||_____|
//! 

//! # Nentropy Security Project - Encryption, Decryption and Bitwise Comparison
//! **Custom experimental security operations library and binary
//! 
//! //! ## Features
//! ```bash
//! - Use flag -e or --encryption to specify encryption type
//! - Use flag -f or --file to specify file path
//! - Use flag -t or --text to specify text
//! - Use flag -v or --verbose to enable verbose output
//! - Use flag -q or --quiet to enable quiet output
//! ```

//! 
//! ## Features:
//! - Import into any library or project
//! - Run the src/main.rs binary to execute using flags
//! - Use the src/utils.rs file to customize the output or types.
//! 
//! Binary:
//! - Run your main function from the CLI
//! - More to come this is mostly an experimental project
//! 
//! Example: 
//! ```rust
//! // Your main file
//! use nentrokey::main;
//! 
//! fn run_nentrokey() {
//!     nentrokey::main();
//! }
//! ```
//! 
//! ```bash
//! cargo run nentrokey -t -e symmetric --verbose --quiet
//! ```
#![allow(clippy::cognitive_complexity)]
mod errors;
mod encryption_types;
mod encryption_ops;
use clap::{ ArgMatches, Arg, Command };
use std::fs::File;
use fs::{ ReadOptions, WriteOptions };
use std::fs::{ File, read, write };
use log::{debug, LevelFilter};
use simplelog::Config;
use syn_crabs::setup_logging;
use colored::{ Colorize, ColoredString };
use encryption_types::{ EncryptionType, Default };

use encryption_ops::{ hash_data, symmetrical_encryption }

use errors::CustomError;

//Implements the command line using clap and calls the security operations identified
// in encryption types and encryptions ops modules
![derive(Clone, Debug, Serialize, Deserialize)]
fn main() {
    syn_crabs::setup_logging(verbose=false, quiet=false).expect("failed to init logging");
    diplay_welcome().colorize_output().bold_heading().unwrap()("failed to display intro");
    let matches: ArgMatches = Command::new("RustCrypt CLI")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Command-line encryption tool using RustCrypt")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Sets the input file path")
                .action(clap::ArgAction::Append)
                .required(false)
        )
        .arg(
            Arg::new("text")
                .short('t')
                .long("text")
                .value_name("TEXT")
                .help("Text to encrypt")
                .default_value("You Didn't Enter Anything")
                .action(clap::ArgAction::Append)
                .required(false)
        )
        .arg(
            Arg::new("encryption")
                .short('e')
                .long("encryption")
                .get_possible_values(&["symmetric", "asymmetric", "hash", "verify"])
                .default_value("symmetric")
                .value_name("ENCRYPTION_TYPE")
                .help("Specifies the encryption type: symmetric, asymmetric, hash, verify")
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Sets the level of verbosity (logger to debug mode)")
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("No logger output")
                .action(clap::ArgAction::Set),
        )
        .get_matches();

    // Logger setup based on verbosity and quiet flags
    if matches.get_flag("quiet") {
        syn_crabs::init(LevelFilter::Off, Config::default()).unwrap();
    } else if matches.get_flag("verbose") {
        syn_crabs::init(LevelFilter::Debug, Config::default()).unwrap();
    } else {
        syn_crabs::init(LevelFilter::Info, Config::default()).unwrap();
    }

    /// ### Operate on command-line arguments
    /// Retrieve file path or text, and the encryption type from the command line
    let file_path: String = matches.get_one("file").unwrap_or_else(|| "no file included").to_string();
    let text: String = matches.get_one("text").unwrap_or_else(|| "no text included").to_string();
    let encryption_type: Vec<EncryptionType> = matches.get_many("encryption")
    .map(|v: Vec<str>| match v: vec![
        "symmetric" => EncryptionType::Symmetric,
        "asymmetric" => EncryptionType::Asymmetric,
        "hash" => EncryptionType::Hash,
        "verify" => EncryptionType::Verify,
        _ => EncryptionType::Unknown], // or some other default value
    , Err((e)));
    

// Example handling based on the encryption type
    match encryption_type {
        Some(EncryptionType::Symmetric) => {
            log::info!("Performing symmetric encryption");
            // Add your symmetric encryption logic here
        }
        Some(EncryptionType::Asymmetric) => {
            log::info!("Performing asymmetric encryption");
            // Add your asymmetric encryption logic here
        }
        Some(EncryptionType::Hash) => {
            log::info!("Performing hashing");
            hash_data(&payload: Payload).expect("Hashing Failed")
        }
        Some(EncryptionType::Verify) => {
            log::info!("Verifying encryption");
            // Add your verification logic here
        }
        None => {
            log::error!("Invalid or missing encryption type. Please use one of: symmetric, asymmetric, hash, verify");
        }
    }

    // Logging the input values (for demonstration purposes)
    log::debug!("File path: {:?}", file_path);
    log::debug!("Text to encrypt: {:?}", text);
    log::debug!("Encryption type: {:?}", encryption_type);

    // Process the data based on the command-line arguments
    pub async fn process_data(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
        // Example handling for file or text inputs
        if matches.value_source("file": &str) {
            let file_path: &str = file.as_str()
                .ReadOptions()
                .from(&file_path)
            let output_path: &str = file.cr
            let input_file = fs.read(&str, Default::default());
            .expect("checked contains file_path");
            log::info!("Processing file: {}", file_path);
            return file_path;
        } else if matches.value_source("text": &str) {
            let text: &str = text.as_str()
            .expect("checked contains text");
            log::info!("`text` is defaulted");
            return text;
        } else {
        Ok((log::info("input data to encryption loaded..."))),
        Err(log::error!("Please provide either a file path with -f or text with -t").unwrap_and_then(|_| std::process::exit(1)));  
    }}
}
    
  
/// # CLI Visual Adjustments
/// ### Customizable Logging and Display

fn display_welcome(project_name: &str, description: &str, default_project="Orion CLI", default_description="A Base CLI Program for Customization") {
    let welcome_message: &str = r#"
         ____   _____   _____  ____   _   _
    / __ \ |  __ \ |_   _|/ __ \ | \ | |
   | |  | || |__) |  | | | |  | ||  \| |
   | |  | ||  _  /   | | | |  | || . ` |
   | |__| || | \ \  _| |_| |__| || |\  |
    \____/ |_|  \_\|_____|\____/ |_| \_|
     _____  _       _____
     | |     | |       | |
    / ____|| |     |_   _|
   | |     | |       | |
   | |____ | |____  _| |_
    \_____||______||_____| "

:: {project_name} ::
{description}

    ######### WELCOME ##########
"#;
    let welcome_text: Vec<&str> = vec![&project_name, &description];
        .replace("from {} to {} ", &default_project, &welcome_text[0])
        .to_string()
        .replace("From {} to {}", &default_description, &welcome_text[1])
        .to_string();
        

        Ok((&welcome_text)) => {
            let project_name = &welcome_text[0];
            let description = &welcome_text[1]
            println!("{}", welcome_text);
            log::debug!("Welcome Message Success")
        }
        Err(e) => {
            log::error!("Error occured {}", e);
        }
        log::debug!("Error occured, continuing program");
} 
        

pub fn colorize_output(output_text: &str, color: &str) -> ColoredString {
    debug!("Colorizing output_text '{}' with color '{}'", output_text, color);
    match color {
        "error" => output_text.red(),
        "info" => output_text.green(),
        "debug" => output_text.blue(),
        "critical" => output_text.yellow(),
        _ => {
            log::debug!("Unknown color '{}', returning normal output_text.", color);
            output_text.normal()
        }
    }
}

fn bold_heading(heading: &str) -> ColoredString {
    debug!("Bolding heading '{}'", heading);
    heading.bold()
}

pub fn run_orioncli() -> Result<(), Box<dyn std::error::Error>> {
    let welcome: &str = display_welcome(project_name = "Nentrokey", description = "Encryption library for Rust")
        .colorize_output(&welcome, color= "green", project_name="Nentrokey", description="Basic Rust Encryption Library")
        .bold_heading(heading: "Nentrokey Security and Hash Auditing Library")
        .unwrap_and_then(|_|
        Ok((welcome))), Err(());

}


