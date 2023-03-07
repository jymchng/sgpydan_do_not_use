#![allow(dead_code)]
use crate::nric::NRIC;
use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit};
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use dotenv;
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SecretNRICString {
    pub encrypted_nric: String,
}

impl SecretNRICString {
    pub fn new(nric: NRIC, filepath: &str, key_var: &str) -> Result<Self> {
        let input: String = nric.to_string();
        // Load the environment variables from the `.env` file
        let secret_array = read_secret_key_parsed_to_array(filepath, key_var)?;
        // Generate a random nonce
        let mut nonce = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce);

        // Create the AES-256-GCM cipher
        let cipher = Aes256Gcm::new_from_slice(&secret_array)
            .map_err(|err| anyhow!("ERROR: Error creating cipher from secret key, err={}", err))?;

        // Encrypt the input string using the cipher and nonce
        let ciphertext = cipher
            .encrypt(&nonce.into(), input.as_bytes())
            .map_err(|err| anyhow!("ERROR: Error encrypting input string, err={}", err))?;

        // Concatenate the nonce and ciphertext into a single byte vector
        let mut encrypted_data = Vec::new();
        encrypted_data.extend_from_slice(&nonce);
        encrypted_data.extend_from_slice(&ciphertext);

        // Encode the encrypted data as a Base64-encoded string
        let encrypted_string = general_purpose::STANDARD_NO_PAD.encode(&encrypted_data);

        Ok(SecretNRICString {
            encrypted_nric: encrypted_string,
        })
    }

    pub fn decrypt(input: impl Into<String>, filepath: &str, key_var: &str) -> Result<String> {
        let input: String = input.into();
        // Load the environment variables from the `.env` file
        let secret_array = read_secret_key_parsed_to_array(filepath, key_var)?;

        // Decode the Base64-encoded input string into a byte vector
        let encrypted_data = general_purpose::STANDARD_NO_PAD
            .decode(&input)
            .map_err(|err| anyhow!("Error decoding input string as Base64, err={}", err))?;

        // Split the encrypted data into the nonce and ciphertext
        let nonce = encrypted_data
            .get(0..12)
            .ok_or_else(|| anyhow!("Error extracting nonce from encrypted data"))?;
        let ciphertext = encrypted_data
            .get(12..)
            .ok_or_else(|| anyhow!("Error extracting ciphertext from encrypted data"))?;

        // Create the AES-256-GCM cipher
        let cipher = Aes256Gcm::new_from_slice(&secret_array)
            .map_err(|err| anyhow!("Error creating cipher from secret key, err={}", err))?;

        // Decrypt the ciphertext using the cipher and nonce
        let plaintext = cipher
            .decrypt(nonce.into(), ciphertext)
            .map_err(|err| anyhow!("Error decrypting ciphertext, err={}", err))?;

        // Convert the plaintext to a UTF-8-encoded string
        let decrypted_string = String::from_utf8(plaintext.to_vec())
            .map_err(|err| anyhow!("Error converting plaintext to string, err={}", err))?;

        Ok(decrypted_string)
    }
}

fn read_secret_key_parsed_to_array(filepath: &str, key_var: &str) -> Result<[u8; 32]> {
    dotenv::from_filename(filepath).map_err(|err| {
        anyhow!(
            "ERROR: Error loading environment variables from `.env` file, err={}",
            err
        )
    })?;

    // Find the value of the SECRET_KEY variable
    let secret_key = env::var(key_var)
        .map_err(|err| anyhow!("ERROR: `SECRET_KEY` not found in `.env` file, err={}", err))?;
    let secret_key: Vec<u8> = general_purpose::STANDARD_NO_PAD
        .decode(&secret_key)
        .map_err(|err| anyhow!("ERROR: `SECRET_KEY` cannot be decoded, err={}", err))?;
    let secret_array: [u8; 32] = secret_key.as_slice().try_into().map_err(|err| {
        anyhow!(
            "`secret_key` cannot be converted to `[u8,32]`, err: {}",
            err
        )
    })?;
    Ok(secret_array)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_and_decrypt_string() -> Result<(), Box<dyn std::error::Error>> {
        // Set up the input values
        let filepath = ".env.example";
        let key_var = "SECRET_KEY";
        let nric: NRIC = NRIC::new("S1234567D".to_string())?;
        let secret_nric_string = SecretNRICString::new(nric, filepath, key_var)?;

        // Encrypt the plaintext using the encrypt_string function
        // Decrypt the encrypted string using the decrypt_string function
        let decrypted_string =
            SecretNRICString::decrypt(secret_nric_string.encrypted_nric, filepath, key_var)?;

        // Check that the decrypted string matches the original plaintext
        assert_eq!(decrypted_string, "S1234567D");

        Ok(())
    }
}
