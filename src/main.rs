mod cli;
mod passphrase;
mod wordlist;

use clap::Parser;
use cli::{Cli, Commands};
use passphrase::generate_passphrase;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Passphrase { length, separator } => {
            let passphrase = generate_passphrase(length, separator);
            println!("{}", passphrase);
        }
    }
}
