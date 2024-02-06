use rand::prelude::*;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMERIC: &str = "0123456789";
const SPECIAL: &str = "!@#$%^&*()-_=+[]{}|;:'\",.?<>\\/";
const AMBIGUOUS: &str = "0Ol1I|,.:;`'\"";

pub fn generate_password(
    length: u32,
    alphabetical: bool,
    numerical: bool,
    special: bool,
    exclude_ambiguous: bool,
) -> String {
    let mut rng = thread_rng();
    let mut charset = Vec::new();

    if alphabetical {
        charset.extend(ALPHABET.chars());
    }

    if numerical {
        charset.extend(NUMERIC.chars());
    }

    if special {
        charset.extend(SPECIAL.chars());
    }

    if exclude_ambiguous {
        charset.retain(|&c| !AMBIGUOUS.contains(c));
    }

    (0..length)
        .map(|_| charset.choose(&mut rng).unwrap())
        .collect()
}
