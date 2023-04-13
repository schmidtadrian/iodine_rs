use thiserror::Error;

use crate::{util::{cmc_b32_5to8, b32_5to8}, encoder::{EncodingInfo, Codecs}};
use super::client::ClientHandshake;


impl ClientHandshake {


    /// Checks if a codec is usable / supported by server, by sending its pattern
    pub async fn test_upstream_encoding(&mut self, pattern: &str) -> anyhow::Result<bool> {

        let url = "z".to_string() + &cmc_b32_5to8(&mut self.cmc) + pattern;
        let response = self.dns_client.query_data(url.clone())?;
        let decoded = self.encoder.decode_byte_to_string(response)?;

        // we may use a dns label < 63
        Ok(url == decoded.replace('.', ""))
    }


    /// Trys to use an provided encoding. First one that works, will be used. Previous codec
    /// is the fallback encoding.
    pub async fn upstream_encoding_handshake(&mut self, encodings: Vec<Codecs>, uid: &u8) -> anyhow::Result<Option<()>> {

        for encoding in encodings {
            if self.test_upstream_encoding(encoding.get_test_pattern()).await? {
                match self.switch_upstream_encoding(EncodingInfo::new(encoding), uid).await {
                    Ok(v) => return Ok(Some(v)),
                    Err(err) => eprintln!("{}", err),
                }
            }
        }

        println!("Failed, to switch upstream encoding, keeping default codec");
        Ok(None)
    }


    /// Sends switch upstream codec request to server
    pub async fn switch_upstream_encoding(&mut self, codec: EncodingInfo, uid: &u8) -> anyhow::Result<()> {

        let url = format!("s{}{}{}",
            b32_5to8(*uid),
            // raw bits per encoded byte
            b32_5to8((8_f32 / codec.factor()).ceil() as u8),
            cmc_b32_5to8(&mut self.cmc)
        );

        let response = self.dns_client.query_data(url)?;
        let decoded = self.encoder.decode_byte_to_string(response)?;

        match decoded {
            x if x.contains("BADLEN")   => return Err(SwitchCodecError::Length.into()),
            x if x.contains("BADIP")    => return Err(SwitchCodecError::Ip.into()),
            x if x.contains("BADCODEC") => return Err(SwitchCodecError::Codec.into()),
            _ => {}
        };

        self.encoder.set_up_codec(codec)?;
        println!("Switched upstream encoding to {}", decoded);

        Ok(())
    }
}


#[derive(Error, Debug)]
pub enum SwitchCodecError {
    #[error("Server received a bad message length")]
    Length,
    #[error("Server rejected the selected codec")]
    Codec,
    #[error("Server rejected IP/User")]
    Ip,
    #[error("Can't handle server response")]
    UnexpectedResponse,
}
