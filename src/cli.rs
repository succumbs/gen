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
    #[command(visible_aliases = ["pp"])]
    Passphrase {
        /// length of the passphrase (in words)
        #[arg(short, long, default_value_t = 6)]
        length: u32,

        /// separator between words
        #[arg(short, long, default_value_t = '-')]
        separator: char,

        /// capitalize each word in the passphrase
        #[arg(short, long)]
        capitalize: bool,
    },

    
    /// generate a password
    /// will include all charsets unless explicitly specified by flags
    #[command(visible_aliases = ["pw", "pwd"], verbatim_doc_comment)]
    Password {
        /// length of the password
        #[arg(short, long, default_value_t = 14)]
        length: u32,

        /// include letters
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