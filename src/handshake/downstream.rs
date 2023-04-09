use thiserror::Error;

use crate::util::{cmc, b32_5to8, cmc_b32_5to8};

use super::client::ClientHandshake;


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

        let url = self.encoder.encode(&bytes, 'n', &self.domain);
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

    pub async fn set_downstream_encoding(&mut self, uid: u8) -> anyhow::Result<()> {

        // 1 char  cmd
        // 1 char  variant
        // 1 char  base32 encoded user id
        // 3 chars base32 encoded
        let url = format!("o{}t{}.{}", b32_5to8(uid), cmc_b32_5to8(&mut self.cmc), &self.domain);
        let response = self.dns_client.query_data(url)?;
        let data = self.encoder.decode_byte_to_string(response)?;

        match data {
            s if s.contains("BADLEN")   => Err(DownEncodingError::MsgLen.into()),
            s if s.contains("BADIP")    => Err(DownEncodingError::Addr.into()),
            s if s.contains("BADCODEC") => Err(DownEncodingError::Codec.into()),
            _ => {
                println!("Set downstream codec to Base32");
                Ok(())
            }
        }
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
