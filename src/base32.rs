
use std::str::Utf8Error;

use data_encoding::{Specification, Encoding, SpecificationError, DecodeError};
use thiserror::Error;

use crate::client::Client;


/// server uses custom base32 alphabet
pub const SYMBOLS: &str = "abcdefghijklmnopqrstuvwxyz012345";

impl Client {
    pub fn make_encoding(symbols: &str) -> Result<Encoding, InitError>{
        let mut spec = Specification::new();
        spec.symbols.push_str(symbols);
        Ok(spec.encoding()?)
    }

    pub fn encode(data: &[u8], cmd: char, domain: &String, encoder: &Encoding) -> String {
        cmd.to_string() + &encoder.encode(data) + "." + domain
    }

    pub fn decode(data: String, encoder: &Encoding) -> Result<Vec<u8>, InitError> {
        Ok(encoder.decode(data[1..].as_bytes())?)
    }

    pub fn decode_i(&self, data: String) -> Result<Vec<u8>, InitError> {
        Self::decode(data, &self.encoder)
    }
    pub fn decode_to_string(data: String, encoder: &Encoding) -> Result<String, InitError> {
        Ok(std::str::from_utf8(&encoder.decode(data[1..].as_bytes())?)?.to_string())
    }

    pub fn build_hostname(&self, data: &[u8], domain: &String, max_len: usize) {
        let avail_space: usize = usize::min(max_len, 4096) - domain.len() - 8;
        let encode = self.encoder.encode(data[..usize::min(avail_space, data.len())].into()) + "." + domain;
        println!("DEC: {:?}", data);
        println!("ENC: {}", encode);
    }
}

#[derive(Error, Debug)]
pub enum InitError {
    #[error("Invalid encoding! {0}")]
    Encoding(#[from] SpecificationError),
    #[error("Decoding error! {0}")]
    Decoding(#[from] DecodeError),
    #[error("Couldn't decode to String! {0}")]
    Utf8(#[from] Utf8Error)
}
