use thiserror::Error;

use crate::util::cmc;

use super::client::ClientHandshake;


impl ClientHandshake {
    pub async fn version_handshake(&mut self) -> anyhow::Result<(u32, u8)> {

        let version = self.version as u32;
        // 4 byte version
        // 2 byte cmc
        let bytes = &[
            version.to_be_bytes().as_slice(),
            &cmc(&mut self.cmc)
        ].concat();

        let url = self.encoder.encode(bytes, 'v', &self.domain);
        let response = self.dns_client.query_data(url).await?;
        let data = self.encoder.decode(response)?;

        // response data:
        // 4 bytes: Status VACK / VNAK / VFUL
        // 4 bytes 4-7: VACK -> login challenge
        //              VNAK -> server protocol version
        //              VFUL -> max users
        // 1 byte    8: if VACK user id
        match std::str::from_utf8(&data[..4])? {
            "VACK" => {
                let uid = data[8];
                println!("Version {:#5x} acknowledged! You are user #{}", version, uid);
                Ok((u32::from_be_bytes(data[4..8].try_into().map_err(|_| VersionError::UnexpectedResponse)?), data[8]))
            },
            "VNAK" => Err(VersionError::VersionDiff.into()),
            "VFUL" => Err(VersionError::NoSlots.into()),
            _ => Err(VersionError::Unknown.into())
        }
    }
}

#[derive(Error, Debug)]
pub enum VersionError {
    #[error("Server uses a different protocol version")]
    VersionDiff,
    #[error("Server has no free slots")]
    NoSlots,
    #[error("Can't handle server response!")]
    UnexpectedResponse,
    #[error("Unknown error")]
    Unknown
}
