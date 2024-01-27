mod long_wordlist;
mod short_wordlist;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// generate a passphrase
    Passphrase {
        /// length of the passphrase (in words)
        #[arg(short, long, default_value_t = 6)]
        length: u8,

        /// separator between words
        #[arg(short, long, default_value = "-")]
        separator: String,

        /// specify which word list to use
        #[arg(long, value_enum, default_value_t = WordList::Long)]
        list: WordList,
    },
}

#[derive(Clone, ValueEnum)]
enum WordList {
    /// the long word list
    Long,
    /// the short word list
    Short,
}

fn main() {
    let _cli = Cli::parse();
}
