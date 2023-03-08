use crate::error;
use anyhow::{anyhow, Result};

pub(crate) fn parse_to_u8_32_array(s: &[u8]) -> Result<[u8; 32]> {
    let s_len = s.len();

    if s_len > 32 {
        return Err(error::Error::InvalidLength { found: (s_len) }.into());
    }

    let mut result: [u8; 32] = Default::default();

    result[..s_len].clone_from_slice(s);

    Ok(result)
}
