/// Encryption Operations Divided into type of encryption
/// Followed by Decryption Operations
/// Inclusion of Master Key (Very insecure)
/// 
/// ### Features:
/// 1. Key size selection
/// 2. Payload design
/// 3. functions implementing the appropriate library
/// 4. Decryption equivalents
/// 
/// 
mod errors;
use encryption_types::*;
use errors::CustomError::{EncryptionError, DecryptionError, \
        SigningError, HashingError, VerificationError, UnknownError};
use lib::{ main, matches };

// Key Selection
enum KeySize {
    Key8(Vec<u8>),    // 8-bit key (1 byte)
    Key16(Vec<u8>),   // 16-bit key (2 bytes)
    Key32(Vec<u8>),   // 32-bit key (4 bytes)
    Key64(Vec<u8>),   // 64-bit key (8 bytes)
    Key128(Vec<u8>),  // 128-bit key (16 bytes)
    Key256(Vec<u8>),  // 256-bit key (32 bytes)
}


/// Payload and Cipher Payload Structure (@TODO Work out how to extract from the main library function)
struct Payload {
    input_data: Option<std::fs::File, String>
    encryption_type: Option<EncryptionType>
    algorithm: Option<HashingAlgs, RSAKeyPair, AESKey>
    key_size: vec<u8>
    
}

struct CipherPayload {
    cipher_text: Option<std::fs::File,String>
    encryption_type: Option<EncryptionType>
    algorithm: Option<HashingAlgs>
    key_size: vec<u8>
    key: Option<AESKey, RSAKeyPair>
}

trait Encrypt {
    fn encrypt_with_aes(input_data: Payload) -> Result<Vec<u8>, EncryptionError>> {
        
    }

}
pub fn hash_data(input_data: Payload, algorithm: HashingAlgs, key_size: KeySize) -> Result<(), HashingError>> {
    
}

pub fn encrypt_with_aes(input_data: Payload) -> Result<Vec<u8>, EncryptionError>> {

}

pub fn encrypt_with_rsa(input_data: Payload) -> Result<Vec<u8>, EncryptionError>> {

}


// Implement RSA for symmetric encryption
pub async fn symmetric_enc(input: Option<&str, fs::File::file_path) -> Result<(), Box<dyn std::error::Error>> {
    println!("To be implemented")
}

pub async fn assymetric_enc(input: Option<&str, fs::File::file_path) -> Result<(), Box<dyn std::error::Error>> {
    println!("To be implemented")
}}

pub async fn hash_data(payload: Payload, fs::File::file_path, alg: &str) -> Result<(), Box<dyn std::error::Error>> {
    
}

pub async fn encrypt_with_aes(payload: Payload, File::read)

pub async fn sign_data(payload: Payload, alg: &str, key_size: &vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    
}


/// Universal Decryption @TODO Universal Encryption

pub fn decrypt_cipher_code(input_data: CipherPayload) -> Result<Vec<u8>, DecryptionError>> {

}