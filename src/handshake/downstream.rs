use thiserror::Error;

use crate::{util::{cmc, b32_5to8, cmc_b32_5to8}, encoder::{create_encoding, Codecs, EncodingInfo}};

use super::{client::ClientHandshake, constants::DOWN_CODEC_CHECK};


impl ClientHandshake {

    pub async fn set_downstream_frag_size(&mut self, uid: u8, size: u16) -> anyhow::Result<u16> {

        // 1 byte user id
        // 2 byte frag size
        // 2 byte cmc
        let bytes = [
            uid.to_be_bytes().as_slice(),
            &size.to_be_bytes(),
            &cmc(&mut self.cmc)
        ].concat();

        let url = "n".to_string() + &self.encoder.encode_default(&bytes);
        let response = self.dns_client.query_data(url)?;
        let data = self.encoder.decode_byte(response)?;

        if let Ok(s) = std::str::from_utf8(&data) {
            if s.contains("BADFRAG") { return Err(FragError::FragSize.into()) }
            else if s.contains("BADIP") { return Err(FragError::Addr.into()) }
        }

        let frag_size = u16::from_be_bytes([data[0], data[1]]);
        println!("Using downstream frag size: {}", frag_size);

        Ok(frag_size)
    }


    /// Checks if the given encoding is usable / supported by server
    pub async fn test_downstream_encoding(&mut self, codec: EncodingInfo) -> anyhow::Result<Option<EncodingInfo>> {

        let url = format!("y{}{}{}",
            codec.url_char,
            b32_5to8(1),
            cmc_b32_5to8(&mut self.cmc)
        );

        let response = self.dns_client.query_data(url)?;
        let decoder = create_encoding(&codec.symbols)?;
        let decoded = decoder.decode(&response[1..])?;

        match decoded == DOWN_CODEC_CHECK {
            true => Ok(Some(codec)),
            false => Ok(None)
        }
    }


    /// Sends switch downstream codec request to server
    /// On failure `self.encoder.down_enc` needs to be reset afterwards!
    pub async fn switch_downstream_encoding(&mut self, codec: EncodingInfo, uid: u8) -> anyhow::Result<()> {

        // 1 char  cmd
        // 1 char  variant
        // 1 char  base32 encoded user id
        // 3 chars base32 encoded
        let url = format!("o{}{}{}",
            b32_5to8(uid),
            codec.url_char,
            cmc_b32_5to8(&mut self.cmc)
        );

        let response = self.dns_client.query_data(url)?;
        self.encoder.set_down_codec(codec)?;
        let data = self.encoder.decode_byte_to_string(response)?;

        match data {
            s if s.contains("BADLEN")   => return Err(DownEncodingError::MsgLen.into()),
            s if s.contains("BADIP")    => return Err(DownEncodingError::Addr.into()),
            s if s.contains("BADCODEC") => return Err(DownEncodingError::Codec.into()),
            _ => {}
        }

        println!("Switched downstream codec to {}", data);
        Ok(())
    }


    /// Trys to use an provided encoding. First one that works, will be used. Previous codec
    /// is the fallback encoding.
    pub async fn downstream_encoding_handshake(&mut self, codecs: Vec<Codecs>, uid: &u8) -> anyhow::Result<Option<()>> {
        
        let fallback = self.encoder.down_enc_info.clone();

        for codec in codecs {
            if let Some(info) = self.test_downstream_encoding(EncodingInfo::new(codec)).await? {
                match self.switch_downstream_encoding(info, *uid).await {
                    Ok(v) => return Ok(Some(v)),
                    Err(err) => eprintln!("{}", err),
                }
            }
        }

        self.encoder.set_down_codec(fallback)?;
        println!("Failed, to switch downstream encoding, keeping default codec");

        Ok(None)
    }
}


#[derive(Error, Debug)]
pub enum DownEncodingError {
    #[error("Server got bad msg len")]
    MsgLen,
    #[error("Server rejected sender IP address")]
    Addr,
    #[error("Server rejected the selected codec")]
    Codec,
}

#[derive(Error, Debug)]
pub enum FragError {
    #[error("Server rejected fragment size")]
    FragSize,
    #[error("Server rejected sender IP address")]
    Addr,
}
