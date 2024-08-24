
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

use crate::lib::{ main, ArgMatches, matches };
use derive::{Deserialize, Serialize, Clone, Debug};

![derive(Clone, Debug, Serialize, Deserialize)]
pub fn main(input: Option<ArgMatches::matches>) -> Result<(), Box<dyn std::error::Error>> {
    crate::lib::main(input)
    Ok(("Encrytption of {:?} completed", input))
    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Error message")))
}