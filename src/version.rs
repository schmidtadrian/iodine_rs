use crate::client::Client;
use data_encoding::Encoding;
use thiserror::Error;
use trust_dns_client::{client::SyncClient, udp::UdpClientConnection};

pub struct VersionResponse {
    pub challenge: u32,
    pub user_id: u8
}

impl Client {
    pub fn version_handshake(
        dns_client: &SyncClient<UdpClientConnection>,
        version: u32,
        cmc: &mut u16,
        domain: &String,
        encoder: &Encoding
    ) -> anyhow::Result<(u32, u8)>{

        // 4 byte version
        // 2 byte cmc
        let bytes = &[
            version.to_be_bytes().as_slice(),
            cmc.to_be_bytes().as_slice()
        ].concat();
        *cmc += 1;

        let url = Self::encode(bytes, 'v', domain, encoder);
        let msg = Self::create_msg(&None, url);
        let answer = Self::send_query(dns_client, msg)?;
        let decoded = Self::decode(answer, encoder)?;

        // response data:
        // 4 bytes: Status VACK / VNAK / VFUL
        // 4 bytes 4-7: VACK -> login challenge
        //              VNAK -> server protocol version
        //              VFUL -> max users
        // 1 byte    8: if VACK user id
        match std::str::from_utf8(&decoded[..4])? {
            "VACK" => {
                println!("Version {:#5x} acknowledged! You are user #{}", version as i32, decoded[8]);
                Ok((u32::from_be_bytes(decoded[4..8].try_into().map_err(|_| VersionError::UnexpectedResponse)?), decoded[8]))
            },
            "VNAK" => Err(VersionError::VersionDiff.into()),
            "VFUL" => Err(VersionError::NoSlots.into()),
            _ => Err(VersionError::Unknown.into())
        }
    }
}

#[derive(Error, Debug)]
pub enum VersionError {
    #[error("Query error: {0}")]
    Query(String),
    #[error("Decode error: {0}")]
    Decode(String),
    #[error("Server uses a different protocol version")]
    VersionDiff,
    #[error("Server has no free slots")]
    NoSlots,
    #[error("Can't handle server response!")]
    UnexpectedResponse,
    #[error("Unknown error")]
    Unknown
}
