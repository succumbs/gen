use regex::Regex;
use std::io::{Error, ErrorKind};

pub fn calc_entropy(password: String) -> Result<f64, Error> {
    if password.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Password cannot be empty",
        ));
    }

    if !password.is_ascii() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Password must be ASCII",
        ));
    }

    let lowercase_regex = Regex::new(r"[a-z]").unwrap();
    let uppercase_regex = Regex::new(r"[A-Z]").unwrap();
    let digit_regex = Regex::new(r"[0-9]").unwrap();
    let special_regex = Regex::new(r"[^a-zA-Z0-9]").unwrap();

    let mut char_range: f64 = 0.0;

    if lowercase_regex.is_match(&password) {
        char_range += 26.0;
    }

    if uppercase_regex.is_match(&password) {
        char_range += 26.0;
    }

    if digit_regex.is_match(&password) {
        char_range += 10.0;
    }

    if special_regex.is_match(&password) {
        char_range += 32.0;
    }

    Ok(password.len() as f64 * char_range.log2())
}
