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
        #[arg(short, long, default_value_t = '-')]
        separator: char,

        /// capitalize each word in the passphrase
        #[arg(short, long)]
        capitalize: bool,
    },

    /// generate a password
    /// will default to using all character types if no options are provided
    #[command(verbatim_doc_comment)]
    Password {
        /// length of the password
        #[arg(short, long, default_value_t = 14)]
        length: u8,

        /// include alphabetical characters (upper & lower case)
        #[arg(short, long)]
        alphabetical: bool,

        /// include numbers
        #[arg(short, long)]
        numerical: bool,

        /// include special characters
        #[arg(short, long)]
        special: bool,

        /// exclude ambiguous characters
        #[arg(short, long)]
        exclude_ambiguous: bool,
    },
}
