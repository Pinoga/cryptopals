use std::env;

use cryptopals::string::{base64::base_64_encode_bytes, hex::hex_decode_bytes};

fn main() {
    let args: Vec<String> = env::args().collect();

    let hex = &args[1];

    println!("{}", hex_to_base_64(hex));
}

fn hex_to_base_64(hex: &str) -> String {
    let hex_bytes = hex_decode_bytes(hex.as_bytes()).unwrap();
    let base_64_bytes = base_64_encode_bytes(&hex_bytes);
    let str = String::from_utf8(base_64_bytes).unwrap();
    return str;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_convert() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let result = hex_to_base_64(input);
        assert_eq!(
            result,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }
}
