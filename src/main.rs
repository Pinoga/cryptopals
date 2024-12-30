use std::{cmp, env};

static BASE_64_CHARACTERS: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = args[1].as_bytes();
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

    let mut output = vec![0u8; size];

    let mut bit_accumulator: u32 = 0;

    let mut bit_count: u32 = 0;

    let mut output_index: usize = 0;
    let mut index: usize = 0;
    while index < input.len() {
        let mut shift = cmp::min(3, input.len() - index) * 8;
        println!("bit_accumulator:               {:032b}", bit_accumulator);
        bit_accumulator <<= shift;

        println!("bit_accumulator <<=24:         {:032b}", bit_accumulator);
        bit_accumulator |= (input[index] as u32) << shift - 8;
        println!("read first byte:               {:032b}", input[index]);
        println!("bit_accumulator |= byte:       {:032b}", bit_accumulator);
        index += 1;
        bit_count += 8;

        if index < input.len() {
            bit_accumulator |= (input[index] as u32) << shift - 16;
            println!("read second byte:              {:032b}", input[index]);
            println!("bit_accumulator |= byte:       {:032b}", bit_accumulator);
            index += 1;
            bit_count += 8;
        }

        if index < input.len() {
            bit_accumulator |= input[index] as u32;
            println!("read third byte:               {:032b}", input[index]);
            println!("bit_accumulator |= byte:       {:032b}", bit_accumulator);
            index += 1;
            bit_count += 8;
        }

        while bit_count >= 6 {
            output[output_index] =
                BASE_64_CHARACTERS[(bit_accumulator >> (bit_count - 6)) as usize];
            output_index += 1;
            println!(
                "base64 6-bit:       {:06b}",
                (bit_accumulator >> (bit_count - 6))
            );
            bit_count -= 6;
            bit_accumulator &= (1 << bit_count) - 1;
            println!(
                "keep last {:02} bits:             {:032b}",
                bit_count, bit_accumulator
            );
        }
    }

    // If there's remaining bits in the accumulator, pad them with zeroes until we have 6 bits
    if bit_count > 0 {
        output[output_index] = BASE_64_CHARACTERS[(bit_accumulator << (6 - bit_count)) as usize];
        output_index += 1;
        println!(
            "base64 6-bit ({} padded zeroes):       {:06b}",
            6 - bit_count,
            (bit_accumulator << (6 - bit_count))
        );
    }

    // Insert padding
    while output_index < output.len() {
        output[output_index] = b'=';
        output_index += 1;
    }

    return output;
}
