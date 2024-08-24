/// # Perform Encryption and Decryption Operations
/// ### Includes Data Structures
/// 1. Encryption Types
/// 2. Hashing Algs
/// 3. AES Key
/// 3. Bit Size for Symmetrical or Assymetrical (Public and Private)
mod encryption_types;

use encryption_operations::*;
use crate::lib::{ main, matches };
use std::fs::{ read, write };
use fs::{ ReadOptions, WriteOptions };



// Data Structures
enum EncryptionType {
    Symmetric: AESKey
    Asymmetric: RSAKeyPair
    Hash: HashingAlgs
    Verify
    Unknown
    Default: Symmetric
}

enum HashingAlgs {
    MD5
    SHA-1
    SHA-256
    SHA-512
}

struct AESKey {
    key: Vec<u8>
    algorithm: Option<String>
    seed: u8
}

struct RSAKeyPair {
    public_key: Vec<u8>
    private_key: Vec<u8>
    algorithm: Option<String>
    bit_size: u8
    seed: u8
}



impl EncryptionType {
    fn default() -> Self {
        EncryptionType::Symmetric
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
