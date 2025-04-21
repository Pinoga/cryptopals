use cryptopals::crypto::{hex::hex_decode_bytes, xor::best_guess_decrypt_single_xor};

fn main() {
    let cipher_hex = b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let cipher_bytes = hex_decode_bytes(cipher_hex).expect("invalid hex string");
    let (best_scored_text, key, _) = best_guess_decrypt_single_xor(&cipher_bytes);

    println!(
        "Decrypted text (UTF-8): {}",
        String::from_utf8(best_scored_text).unwrap()
    );

    println!("Key (UTF-8): {}", String::from_utf8(key).unwrap());
}
