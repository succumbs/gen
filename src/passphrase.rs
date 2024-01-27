use crate::cli::WordList;
use crate::wordlist_long::LONG_WORDLIST;
use crate::wordlist_short::SHORT_WORDLIST;
use rand::prelude::*;

pub fn generate_passphrase(wordlist: WordList, length: u8, separator: String) -> String {
    let mut rng = rand::thread_rng();

    let chosen_words: Vec<&str> = match wordlist {
        WordList::Long => LONG_WORDLIST
            .choose_multiple(&mut rng, length as usize)
            .cloned()
            .collect(),
        WordList::Short => SHORT_WORDLIST
            .choose_multiple(&mut rng, length as usize)
            .cloned()
            .collect(),
    };

    chosen_words.join(separator.as_str())
}