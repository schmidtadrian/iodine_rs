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

    pub fn encode(&self, data: &[u8], cmd: char, domain: &String) -> String {
        cmd.to_string() + &self.encoding.encode(data) + "." + domain
    }

    pub fn encode_data(&self, data: &[u8], domain: &String) -> String {
        self.encoding.encode(data) + "." + domain
    }

    pub fn decode(&self, data: String) -> Result<Vec<u8>, EncodingError> {
        Ok(self.encoding.decode(data[1..].as_bytes())?)
    }

    pub fn decode_to_string(&self, data: String) -> Result<String, EncodingError>{
        Ok(std::str::from_utf8(&self.decode(data)?)?.to_string())
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
