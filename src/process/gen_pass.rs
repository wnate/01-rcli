const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const SYMBOLS: &[u8] = b"!@#$%^&*()_+-=[]{}|;:,.<>?";

pub fn generate_password(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    symbol: bool,
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

    if charset.is_empty() {
        return Err(anyhow!("At least one character set must be selected"));
    }

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
    password
        .extend((0..length as usize - password.len()).map(|_| *charset.choose(&mut rng).unwrap()));

    // Shuffle the password
    password.shuffle(&mut rng);

    let password = String::from_utf8(password).unwrap();
    println!("Generated password: {}", password);

    Ok(())
}
