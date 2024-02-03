use rand::prelude::*;

pub fn generate_password(length: u8, alphabetical: bool, numeric: bool, special: bool) -> String {
    let mut rng = thread_rng();
    let mut charset = Vec::new();

    if alphabetical {
        charset.extend('a'..='z');
        charset.extend('A'..='Z');
    }

    if numeric {
        charset.extend('0'..='9');
    }

    if special {
        charset.extend("!@#$%^&*".chars());
    }

    (0..length)
        .map(|_| {
            let index = rng.gen_range(0..charset.len());
            charset[index]
        })
        .collect()
}
