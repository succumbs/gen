use crate::cli::WordList;
use crate::long_wordlist::LONG_WORD_LIST;
use crate::short_wordlist::SHORT_WORD_LIST;
use rand::prelude::*;

pub fn generate_passphrase(wordlist: WordList, length: u8, separator: String) -> String {
    let mut rng = rand::thread_rng();

    let chosen_words: Vec<&str> = match wordlist {
        WordList::Long => LONG_WORD_LIST
            .choose_multiple(&mut rng, length as usize)
            .cloned()
            .collect(),
        WordList::Short => SHORT_WORD_LIST
            .choose_multiple(&mut rng, length as usize)
            .cloned()
            .collect(),
    };

    chosen_words.join(separator.as_str())
}