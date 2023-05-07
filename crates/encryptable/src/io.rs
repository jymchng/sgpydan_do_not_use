use crate::utils;
use crate::error;

use anyhow::{anyhow, Error, Result};
use base64::{
    engine::{general_purpose, GeneralPurpose},
    Engine as _,
};
use crypto_box::SecretKey;
use dotenv;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

static B64_GPSNP: GeneralPurpose = general_purpose::STANDARD_NO_PAD;

pub fn get_env_key_as_array(filepath: &str, key_var: &str) -> Result<[u8; 32]> {
    dotenv::from_filename(filepath)
        .map_err(|_err| anyhow!(Into::<Error>::into(error::Error::EnvIOError)))?;

    // Find the value of the SECRET_KEY variable
    let key = env::var(key_var).map_err(|_err| {
        anyhow!(Into::<Error>::into(error::Error::EnvKeyNotFound {
            env_key: key_var.to_string(),
            source: std::env::VarError::NotPresent
        }))
    })?;
    let key: Vec<u8> = B64_GPSNP
        .decode(key)
        .map_err(|err| anyhow!("ERROR: {key_var} cannot be decoded, err={err}"))?;
    let array: [u8; 32] = key
        .as_slice()
        .try_into()
        .map_err(|err| anyhow!("{key_var} cannot be converted to `[u8,32]`, err: {err}",))?;
    Ok(array)
}

pub fn write_keypairs_to_env_example(
    filepath: impl Into<String>,
    public_key: &str,
    private_key: &str,
) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filepath.into())?;
    file.write_all(format!("PUBLIC_KEY={}\nPRIVATE_KEY={}\n", public_key, private_key).as_bytes())?;
    Ok(())
}

pub fn get_secret_key_from_env(filepath: &str, key_var: &str) -> Result<SecretKey> {
    dotenv::from_filename(filepath)
        .map_err(|_err| anyhow!(Into::<Error>::into(error::Error::EnvIOError)))?;

    // Find the value of the SECRET_KEY variable
    let key = env::var(key_var).map_err(|_err| {
        anyhow!(Into::<Error>::into(error::Error::EnvKeyNotFound {
            env_key: "SECRET_KEY".to_string(),
            source: std::env::VarError::NotPresent
        }))
    })?;

    let secret_key = SecretKey::from(utils::parse_u8_to_u8_32_array(key.as_bytes())?);
    Ok(secret_key)
}
