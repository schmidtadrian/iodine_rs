use data_encoding::{Specification, Encoding};


/// server uses custom base32 alphabet
pub const SYMBOLS: &str = "abcdefghijklmnopqrstuvwxyz012345";

pub struct Base32Encoder {
    pub encoder: Encoding
}

impl Base32Encoder {
    pub fn new() -> Result<Base32Encoder, data_encoding::SpecificationError> {
        let mut spec = Specification::new();
        spec.symbols.push_str(SYMBOLS);
        Ok(Base32Encoder { encoder: spec.encoding()? })
    }

    /// Returns a string with `cmd` as first char followed by encoded `data` and the `domain`
    /// Example: `<cmd><enc_data>.<domain>`
    pub fn encode(&self, data: &[u8], cmd: char, domain: &String) -> String {
        cmd.to_string() + &self.encoder.encode(data) + "." + domain
    }

    /// Omits the first char (`cmd`) from `data` and returns the decoded bytes from the string
    pub fn decode(&self, data: String) -> Result<Vec<u8>, data_encoding::DecodeError> {
        self.encoder.decode(data[1..].as_bytes())
    }
}
