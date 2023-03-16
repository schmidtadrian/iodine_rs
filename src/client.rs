use data_encoding::Encoding;
use thiserror::Error;
use trust_dns_client::{client::SyncClient, udp::UdpClientConnection};

use crate::{version::send_version, base32::Base32Encoder, trust_dns::{connect, query}};

#[derive(Clone, Copy)]
pub enum ProtocolVersion {
    V502 = 0x00000502
}


pub struct Client {
    version: ProtocolVersion,
    domain: String,
    enc: Base32Encoder,
    dns: SyncClient<UdpClientConnection>
}

impl Client {
    pub fn new(version: ProtocolVersion, domain: String, nameserver: String, port: u16) -> anyhow::Result<Client> {
        Ok(Client {
            version,
            domain,
            enc: Base32Encoder::new()?,
            dns: connect(nameserver + ":" + &port.to_string())?
        })
    }

    pub fn init(&self) {
        // 1. connect dns client
        // 2. version handshake
        // 3. login
        // 4. request ip
        // 5. setup tun
    }

    pub fn version_handshake(&self) {
        self.send_version();
    }

    pub fn send_version(&self) -> anyhow::Result<Vec<u8>> {
        let version = self.version as i32;
        let seed = rand::random::<u16>();
        let bytes: [u8; 6] = [
            (version >> 24) as u8,
            (version >> 16) as u8,
            (version >>  8) as u8,
            version as u8,
            (seed >> 8) as u8,
            seed as u8
        ];
        let url = self.enc.encode(&bytes, 'v', &self.domain);

        let answer = match query(&url, &self.dns) {
            Ok(answer) => {
                match answer {
                    Some(answer) => answer,
                    None => "".to_string()
                }
            },
            Err(err) => return Err(VersionError::Query(err.to_string()).into())
        };

        let response_data = match self.enc.decode(answer) {
            Ok(data) => data,
            Err(err) => return Err(VersionError::Decode(err.to_string()).into())
        };

        println!("{:?}", response_data);
        println!("{:?} => {:?}", &response_data[..4], std::str::from_utf8(&response_data[..4]).unwrap());
        println!("{:?} => {:?}", &response_data[4..7], std::str::from_utf8(&response_data[4..8]).unwrap());

        match std::str::from_utf8(&response_data[..4]).unwrap_or("") {
            "VACK" => {
                println!("Version {:#5x} acknowledged!", self.version as i32);
                Ok(response_data[4..].to_vec())
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
    #[error("Unknown error")]
    Unknown
}
