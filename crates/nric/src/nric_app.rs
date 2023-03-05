mod builder;
mod digits;
mod prefix;
mod suffix;

#[cfg(feature = "python")]
pub mod python;

pub mod nric;

use crate::nric::NRIC;
use clap::{App, Arg};

fn main() {
    let matches = App::new("nric")
        .version("1.0")
        .author("DO NOT USE!")
        .about("CLI to validate NRIC from a text, DO NOT USE!")
        .arg(
            Arg::with_name("INPUT")
                .short('i')
                .long("input")
                .help("Your Input String")
                .required(true)
                .index(1),
        )
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let nric = NRIC::new(input);

    match nric {
        Ok(_) => println!("Valid"),
        Err(_) => println!("Invalid"),
    }
}
