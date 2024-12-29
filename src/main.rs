use std::env;

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

    let mut bit_accumulator: u16 = 0;

    let mut bit_count: u32 = 0;

    let mut output_index: usize = 0;
    let mut index: usize = 0;
    while output_index < size {
        // println!("bit_count: {:<16}", bit_count);

        if index < input.len() {
            println!("bit_accumulator:         {:016b}", bit_accumulator);
            bit_accumulator <<= 8;

            println!("bit_accumulator <<=8:    {:016b}", bit_accumulator);
            bit_accumulator |= input[index] as u16;
            println!("read byte:               {:016b}", input[index]);
            println!("bit_accumulator |= byte: {:016b}", bit_accumulator);

            bit_count += 8;
            index += 1
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
                "keep last {} bits:        {:016b}",
                bit_count, bit_accumulator
            );
        }

        if bit_count > 0 {
            output[output_index] =
                BASE_64_CHARACTERS[(bit_accumulator << (6 - bit_count)) as usize];
            output_index += 1;
            println!(
                "base64 6-bit ({} padded zeroes):       {:06b}",
                6 - bit_count,
                (bit_accumulator << (6 - bit_count))
            );
            bit_count = 0;
            bit_accumulator = 0;
        } else if output_index < output.len() {
            output[output_index] = b'=';
            output_index += 1;
        }
    }
    output
}
