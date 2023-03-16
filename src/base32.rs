use data_encoding::{Specification, Encoding};


const SYMBOLS: &str = "abcdefghijklmnopqrstuvwxyz012345";

pub fn encode_url(data: &[u8], cmd: char, domain: String) -> String {

    let base_32: Encoding = {
        let mut spec = Specification::new();
        spec.symbols.push_str(SYMBOLS);
        spec.encoding().unwrap()
    };

    cmd.to_string() + &base_32.encode(data) + "." +  &domain
}

pub fn decode(data: String) {

    let base_32: Encoding = {
        let mut spec = Specification::new();
        spec.symbols.push_str(SYMBOLS);
        spec.encoding().unwrap()
    };
    let mut chars = data.chars();
    chars.next();
    println!("{}", chars.as_str());
    let dec = base_32.decode(chars.as_str().as_bytes()).unwrap();
    println!("{:?}", dec);
    println!("{:?}", std::str::from_utf8(&dec[0..4]));
    dec.into_iter().for_each(|c| print!("{}, ", c as char));
    println!();
}

pub struct Base32Encoder {
    encoder: Encoding
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
