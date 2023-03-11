mod io;
mod error;
mod utils;

use base64::{
    engine::{general_purpose, GeneralPurpose},
    Engine as _,
};
use clap::{App, Arg, SubCommand};
use serde_encrypt::{
    key::key_pair::{ReceiverKeyPair, SenderKeyPair},
    ReceiverKeyPairCore, SenderKeyPairCore,
};
use x25519_dalek::{PublicKey, StaticSecret};

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

            let sender_private_key_array = io::get_env_key_as_array(tx_file, "SENDER_PRIVATE_KEY")
                .expect("Expects: Key = `SENDER_PRIVATE_KEY` in {tx_file}");
            let receiver_public_key_array =
                io::get_env_key_as_array(tx_file, "RECEIVER_PUBLIC_KEY")
                    .expect("Expects: Key = `RECEIVER_PUBLIC_KEY` in {tx_file}");
            let sender_private_key = StaticSecret::from(sender_private_key_array);
            let receiver_public_key = PublicKey::from(receiver_public_key_array);

            let shared_key = sender_private_key.diffie_hellman(&receiver_public_key);
            println!("Shared key: {:#?}", B64_GPSNP.encode(shared_key.as_bytes()));
        }
        _ => {}
    }
}
