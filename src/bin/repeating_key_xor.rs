use std::{env, fs};

use cryptopals::crypto::{hex::hex_encode, xor::repeating_key_xor};

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let key = &args[2];

    let file_bytes = fs::read(path).expect("Error reading file");

    let cipher = repeating_key_xor(&file_bytes, key);
    println!("{}", hex_encode(&cipher));
}
