
use data_encoding::Specification;
use thiserror::Error;
use trust_dns_client::{op::{Message, Query, MessageType, Edns}, rr::{RecordType, DNSClass, Name}, client::Client, serialize::binary::BinDecodable, error::LexerErrorKind};

use crate::{util::b32_5to8};

/// fix 48 byte check value
/// first 16 bytes are:
///     4 times: 0000 0000,
///     4 times: 1111 1111,
///     4 times: 0101 0101,
///     4 times: 1010 1010
/// followed by 32 random bytes
pub const DOWN_CODEC_CHECK: [u8; 48] = [
    0o000, 0o000, 0o000, 0o000, 0o377, 0o377, 0o377, 0o377,
    0o125, 0o125, 0o125, 0o125, 0o252, 0o252, 0o252, 0o252,
    0o201, 0o143, 0o310, 0o322, 0o307, 0o174, 0o262, 0o027,
    0o137, 0o117, 0o316, 0o311, 0o111, 0o055, 0o122, 0o041,
	0o141, 0o251, 0o161, 0o040, 0o045, 0o263, 0o006, 0o163,
    0o346, 0o330, 0o104, 0o060, 0o171, 0o120, 0o127, 0o277
];

impl crate::client::Client {
    pub fn edns0_check(&mut self) -> anyhow::Result<bool> {
        self.use_edns(true);
        let url = "ytbqqq.".to_owned() + &self.domain;
        let msg = self.create_msg(url);

        let response = self.send_query(msg)?;
        let response_data = self.enc.decode(response)?;

        let use_edns = response_data.eq(&DOWN_CODEC_CHECK);
        self.use_edns(use_edns);

        Ok(use_edns)
    }

    pub fn set_downstream_encoding(&self, uid: u8) -> anyhow::Result<()> {
        let url = "o".to_string() + &b32_5to8(uid as usize).to_string() + "tqqq." + &self.domain;
        let msg = self.create_msg(url);
        let answer = self.send_query(msg)?;

        let dec = &self.enc.decode(answer)?;
        let response_data = std::str::from_utf8(dec)?;

        match response_data {
            s if s.contains("BADLEN")   => return Err(DownEncodingError::MsgLen.into()),
            s if s.contains("BADIP")    => return Err(DownEncodingError::Addr.into()),
            s if s.contains("BADCODEC") => return Err(DownEncodingError::Codec.into()),
            _ => println!("Set downstream codec to Base32")
        }
        Ok(())
    }

    pub fn set_downstream_frag_size(&self, uid: u8, size: u16) -> anyhow::Result<u16> {

        // 1 byte user id
        // 2 byte frag size
        // 2 byte cmc
        let bytes = [
            uid.to_be_bytes().as_slice(),
            size.to_be_bytes().as_slice(),
            rand::random::<u16>().to_be_bytes().as_slice()
        ].concat();

        let url = self.enc.encode(&bytes, 'n', &self.domain);
        let answer = self.send_query(self.create_msg(url))?;
        let decode = &self.enc.decode(answer)?;
        let response_data = std::str::from_utf8(decode)?;

        match response_data {
            s if s.contains("BADFRAG") => Err(FragError::FragSize.into()),
            s if s.contains("BADIP") => Err(FragError::Addr.into()),
            _ => {
                let bytes: [u8; 2] = decode[0..2].try_into().map_err(|_| FragError::UnexpectedResponse)?;
                let frag_size = u16::from_be_bytes(bytes);
                println!("Using downstream frag size: {}", frag_size);
                Ok(frag_size)
            }
        }
    }

}

#[derive(Error, Debug)]
pub enum DownEncodingError {
    #[error("Query error: {0}")]
    Query(String),
    #[error("Decode error: {0}")]
    Decode(String),
    #[error("Server got bad msg len")]
    MsgLen,
    #[error("Server rejected sender IP address")]
    Addr,
    #[error("Server rejected the selected codec")]
    Codec,
    #[error("Unknown error")]
    Unknown
}

#[derive(Error, Debug)]
pub enum FragError {
    #[error("Server rejected fragment size")]
    FragSize,
    #[error("Server rejected sender IP address")]
    Addr,
    #[error("Can't handle server response")]
    UnexpectedResponse,
    #[error("Unknown error")]
    Unknown
}
