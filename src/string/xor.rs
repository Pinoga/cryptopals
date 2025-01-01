use std::fmt::{Display, Formatter};

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
