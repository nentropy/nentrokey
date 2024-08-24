//! # Nentrokey Encryption Library
//! 
//! ```sh
//! cargo run
//! ```
//! 
//! ## Features
//! - Use flag -e or --encryption to specify encryption type
//! - Use flag -f or --file to specify file path
//! - Use flag -t or --text to specify text
//! - Use flag -v or --verbose to enable verbose output
//! - Use flag -q or --quiet to enable quiet output
//! 
//! - Run your main function from the CLI
//! - More to come this is mostly an experimental project
//! 
//! Example running as a standalone with flags: 
//! ```bash
//! cargo run nentrokey -t -e symmetric --verbose --quiet
//! ```
//! Altering the library to suit your security needs:
//! ```cargo
//! use nentrokey::main;
//! 
//! fn run_nentrokey() {
//!     main();
//! }
//! 
//! ```
//! 

use crate::lib::{ main, ArgMatches, matches };

pub fn main(input: Option<ArgMatches::matches>) -> Result<(), Box<dyn std::error::Error>> {
    crate::lib::main(input)
    Ok(("Encrytption of {:?} completed", input))
    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Error message")))
}