use clap::Parser;
use cli::{Cli, Commands};
use passphrase::generate_passphrase;

mod cli;
mod wordlist_long;
mod passphrase;
mod wordlist_short;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Passphrase {
            length,
            separator,
            wordlist,
        } => println!("{}", generate_passphrase(wordlist, length, separator)),
    }
}
