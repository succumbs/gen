use clap::{Parser, Subcommand, ValueEnum};

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
        separator: String,

        /// specify which word list to use
        #[arg(short, long, value_enum, default_value_t = WordList::Long)]
        wordlist: WordList,
    },

    /// calculate a password's entropy/strength
    Entropy {
        /// the password to calculate entropy from
        #[arg()]
        password: String,
    },
}

#[derive(Clone, ValueEnum)]
pub enum WordList {
    /// use the long word list
    Long,
    /// use the short word list
    Short,
}
