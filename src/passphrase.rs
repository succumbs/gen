use crate::wordlist::WORDLIST;
use rand::prelude::*;

pub fn generate_passphrase(length: u32, separator: char, capitalize: bool) -> String {
    let mut rng = thread_rng();

    (0..length)
        .map(|_| {
            let word = WORDLIST.choose(&mut rng).unwrap();
            match capitalize {
                true => capitalize_str(word),
                false => word.to_string(),
            }
        })
        .collect::<Vec<String>>()
        .join(&separator.to_string())
}

fn capitalize_str(s: &str) -> String {
    let mut chars = s.chars();
    chars.next().unwrap().to_uppercase().chain(chars).collect()
}
