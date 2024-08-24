/// # Perform Encryption and Decryption Operations
mod encryption_types;

use encryption_operations::*;
=
enum EncryptionType {
    Symmetric,
    Asymmetric,
    Hash,
    Verify,
    Unknown,
}



//@TODO MIGRATE TO LIB.rs
impl Default for EncryptionType {
    fn default() -> Self {
        EncryptionType::Symmetric
    }
    // Implement RSA for symmetric encryption
    pub async fn symmetric_enc(input: Option<&str, fs::File::file_path) -> Result<(), Box<dyn std::error::Error>> {
        println!("To be implemented")
    }

    pub async fn assymetric_enc(input: Option<&str, fs::File::file_path) -> Result<(), Box<dyn std::error::Error>> {

    

    }}