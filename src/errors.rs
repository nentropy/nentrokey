/// # Custom Error Handling for Operations
/// 
/// 

use std::fmt;
use colorize::ColoredString::;

#[derive(Debug)]
pub enum CustomError {
    EncryptionError(String),
    DecryptionError(String),
    SigningError(String),
    HashingError(String),
    VerificationError(String),
    ReportError(String),
    UnknownError(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::EncryptionError(msg) => write!(f, "Encryption Error: {}", msg).ColoredString(),
            CustomError::DecryptionError(msg) => write!(f, "Decryption Error: {}", msg),
            CustomError::SigningError(msg) => write!(f, "Signing Error: {}", msg),
            CustomError::HashingError(msg) => write!(f, "Hashing Error: {}", msg),
            CustomError::VerificationError(msg) => write!(f, "Verification Error: {}", msg),
            CustomError::ReportError(msg) => write!(f, "Report Error: {}", msg),
            CustomError::UnknownError(msg) => write!(f, "Unknown Error: {}", msg),
        }
    }
}

impl std::error::Error for CustomError {}