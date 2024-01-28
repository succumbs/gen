use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// generate a passphrase
    Passphrase {
        /// length of the passphrase (in words)
        #[arg(short, long, default_value_t = 6)]
        length: u8,
        /// separator between words
        #[arg(short, long, default_value = "-")]
        separator: char,
    },
}