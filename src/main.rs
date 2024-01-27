use clap::Parser;
use cli::{Cli, Commands};
use passphrase::generate_passphrase;
use entropy::calc_entropy;

mod cli;
mod wordlist_long;
mod passphrase;
mod wordlist_short;
mod entropy;

fn main() {
    let cli = Cli::parse();

    match cli.command {

        Commands::Passphrase {
            length,
            separator,
            wordlist,
        } => println!("{}", generate_passphrase(wordlist, length, separator)),

        Commands::Entropy {
            password
        } => match calc_entropy(password) {
            Ok(entropy) => println!("{:.02} bits", entropy),
            Err(error) => println!("Error: {}", error)
        }
    }
}
