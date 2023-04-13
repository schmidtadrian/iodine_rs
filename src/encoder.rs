use std::str::Utf8Error;

use data_encoding::{Specification, Encoding, SpecificationError, DecodeError};
use strum::{Display, EnumIter};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct EncodingInfo {
    pub symbols: String,
    raw_len: u8,
    enc_len: u8,
    pub url_char: char
}

impl Default for EncodingInfo {
    fn default() -> Self {
        EncodingInfo::new(Codecs::default())
    }
}
impl EncodingInfo {
    pub fn new(encoding: Codecs) -> Self {
        let (symbols, raw_len, enc_len, url_char) = encoding.get_encoding_info();
        EncodingInfo { symbols, raw_len, enc_len, url_char }
    }

    /// Growth factor 
    pub fn factor(&self) -> f32 {
        self.enc_len as f32 / self.raw_len as f32
    }

}

#[derive(Display, EnumIter, Debug, PartialEq)]
pub enum Codecs {
    Base64,
    Base32,
}

impl Default for Codecs {
    fn default() -> Self { Self::Base32 }
}
impl Codecs {
    pub fn get_test_pattern(&self) -> &str {
        match self {
            Self::Base32 => "",
            Self::Base64 => "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ+0129-"
        }
    }

    pub fn get_encoding_info(&self) -> (String, u8, u8, char) {
        match self {
            Self::Base32 => ("abcdefghijklmnopqrstuvwxyz012345".to_owned(), 5, 8, 't'),
            Self::Base64 => ("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-0123456789+".to_owned(), 3, 4, 's')
        }
    }
}

pub struct Encoder {
    pub base_encoding: Encoding,
    pub up_enc: Encoding,
    pub up_enc_info: EncodingInfo,
    pub down_enc: Encoding,
    pub down_enc_info: EncodingInfo,
}


pub fn create_encoding(symbols: &str) -> Result<Encoding, EncodingError> {
    let mut spec = Specification::new();
    spec.symbols.push_str(symbols);
    Ok(spec.encoding()?)
}

impl Encoder {

    pub fn new(codec_info: EncodingInfo) -> Result<Self, EncodingError> {
        let base_encoding = create_encoding(&codec_info.symbols)?;
        Ok(Encoder {
            base_encoding: base_encoding.clone(),
            up_enc: base_encoding.clone(),
            up_enc_info: codec_info.clone(),
            down_enc: base_encoding,
            down_enc_info: codec_info
        })
    }

    fn set_codec(codec: &mut Encoding, info: &mut EncodingInfo, new_info: EncodingInfo) -> Result<(), EncodingError> {
        *codec = create_encoding(&new_info.symbols)?;
        *info = new_info;
        Ok(())
    }

    pub fn set_up_codec(&mut self, codec_info: EncodingInfo) -> Result<(), EncodingError> {
        Self::set_codec(&mut self.up_enc, &mut self.up_enc_info, codec_info)
    }

    pub fn set_down_codec(&mut self, codec_info: EncodingInfo) -> Result<(), EncodingError> {
        Self::set_codec(&mut self.down_enc, &mut self.down_enc_info, codec_info)
    }

    pub fn enc(&self, data: &[u8]) -> String {
        self.up_enc.encode(data)
    }

    pub fn encode_default(&self, data: &[u8]) -> String {
        self.base_encoding.encode(data)
    }

    pub fn decode_byte(&self, data: Vec<u8>) -> Result<Vec<u8>, EncodingError> {
        Ok(self.down_enc.decode(&data[1..])?)
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
