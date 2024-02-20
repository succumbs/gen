use rand::prelude::*;

const ALPHABETICAL: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMERICAL: &str = "0123456789";
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
    let mut charset = String::new();

    if alphabetical {
        charset.push_str(ALPHABETICAL);
    }

    if numerical {
        charset.push_str(NUMERICAL);
    }

    if special {
        charset.push_str(SPECIAL);
    }

    if exclude_ambiguous {
        charset.retain(|c| !AMBIGUOUS.contains(c));
    }

    (0..length)
        .map(|_| charset.chars().choose(&mut rng).unwrap())
        .collect()
}
