use clap::{App, Arg, SubCommand};
use serde_encrypt::{
    key::key_pair::{ReceiverKeyPair, SenderKeyPair},
    shared_key::SharedKey,
    traits::SerdeEncryptPublicKey,
    SenderKeyPairCore,
    ReceiverKeyPairCore,
    serialized::TypedSerialized,
};

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
            println!("Public key: {:#?}", sender_key_pair.public_key());
            println!("Private key: {:#?}", sender_key_pair.private_key());
        }
        Some(("receiver", _)) => {
            let receiver_key_pair = ReceiverKeyPair::generate();
            println!("Public key: {:#?}", receiver_key_pair.public_key());
            println!("Private key: {:#?}", receiver_key_pair.private_key());
        }
        Some(("shared", submatches)) => {
            let rx_file = submatches.value_of("rx").unwrap();
            let tx_file = submatches.value_of("tx").unwrap();

            let receiver_public_key = std::fs::read_to_string(rx_file).unwrap();
            let sender_private_key = std::fs::read_to_string(tx_file).unwrap();

            let receiver_key_pair = ReceiverKeyPair::from_public_key(&receiver_public_key);
            let sender_key_pair = SenderKeyPair::from_private_key(&sender_private_key);

            let sender_combined_key =
                SenderCombinedKey::new(bob_key_pair.private_key(), alice_key_pair.public_key());

            std::fs::write("shared_key.bin", &encrypted_shared_key).unwrap();
        }
        _ => {}
    }
}
