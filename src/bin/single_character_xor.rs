use std::fs;

use cryptopals::string::{hex::hex_decode_bytes, xor::best_guess_decrypt_single_xor};

fn main() {
    let file = fs::read("./data/single_character_xor.txt").expect("Error reading file");

    let text = String::from_utf8(file).expect("Text not in UTF-8 format");

    let mut key: Vec<u8> = Vec::with_capacity(0);
    let mut decrypted: Vec<u8> = Vec::with_capacity(0);
    let mut best_score: u32 = 0;

    let mut lines_iter = text.split("\n");
    while let Some(line) = lines_iter.next() {
        let cipher_bytes = hex_decode_bytes(line.as_bytes()).expect("invalid hex string");
        let (maybe_decrypted, maybe_key, maybe_best_score) =
            best_guess_decrypt_single_xor(&cipher_bytes);
        if maybe_best_score > best_score {
            best_score = maybe_best_score;
            decrypted = maybe_decrypted;
            key = maybe_key;
        }
    }

    println!("Decrypted: {}", String::from_utf8(decrypted).unwrap());
    println!("Key: {}", String::from_utf8(key).unwrap());
}
