use clap::{App, Arg};
use gemblockchain::GemAddress;

fn main() {
    let matches = App::new("gemgen")
        .version("0.2.1")
        .author("DEADBLACKCLOVER <deadblackclover@protonmail.com>")
        .about("CLI address generator for Gem blockchain")
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .value_name("INT")
                .help("Sets a address generate count"),
        )
        .get_matches();

    let count_arg = matches.value_of("count").unwrap_or("1");
    let count = count_arg.parse().unwrap();

    for i in 0..count {
        let gem = GemAddress::generate(None);

        println!("# {}", i + 1);
        println!("Address: {}", gem.address);
        println!("Mnemonic phrase: {}", gem.mnemonic_phrase);
        println!("Mini secret key: {}", gem.mini_secret_key_to_string());
        println!("Public key: {}", gem.public_key_to_string());
        println!("---------------------------------------------------");
    }
}
