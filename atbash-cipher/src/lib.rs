const ASCII_A: u8 = b'a' as u8;
const ASCII_Z: u8 = b'z' as u8;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(map_char)
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|ch| ch.into_iter().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .map(map_char)
        .collect::<String>()
}

fn map_char(c: char) -> char {
    if c.is_numeric() {
        c
    } else {
        (ASCII_Z - (c as u8 - ASCII_A)) as char
    }
}
