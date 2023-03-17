use crate::client::Client;
use crate::trust_dns::query;
use thiserror::Error;

impl Client {
    /// Sends client version to server.
    /// On success returns a tuple of (login_challenge, user_id)
    pub fn send_version(&self) -> anyhow::Result<(u32, u8)> {
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
            Ok(answer) => answer,
            Err(err) => return Err(VersionError::Query(err.to_string()).into())
        };

        // response data:
        // 4 bytes 0-3: first for bytes is VACK/VNAK or VFUL
        // 4 bytes 4-7: VACK -> login challenge
        //              VNAK -> server protocol version
        //              VFUL -> max users
        // 1 byte    8: if VACK user id
        let response_data = match self.enc.decode(answer) {
            Ok(data) => data,
            Err(err) => return Err(VersionError::Decode(err.to_string()).into())
        };

        match std::str::from_utf8(&response_data[..4]).unwrap_or("") {
            "VACK" => {
                println!("Version {:#5x} acknowledged! You are user #{}", self.version as i32, response_data[8]);
                Ok((u32::from_be_bytes(response_data[4..8].try_into().unwrap()), response_data[8]))
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
