use std::{fmt::Display, fmt::Formatter};

#[derive(Debug, Clone)]
pub struct InvalidHexCharError;

impl std::error::Error for InvalidHexCharError {}

impl Display for InvalidHexCharError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "invalid hex character")
    }
}

static HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

fn hex_char_to_byte(char: u8) -> Result<u8, InvalidHexCharError> {
    match char {
        b'0'..=b'9' => Ok(char - b'0'),
        b'a'..=b'f' => Ok(char - b'a' + 10),
        b'A'..=b'F' => Ok(char - b'A' + 10),
        _ => Err(InvalidHexCharError),
    }
}

pub fn hex_decode_bytes(input: &[u8]) -> Result<Vec<u8>, InvalidHexCharError> {
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

pub fn hex_encode(input: &[u8]) -> String {
    let mut output = Vec::with_capacity(input.len() * 2);

    let mut iter = input.iter();

    while let Some(&byte) = iter.next() {
        output.push(HEX_CHARS[(byte >> 4) as usize]);
        output.push(HEX_CHARS[(byte & 0xF) as usize]);
    }

    // This is safe because we know output is ASCII
    return unsafe { String::from_utf8_unchecked(output) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        let empty_input = [];
        let result = hex_decode_bytes(&empty_input).expect("fail to parse hex string");
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn one_character() {
        let empty_input = [b'a'];
        let result = hex_decode_bytes(&empty_input).expect("fail to parse hex string");
        assert_eq!(result.len(), 1);
        assert_eq!(result, [0xA]);
    }

    #[test]
    fn two_characters() {
        let empty_input = [b'a', b'3'];
        let result = hex_decode_bytes(&empty_input).expect("fail to parse hex string");
        assert_eq!(result.len(), 1);
        assert_eq!(result, [0xA3]);
    }

    #[test]
    fn three_characters() {
        let empty_input = [b'a', b'3', b'f'];
        let result = hex_decode_bytes(&empty_input).expect("fail to parse hex string");
        assert_eq!(result.len(), 2);
        assert_eq!(result, [0xA, 0x3F]);
    }

    #[test]
    fn long_input() {
        let empty_input = [b'0', b'0', b'0', b'f', b'1', b'2', b'0', b'0', b'c', b'e'];
        let result = hex_decode_bytes(&empty_input).expect("fail to parse hex string");
        assert_eq!(result.len(), 5);
        assert_eq!(result, [0x0, 0x0F, 0x12, 0x0, 0xCE]);
    }
}
