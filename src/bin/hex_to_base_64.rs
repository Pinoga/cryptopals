use std::env;

use cryptopals::string::{base64::base_64_encode_bytes, hex::hex_decode_bytes};

fn main() {
    let args: Vec<String> = env::args().collect();

    let hex = args[1].as_bytes();

    let hex_bytes = hex_decode_bytes(hex).expect("cannot parse hex string");
    for byte in &hex_bytes {
        print!("{:08b}.", byte);
    }
    println!();

    let output = base_64_encode_bytes(&hex_bytes);

    print!("{}", std::str::from_utf8(&output).unwrap());
}
