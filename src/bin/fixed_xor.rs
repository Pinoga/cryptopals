use std::{env, error::Error};

use cryptopals::crypto::{
    hex::{hex_decode_bytes, hex_encode, InvalidHexCharError},
    xor::{xor, XORDistinctLengthInputsError},
};

#[derive(Debug)]
pub enum FixedXorError {
    InvalidHexChar(InvalidHexCharError),
    XORDistinctLengthInputs(XORDistinctLengthInputsError),
    Utf8Error(std::string::FromUtf8Error),
}

impl std::fmt::Display for FixedXorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base_message = "Error producing XOR";
        match self {
            FixedXorError::InvalidHexChar(e) => write!(f, "{base_message}: {e:?}"),
            FixedXorError::XORDistinctLengthInputs(e) => {
                write!(f, "{base_message}: {e:?}")
            }
            FixedXorError::Utf8Error(e) => write!(f, "{base_message}: {e:?}"),
        }
    }
}

impl std::error::Error for FixedXorError {}

impl From<InvalidHexCharError> for FixedXorError {
    fn from(err: InvalidHexCharError) -> FixedXorError {
        FixedXorError::InvalidHexChar(err)
    }
}

impl From<XORDistinctLengthInputsError> for FixedXorError {
    fn from(err: XORDistinctLengthInputsError) -> FixedXorError {
        FixedXorError::XORDistinctLengthInputs(err)
    }
}

impl From<std::string::FromUtf8Error> for FixedXorError {
    fn from(err: std::string::FromUtf8Error) -> FixedXorError {
        FixedXorError::Utf8Error(err)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let result = fixed_xor(&args[1], &args[2])?;
    println!("{}", result);
    return Ok(());
}

fn fixed_xor(hex1: &str, hex2: &str) -> Result<String, FixedXorError> {
    let hex1_bytes = hex_decode_bytes(hex1.as_bytes())?;
    let hex2_bytes = hex_decode_bytes(hex2.as_bytes())?;

    let xor_bytes = xor(&hex1_bytes, &hex2_bytes)?;
    return Ok(hex_encode(&xor_bytes));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {}
}
