/// # Perform Encryption and Decryption Operations
/// ### Includes Data Structures
/// 1. Encryption Types
/// 2. Hashing Algs
/// 3. AES Key
/// 3. Bit Size for Symmetrical or Assymetrical (Public and Private)
/// 
/// Import this into the encryption operations and perform any operations required using 
/// the libraries of your choice.


use encryption_operations::*;
use std::fs::{ read, write };
use fs::{ ReadOptions, WriteOptions };

#[derive(Debug)]
pub enum EncryptionType {
    Symmetric(AESKey),
    Asymmetric(RSAKeyPair),
    Hash(HashingAlgs),
    Verify,
    Unknown,
}

#[derive(Debug)]
pub enum HashingAlgs {
    MD5,
    SHA1,
    SHA256,
    SHA512,
}

#[derive(Debug)]
pub struct AESKey {
    key: Vec<u8>,
    algorithm: Option<String>,
    seed: u8,
}

#[derive(Debug)]
pub struct RSAKeyPair {
    public_key: Vec<u8>,
    private_key: Vec<u8>,
    algorithm: Option<String>,
    bit_size: u8,
    seed: u8,
}

impl EncryptionType {
    //Create an AESKey with a default setting
    pub fn new_symmetric(key: Vec<u8>, seed: u8) -> Self {
        EncryptionType::Symmetric(AESKey {
            key,
            algorithm: Some("AES-256".to_string()), // Default algorithm
            seed,
        })
    }

    //Create an RSAKeyPair with a default setting
    pub fn new_asymmetric(public_key: Vec<u8>, private_key: Vec<u8>, seed: u8) -> Self {
        EncryptionType::Asymmetric(RSAKeyPair {
            public_key,
            private_key,
            algorithm: Some("RSA".to_string()), // Default
            bit_size: 2048, // Default
            seed,
        })
    }

    // Create a Hashing Algorithm
    pub fn new_hashing(algorithm: HashingAlgs) -> Self {
        EncryptionType::Hash(algorithm)
    }

    // Select encryption type based on command-line matches
    pub fn from_matches(matches: &ArgMatches) -> Self {
        if let Some(encryption_type) = matches.value_of("encryption") {
            match encryption_type.to_lowercase().as_str() {
                "symmetric" => EncryptionType::new_symmetric(vec![0u8; 32], 42),
                "asymmetric" => EncryptionType::new_asymmetric(vec![1, 2, 3], vec![4, 5, 6], 42),
                "md5" => EncryptionType::new_hashing(HashingAlgs::MD5),
                "sha1" => EncryptionType::new_hashing(HashingAlgs::SHA1),
                "sha256" => EncryptionType::new_hashing(HashingAlgs::SHA256),
                "sha512" => EncryptionType::new_hashing(HashingAlgs::SHA512),
                "verify" => EncryptionType::Verify,
                _ => EncryptionType::Unknown,
            }
        } else {
            EncryptionType::Unknown
        }
    }}