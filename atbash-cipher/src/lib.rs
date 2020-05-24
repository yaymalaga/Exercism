const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut encoded_text = String::new();
    let mut i = 0; 
    for letter in plain.replace(' ', "").to_lowercase().chars() {
        if letter.is_alphanumeric() && !letter.is_ascii_punctuation() {
            i += 1;

            if letter.is_numeric() {
                encoded_text.push(letter);
            } else {
                encoded_text.push(ALPHABET[25 - get_letter_index(letter)]);
            }

            if i % 5 == 0 {
                encoded_text.push_str(" ");
            }
        }
    }
    encoded_text.trim_end().to_owned()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let mut encoded_text = String::new();
    for letter in cipher.replace(' ', "").chars() {
        if letter.is_numeric() {
            encoded_text.push(letter);
        } else {
            encoded_text.push(ALPHABET[25 - get_letter_index(letter)]);
        }
    }
    encoded_text
}

pub fn get_letter_index(letter: char) -> usize {
    let mut result: usize = 0;
    let mut s: Vec<&str> = Vec::new();
    
    for (i, val) in ALPHABET.iter().enumerate() {
        if *val == letter {
            result = i;
        }
    }
    result
}
