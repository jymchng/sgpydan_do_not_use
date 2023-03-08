use base64::engine::GeneralPurpose;
use base64::{engine::general_purpose, Engine as _};
use clap::{App, Arg};
use rand::rngs::OsRng;
use rand::{CryptoRng, Rng};


// https://github.com/laysakura/serde-encrypt/blob/main/serde-encrypt/tests/example_serde_encrypt_shared_key_encryption_with_key_exchange.rs
fn main() {
    let matches = App::new("AES-256 Key Generator")
        .version("1.0")
        .author("Jim Chng <jimchng@outlook.com>")
        .about("Generates a secure AES-256 key")
        .arg(
            Arg::new("out")
                .short('o')
                .long("out")
                .value_name("OUTPUT_FILE_NAME")
                .help("Sets the output file for the generated key.")
                .takes_value(true),
        )
        .get_matches();

    let key = generate_aes256_key();
    let encoded_key: String = general_purpose::STANDARD_NO_PAD.encode(&key);

    if let Some(filename) = matches.value_of("out") {
        std::fs::write(filename, &key).expect("Failed to write key to file");
    } else {
        println!("{:?}", encoded_key);
    }
}

fn generate_aes256_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    let mut rng = OsRng;
    rng.fill(&mut key);
    key
}
