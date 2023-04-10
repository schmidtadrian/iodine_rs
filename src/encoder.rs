use std::str::Utf8Error;

use data_encoding::{Specification, Encoding, SpecificationError, DecodeError};
use thiserror::Error;


pub struct Encoder {
    pub encoding: Encoding
}

impl Encoder {
    pub fn new(symbols: &str) -> Result<Self, EncodingError> {
        let mut spec = Specification::new();
        spec.symbols.push_str(symbols);
        Ok(Encoder { encoding: spec.encoding()? })
    }

    pub fn enc(&self, data: &[u8]) -> String {
        self.encoding.encode(data)
    }

    pub fn decode_byte(&self, data: Vec<u8>) -> Result<Vec<u8>, EncodingError> {
        Ok(self.encoding.decode(&data[1..]).unwrap())
    }

    pub fn decode_byte_to_string(&self, data: Vec<u8>) -> Result<String, EncodingError>{
        Ok(std::str::from_utf8(&self.decode_byte(data)?)?.to_string())
    }
}

#[derive(Error, Debug)]
pub enum EncodingError {
    #[error("The given symbols aren't a valid encoding!")]
    InvalidEncoding(#[from] SpecificationError),
    #[error("Couldn't decode data! {0}")]
    DecodeError(#[from] DecodeError),
    #[error("Couldn't decode to String! {0}")]
    Utf8(#[from] Utf8Error)
}
