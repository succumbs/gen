use crate::wordlist::WORDLIST;
use rand::prelude::*;

pub fn generate_passphrase(length: u8, separator: char, capitalize: bool) -> String {
    let mut rng = rand::thread_rng();

    WORDLIST
        .choose_multiple(&mut rng, length as usize)
        .map(|word| {
            if capitalize {
                capitalize_str(word)
            } else {
                word.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(&separator.to_string())
}

fn capitalize_str(s: &str) -> String {
    let mut chars = s.chars();
    chars.next().unwrap().to_uppercase().collect::<String>() + chars.as_str()
}
