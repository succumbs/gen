use crate::wordlist::WORDLIST;
use rand::prelude::*;

pub fn generate_passphrase(length: u8, separator: char) -> String {
    let mut rng = rand::thread_rng();

    WORDLIST
        .choose_multiple(&mut rng, length as usize)
        .cloned()
        .collect::<Vec<&str>>()
        .join(separator.to_string().as_str())
}