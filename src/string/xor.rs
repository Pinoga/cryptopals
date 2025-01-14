use std::fmt::{Display, Formatter};

use super::utils::english_score;

#[derive(Debug, Clone)]
pub struct XORDistinctLengthInputsError;

impl std::error::Error for XORDistinctLengthInputsError {}

impl Display for XORDistinctLengthInputsError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "inputs have distinct lengths")
    }
}

pub fn xor(buf1: &[u8], buf2: &[u8]) -> Result<Vec<u8>, XORDistinctLengthInputsError> {
    if buf1.len() != buf2.len() {
        return Err(XORDistinctLengthInputsError);
    }

    let mut output: Vec<u8> = Vec::with_capacity(buf1.len());

    let mut buf1_iter = buf1.iter();
    let mut buf2_iter = buf2.iter();

    while let (Some(&byte1), Some(&byte2)) = (buf1_iter.next(), buf2_iter.next()) {
        output.push(byte1 ^ byte2);
    }

    return Ok(output);
}

pub fn best_guess_decrypt_single_xor(cipher_bytes: &[u8]) -> (Vec<u8>, Vec<u8>, u32) {
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
    (best_scored_text, key, largest_score)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn distinct_lengths() {
        let buf1 = &[0b01011000, 0b01011000];
        let buf2 = &[0b01011000];
        let result = xor(buf1, buf2);

        assert!(result.is_err());
    }

    #[test]
    fn equal_inputs_small() {
        let buf1 = &[0b01011000];
        let buf2 = &[0b01011000];
        let result = xor(buf1, buf2).unwrap();

        assert_eq!(result, [0]);
    }

    #[test]
    fn equal_inputs_big() {
        let buf1 = &[
            0b01011000, 0b01011000, 0b01011000, 0b01011000, 0b01011000, 0b01011000,
        ];
        let buf2 = &[
            0b01011000, 0b01011000, 0b01011000, 0b01011000, 0b01011000, 0b01011000,
        ];
        let result = xor(buf1, buf2).unwrap();

        assert_eq!(result, [0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn distinct_inputs() {
        let buf1 = &[0b01011000, 0b00011000];
        let buf2 = &[0b11010010, 0b00100001];
        let result = xor(buf1, buf2).unwrap();

        assert_eq!(result, [0b10001010, 0b00111001]);
    }
}
