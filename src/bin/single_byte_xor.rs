use cryptopals::string::{hex::hex_decode_bytes, utils::english_score, xor::xor};

fn main() {
    let cipher_hex = b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let cipher_bytes = hex_decode_bytes(cipher_hex).expect("invalid hex string");
    let mut largest_score: u32 = 0;
    let mut best_scored_text = Vec::with_capacity(cipher_bytes.len());
    let mut key = Vec::with_capacity(cipher_bytes.len());

    let mut char: u8 = 32;
    while char < 128 {
        let maybe_key = vec![char; cipher_bytes.len()];
        let maybe_original_text = xor(&cipher_bytes, &maybe_key).unwrap();

        let score = english_score(&maybe_original_text);
        if score > largest_score {
            largest_score = score;
            best_scored_text = maybe_original_text;
            key = maybe_key;
        }

        char += 1;
    }

    println!(
        "Decrypted text (UTF-8): {}",
        String::from_utf8(best_scored_text).unwrap()
    );

    println!("Key (UTF-8): {}", String::from_utf8(key).unwrap());
}
