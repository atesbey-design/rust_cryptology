use std::io;

fn encrypt(text: &str, shift: u8) -> String {
    let mut result = String::new();
    for ch in text.chars() {
        if ch.is_ascii_alphabetic() {
            let base = if ch.is_ascii_uppercase() { b'A' } else { b'a' };
            let offset = (ch as u8).wrapping_sub(base);
            let cipher = (offset + shift) % 26 + base;
            result.push(char::from(cipher));
        } else {
            result.push(ch);
        }
    }
    result
}

fn decrypt(cipher: &str, shift: u8) -> String {
    encrypt(cipher, 26 - shift)
}

fn main() {
    println!("Pls enter text for CipherText");
    let mut plaintext = String::new();
    io::stdin().read_line(&mut plaintext).unwrap_or_default();

    println!("Pls enter shift for CipherText");
    let mut shift_str = String::new();
    io::stdin().read_line(&mut shift_str).unwrap_or_default();
    let shift: u8 = shift_str.trim().parse().unwrap_or_default();

    let ciphertext = encrypt(&plaintext, shift);
    println!("Ciphertext: {}", ciphertext);
    let decrypted_text = decrypt(&ciphertext, shift);
    println!("Decrypted text: {}", decrypted_text);
}


