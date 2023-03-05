#![allow(dead_code)]
use crate::nric::NRIC;
use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit};
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use dotenv;
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use std::env;
use std::dbg;

#[derive(Debug, Deserialize, Serialize)]
pub struct SecretNRICString {
    secret: String,
}

impl std::fmt::Display for SecretNRICString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.secret)
    }
}

impl SecretNRICString {
    fn new(nric: NRIC) -> Self {
        SecretNRICString {
            secret: nric.to_string(),
        }
    }

    fn encrypt(&self, filepath: &str, key_var: &str) -> Result<String> {
        let input: String = self.secret.to_owned();
        // Load the environment variables from the `.env` file
        dotenv::from_filename(filepath).map_err(|err| {
            anyhow!(
                "Error loading environment variables from `.env` file, err={}",
                err
            )
        })?;

        // Find the value of the SECRET_KEY variable
        let secret_key = env::var(key_var)
            .map_err(|err| anyhow!("SECRET_KEY not found in `.env` file, err={}", err))?;
        dbg!(&secret_key, &secret_key.as_bytes(), &secret_key.len());
        // Generate a random nonce
        let mut nonce = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce);

        // Create the AES-256-GCM cipher
        let cipher = Aes256Gcm::new_from_slice(secret_key.as_bytes())
            .map_err(|err| anyhow!("Error creating cipher from secret key, err={}", err))?;

        // Encrypt the input string using the cipher and nonce
        let ciphertext = cipher
            .encrypt(&nonce.into(), input.as_bytes())
            .map_err(|err| anyhow!("Error encrypting input string, err={}", err))?;

        // Concatenate the nonce and ciphertext into a single byte vector
        let mut encrypted_data = Vec::new();
        encrypted_data.extend_from_slice(&nonce);
        encrypted_data.extend_from_slice(&ciphertext);

        // Encode the encrypted data as a Base64-encoded string
        let encrypted_string = general_purpose::STANDARD_NO_PAD.encode(&encrypted_data);

        Ok(encrypted_string)
    }

    fn decrypt(&self, filepath: &str, key_var: &str) -> Result<String> {
        let input = &self.secret.to_owned();
        // Load the environment variables from the `.env` file
        dotenv::from_filename(filepath).map_err(|err| {
            anyhow!(
                "Error loading environment variables from `.env` file, err={}",
                err
            )
        })?;

        // Find the value of the SECRET_KEY variable
        let secret_key = env::var(key_var)
            .map_err(|err| anyhow!("SECRET_KEY not found in `.env` file, err={}", err))?;

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
        let cipher = Aes256Gcm::new_from_slice(secret_key.as_bytes())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_and_decrypt_string() -> Result<(), Box<dyn std::error::Error>> {
        // Set up the input values
        let secret_nric_string = SecretNRICString {
            secret: "Hello, world!".to_owned(),
        };
        let filepath = ".env.example";
        let key_var = "SECRET_KEY";

        // Encrypt the plaintext using the encrypt_string function
        let _encrypted_string = secret_nric_string.encrypt(filepath, key_var)?;

        // Decrypt the encrypted string using the decrypt_string function
        let decrypted_string = secret_nric_string.decrypt(filepath, key_var)?;

        // Check that the decrypted string matches the original plaintext
        assert_eq!(decrypted_string, secret_nric_string.secret);

        Ok(())
    }
}
