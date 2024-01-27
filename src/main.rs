use clap::Parser;
use cli::{Cli, Commands};
use entropy::calc_entropy;
use passphrase::generate_passphrase;

mod cli;
mod entropy;
mod passphrase;
mod wordlist_long;
mod wordlist_short;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Passphrase {
            length,
            separator,
            wordlist,
        } => println!("{}", generate_passphrase(wordlist, length, separator)),

        Commands::Entropy { password } => match calc_entropy(password) {
            Ok(entropy) => println!("{:.02} bits", entropy),
            Err(error) => println!("Error: {}", error),
        },
    }
}
