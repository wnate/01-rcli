const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const SYMBOLS: &[u8] = b"!@#$%^&*()_+-=[]{}|;:,.<>?";
const NUMBERS: &[u8] = b"0123456789";

use zxcvbn::zxcvbn;

pub fn generate_password(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    symbol: bool,
    number: bool,
) -> anyhow::Result<()> {
    use anyhow::anyhow;
    use rand::seq::SliceRandom;

    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let mut charset = Vec::new();
    let mut password = Vec::with_capacity(length as usize);

    if uppercase {
        charset.extend_from_slice(UPPERCASE);
    }
    if lowercase {
        charset.extend_from_slice(LOWERCASE);
    }
    if symbol {
        charset.extend_from_slice(SYMBOLS);
    }
    if number {
        charset.extend_from_slice(NUMBERS);
    }

    if charset.is_empty() {
        return Err(anyhow!("At least one character set must be selected"));
    }

    // Check if the requested length is less than the number of required character types
    let required_chars = uppercase as u8 + lowercase as u8 + symbol as u8 + number as u8;
    if length < required_chars {
        return Err(anyhow!(
            "Password length must be at least {} to satisfy all requirements",
            required_chars
        ));
    }

    // Adjust the remaining length calculation
    let remaining_length = length - required_chars;

    // Ensure at least one character from each selected type
    if uppercase {
        password.push(*UPPERCASE.choose(&mut rng).unwrap());
    }
    if lowercase {
        password.push(*LOWERCASE.choose(&mut rng).unwrap());
    }
    if symbol {
        password.push(*SYMBOLS.choose(&mut rng).unwrap());
    }

    // Fill the rest of the password
    password.extend((0..remaining_length).map(|_| *charset.choose(&mut rng).unwrap()));

    // Shuffle the password
    password.shuffle(&mut rng);

    let password = String::from_utf8(password).unwrap();
    println!("Generated password: {}", password);

    let result = zxcvbn(&password, &[]);
    println!("Password strength: {}", result.score());
    Ok(())
}
