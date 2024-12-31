use std::{env, fmt::Display, fmt::Formatter};

static BASE_64_CHARACTERS: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

#[derive(Debug, Clone)]
struct InvalidHexCharError;

impl std::error::Error for InvalidHexCharError {}

impl Display for InvalidHexCharError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "invalid hex character")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let hex = args[1].as_bytes();

    let hex_bytes = hex_to_bytes(hex).expect("cannot parse hex string");

    for byte in &hex_bytes {
        print!("{:08b}", *byte);
    }
    println!();

    // let input = args[1].as_bytes();
    // let input = b"ab";
    // for letter in input {
    //     println!("letter (input): {:08b}", *letter)
    // }

    let output = base_64_encode(&hex_bytes);

    for letter in &output {
        print!("{}", *letter as char);
    }
}

fn hex_char_to_byte(char: u8) -> Result<u8, InvalidHexCharError> {
    match char {
        b'0'..=b'9' => Ok(char - b'0'),
        b'a'..=b'z' => Ok(char - b'a' + 10),
        b'A'..=b'Z' => Ok(char - b'A' + 10),
        _ => Err(InvalidHexCharError),
    }
}

fn hex_to_bytes(input: &[u8]) -> Result<Vec<u8>, InvalidHexCharError> {
    let size = input.len().div_ceil(2);

    let mut output = Vec::with_capacity(size);

    let mut iter = input.iter();

    // Since two hexes form a byte, if we have an odd amount of hexes we must treat the first hex as a full byte
    if input.len() % 2 == 1 {
        let first_hex_digit = hex_char_to_byte(*iter.next().unwrap())?;
        output.push(first_hex_digit);
    }

    // reading two hex characters at a time
    while let (Some(&high), Some(&low)) = (iter.next(), iter.next()) {
        let first_hex_digit = hex_char_to_byte(high)?;
        let second_hex_digit = hex_char_to_byte(low)?;
        output.push((first_hex_digit << 4) | (second_hex_digit));
    }

    return Ok(output);
}

fn base_64_encode(input: &[u8]) -> Vec<u8> {
    let size: usize = (input.len().div_ceil(3) * 4) as usize;

    let mut output = Vec::with_capacity(size);

    let mut index: usize = 0;

    // We have at least 3 bytes to read
    while index + 2 < input.len() {
        // load 3 bytes into chunk
        let chunk = ((input[index] as u32) << 16)
            | ((input[index + 1] as u32) << 8)
            | (input[index + 2] as u32);

        // read 6 bits
        output.push(BASE_64_CHARACTERS[((chunk >> 18) & 0x3F) as usize]);
        // read 6 bits
        output.push(BASE_64_CHARACTERS[((chunk >> 12) & 0x3F) as usize]);
        // read 6 bits
        output.push(BASE_64_CHARACTERS[((chunk >> 6) & 0x3F) as usize]);
        // read 6 bits
        output.push(BASE_64_CHARACTERS[(chunk & 0x3F) as usize]);

        index += 3;
    }

    let remaining_bytes = input.len() - index;

    if remaining_bytes == 1 {
        let chunk = (input[index] as u32) << 16;

        // read 6 bits
        output.push(BASE_64_CHARACTERS[((chunk >> 18) & 0x3F) as usize]);

        // read 6 bits (with 4-bit padding)
        output.push(BASE_64_CHARACTERS[((chunk >> 12) & 0x3F) as usize]);

        // since we've formed only two letters, pad the output with two ='s
        output.push(b'=');
        output.push(b'=');
    } else if remaining_bytes == 2 {
        let chunk = ((input[index] as u32) << 16) | ((input[index + 1] as u32) << 8);

        // read 6 bits
        output.push(BASE_64_CHARACTERS[((chunk >> 18) & 0x3F) as usize]);

        // read 6 bits
        output.push(BASE_64_CHARACTERS[((chunk >> 12) & 0x3F) as usize]);

        // read 6 bits (with 2-bit padding)
        output.push(BASE_64_CHARACTERS[((chunk >> 6) & 0x3F) as usize]);

        // since we've formed only three letters, pad the output with a =
        output.push(b'=');
    }

    return output;
}
