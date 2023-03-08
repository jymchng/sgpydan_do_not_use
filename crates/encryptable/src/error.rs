use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("ERROR: Length of error is more than 32 bytes! Invalid Length found: {found:?})")]
    InvalidLength {
        found: usize,
    },
    #[error("ERROR: Error loading environment variables from `.env` file.")]
    EnvIOError,
    #[error("ERROR: `{}` not found in `.env` file", .env_key)]
    EnvKeyNotFound {
        env_key: String,
        #[source]
        source: std::env::VarError,
    }
}