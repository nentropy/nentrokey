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
use encryption_types::*;

// Key Selection
let key_8: Vec<u8> = vec![0; 1];   // 8-bit key (1 byte)
let key_16: Vec<u8> = vec![0; 2];  // 16-bit key (2 bytes)
let key_32: Vec<u8> = vec![0; 4];  // 32-bit key (4 bytes)
let key_64: Vec<u8> = vec![0; 8];  // 64-bit key (8 bytes)
let key_128: Vec<u8> = vec![0; 16]; // 128-bit key (16 bytes)
let key_256: Vec<u8> = vec![0; 32]; // 256-bit key (32 bytes)
struct Payload {
    input_data: Vec<String>
    encryption_type: Option<EncryptionType>
    key: Option<AESKey, RSAKeyPair>
    
}


pub fn hash_data(input_data: Payload, algorithm: HashingAlgs, key_size: KeySize) -> Result<(), Box<dyn std::error::Error>> {
    
}

pub fn encrypt_with_aes(input_data: Payload) {

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