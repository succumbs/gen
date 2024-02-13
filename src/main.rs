mod cli;
mod passphrase;
mod password;
mod wordlist;

use clap::Parser;
use cli::{Cli, Commands};
use passphrase::generate_passphrase;
use password::generate_password;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Passphrase {
            length,
            separator,
            capitalize,
        } => {
            println!("{}", generate_passphrase(length, separator, capitalize));
        }

        Commands::Password {
            length,
            mut letters,
            mut digits,
            mut specials,
            exclude_ambiguous,
        } => {
            match (letters, digits, specials) {
                (false, false, false) => {
                    letters = true;
                    digits = true;
                    specials = true;
                }
                _ => {}
            }

            println!(
                "{}",
                generate_password(length, letters, digits, specials, exclude_ambiguous)
            );
        }
    }
}
