use crate::{dns::DnsClient, encoder::Encoder, client::ProtocolVersion, constants::SYMBOLS};


pub struct ClientHandshake {
    pub dns_client: DnsClient,
    pub encoder: Encoder,
    pub cmc: u16,
    pub version: ProtocolVersion,
    pub domain: String
}


impl ClientHandshake {
    pub fn new(version: ProtocolVersion, domain: String, nameserver: String, port: String) -> anyhow::Result<Self> {
        Ok(ClientHandshake {
            dns_client: DnsClient::new(nameserver + ":" + &port)?,
            encoder: Encoder::new(SYMBOLS)?,
            cmc: rand::random(),
            version,
            domain
        })
    }

}

