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
        SigningError, HashingError, VerificationError, UnknownError, ReportError};
use lib::{ main, matches };
use bitvec::prelude::*;
use sha2::{ Sha256, Sha512, Digest };

use crate::encryption_types::HashingAlgs;


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
    key_size: Option<KeySize>
    key: Option<AESKey, RSAKeyPair>
    
}

struct CipherPayload {
    cipher_text: Option<std::fs::File,String>
    encryption_type: Option<EncryptionType>
    algorithm: Option<HashingAlgs>
    key_size: Option<KeySize>
    key: Option<AESKey, RSAKeyPair>
}

let payload: Payload = Payload {
    input_data: matches.value_of("input").unwrap().to_string(),
    encryption_type: matchess.value_of("encryption").unwrap().to_string(),
    algorithm: matches.value_of("algorithm").unwrap().to_string(),
    key_size: mactches.value_of("key_size").unwrap().to_string(),
    key: matches.value_of("key").unwrap().to_string(),
}


trait Encrypt {
    fn encrypt_with_aes(input_data: Payload) -> Result<Vec<u8>, EncryptionError>> {
        
    }

}
    pub async fn hash_data(input_data: Payload) -> Result<(), HashingError>> {
        use std::vec::bitvec;
        use encryptions_ops::hash_data

        let mut key: bitvec<i32> = bitvec![0; 512];
        let mut input_data: String = String::new();
        let mut encyrption_type: String = String::new("hashing": HashingAlgs);
        let mut hasher = Sha512::new();
        let mut key_256: bitvec<i32> = bitvec![0; 256]; // 256-bit key
        let mut key_512: bitvec<i32>  = bitvec![2; 512]; // 256-bit key
        key_512.set(2, true);

        hasher.update(input_data);

        // retrieve the result
        let result = hasher.finalize().expect(log::error!("Hashing Error"));
        Ok(result), Err(HashingError);
    }
    


    fn sign_message(input_data: Payload) -> Result<Vec<u8>, SigningError>> {

    }

    fn encrypt_with_rsa(input_data: Payload) -> Result<Vec<u8>, EncryptionError>> {
        use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
            let mut rng = rand::thread_rng();
            let bits = 2048;
            let priv_key = rsaPivateKey::new(&mut rng, bits).expect("Failed to generate key");
            let pub_key = rsaPublicKey::from(&priv_key);

        //Encryption
        if matches.text {
            let data = String::from(&matches)
        }
        let data = std::fs::File::open(&matches)
        let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
        assert_ne!(&data[..], &enc_data[..]);



    }

/// Universal Decryption @TODO Universal Encryption
trait Decrypt {
    fn decrypt_cipher_code(input_data: CipherPayload) -> Result<Vec<u8>, DecryptionError>> {
            let algorithm = input_data.algorithm.unwrap();
            let key_size = input_data.key_size.unwrap();
            if input_data.key {
                let key = input_data.key.unwrap();
            }
            let plain_text = println!("//To be implemented");

            //RSA Decryption
            // Decrypt
            if algorithm == "RSA" {
                let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
                assert_eq!(&data[..], &dec_data[..]);
                
            } else if algoritm == "Symmetric" {
                // TODO Implement
            }
            
    }


    fn decrypt_report(output_data: Metrics) -> Result<Report, ReportError> {

    }

    fn test_hash(hash_result: i32, )


}

