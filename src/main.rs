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
            mut alpha,
            mut numeric,
            mut special,
        } => {
            if !alpha && !numeric && !special {
                alpha = true;
                numeric = true;
                special = true;
            }

            println!(
                "{}",
                generate_password(length, alpha, numeric, special)
            );
        }
    }
}
