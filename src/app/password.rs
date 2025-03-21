use rand::Rng;

// All kind of possible char
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";
const SPECIALS: &str = "!@#$%^&*()-_=+[]{}|;:,.<>?";

pub fn generate_password(
    length: usize,
    include_lowercase: bool,
    include_uppercase: bool,
    include_numbers: bool,
    include_specials: bool,
) -> String {
    let mut charset = String::new();

    if include_lowercase {
        charset.push_str(LOWERCASE);
    }
    if include_uppercase {
        charset.push_str(UPPERCASE);
    }
    if include_numbers {
        charset.push_str(NUMBERS);
    }
    if include_specials {
        charset.push_str(SPECIALS);
    }
    if charset.is_empty() {
        return "Error: No character type selected".to_string();
    }

    let mut rng = rand::thread_rng();
    (0..length)
    .map(|_| {
        let idx = rng.gen_range(0..charset.len());
        charset.chars().nth(idx).unwrap()
    })
    .collect()
}