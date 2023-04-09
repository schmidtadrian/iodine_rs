use crate::{encoder::Encoder, client::ProtocolVersion, constants::SYMBOLS, dns_client::client::DnsClient};


pub struct ClientHandshake {
    pub dns_client: DnsClient,
    pub encoder: Encoder,
    pub cmc: u16,
    pub version: ProtocolVersion,
    pub domain: String
}


impl ClientHandshake {
    pub async fn new(version: ProtocolVersion, domain: String, nameserver: String, port: String) -> anyhow::Result<Self> {
        Ok(ClientHandshake {
            dns_client: DnsClient::new("0.0.0.0:8000", &(nameserver + ":" + &port))?,
            encoder: Encoder::new(SYMBOLS)?,
            cmc: rand::random(),
            version,
            domain
        })
    }

}

