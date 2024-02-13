use rand::prelude::*;

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SPECIALS: &str = "!@#$%^&*()-_=+[]{}|;:'\",.?<>\\/";
const AMBIGUOUS: &str = "0Ol1I|,.:;`'\"";

pub fn generate_password(
    length: u32,
    letters: bool,
    digits: bool,
    specials: bool,
    exclude_ambiguous: bool,
) -> String {
    let mut rng = thread_rng();
    let mut charset = String::new();

    if letters {
        charset.push_str(LETTERS);
    }

    if digits {
        charset.push_str(DIGITS);
    }

    if specials {
        charset.push_str(SPECIALS);
    }

    if exclude_ambiguous {
        charset.retain(|c| !AMBIGUOUS.contains(c));
    }

    (0..length)
        .map(|_| charset.chars().choose(&mut rng).unwrap())
        .collect()
}
