use std::env;

static BASE_64_CHARACTERS: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = args[1].as_bytes();
    // let input = b"ab";
    for letter in input {
        println!("letter (input): {:08b}", *letter)
    }

    let output = base_64_encode(input);

    for letter in &output {
        print!("{}", *letter as char);
    }
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
