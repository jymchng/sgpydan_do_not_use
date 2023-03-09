mod inout;

use anyhow::{anyhow, Result};
use base64::{
    engine::{general_purpose, GeneralPurpose},
    Engine as _,
};
use clap::{App, Arg, SubCommand};
use crypto_box::{PublicKey, SecretKey};
use serde_encrypt::{
    key::key_pair::{ReceiverKeyPair, SenderKeyPair},
    shared_key::SharedKey,
    traits::SerdeEncryptPublicKey,
    AsSharedKey, ReceiverKeyPairCore, SenderCombinedKey, SenderKeyPairCore, SenderPrivateKey
};

static B64_GPSNP: GeneralPurpose = general_purpose::STANDARD_NO_PAD;

// https://github.com/laysakura/serde-encrypt/blob/main/serde-encrypt/tests/example_serde_encrypt_shared_key_encryption_with_key_exchange.rs
fn main() {
    let matches = App::new("key-pair-gen")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .subcommand(SubCommand::with_name("sender").about("Generate key pair for sender"))
        .subcommand(SubCommand::with_name("receiver").about("Generate key pair for receiver"))
        .subcommand(
            SubCommand::with_name("shared")
                .about("Generate shared key")
                .arg(
                    Arg::with_name("rx")
                        .short('r')
                        .long("receiver-key")
                        .value_name("FILE")
                        .help("Receiver's public key file path")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("tx")
                        .short('s')
                        .long("sender-key")
                        .value_name("FILE")
                        .help("Sender's private key file path")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("sender", _)) => {
            let sender_key_pair = SenderKeyPair::generate();
            println!(
                "Public key: {:#?}",
                B64_GPSNP.encode(sender_key_pair.public_key().as_ref().as_bytes()),
            );
            println!(
                "Private key: {:#?}",
                B64_GPSNP.encode(sender_key_pair.private_key().as_ref().to_bytes()),
            );
        }
        Some(("receiver", _)) => {
            let receiver_key_pair = ReceiverKeyPair::generate();
            println!(
                "Public key: {:#?}",
                B64_GPSNP.encode(receiver_key_pair.public_key().as_ref().as_bytes())
            );
            println!(
                "Private key: {:#?}",
                B64_GPSNP.encode(receiver_key_pair.private_key().as_ref().to_bytes())
            );
        }
        Some(("shared", submatches)) => {
            let rx_file = submatches.value_of("rx").unwrap();
            let tx_file = submatches.value_of("tx").unwrap();

            let sender_secret_private_key = inout::get_secret_key_from_env(tx_file, "PRIVATE_KEY")
                .expect("Expects: Key = `PRIVATE_KEY` in {tx_file}");
            let sender_private_key = sender_secret_private_key.into();

            let shared_key = serde_encrypt::shared_key::SharedKey::generate();

            let sender_combined_key: SenderCombinedKey::new();
            // let encrypted_shared_key = shared_key.encrypt(combined_key)?;
            // let s = sender_key_pair.public_key();

            // fn bob_sends_shared_key(
            //     shared_key: &SharedKey,
            //     combined_key: &SenderCombinedKey,
            // ) -> Result<Vec<u8>> {

            //     Ok()
            // }

            // std::fs::write("shared_key.bin", &encrypted_shared_key).unwrap();
        }
        _ => {}
    }
}
