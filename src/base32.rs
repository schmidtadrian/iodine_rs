use data_encoding::{Specification, Encoding};


/// server uses custom base32 alphabet
const SYMBOLS: &str = "abcdefghijklmnopqrstuvwxyz012345";

pub struct Base32Encoder {
    pub encoder: Encoding
}

impl Base32Encoder {
    pub fn new() -> Result<Base32Encoder, data_encoding::SpecificationError> {
        let mut spec = Specification::new();
        spec.symbols.push_str(SYMBOLS);
        Ok(Base32Encoder { encoder: spec.encoding()? })
    }

    pub fn encode(&self, data: &[u8], cmd: char, domain: &String) -> String {
        cmd.to_string() + &self.encoder.encode(data) + "." + domain
    }

    pub fn decode(&self, data: String) -> Result<Vec<u8>, data_encoding::DecodeError> {
        self.encoder.decode(data[1..].as_bytes())
    }
}
