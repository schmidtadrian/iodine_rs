use std::net::ToSocketAddrs;

use crate::{encoder::Encoder, client::ProtocolVersion, dns_client::client::DnsClient};


pub struct ClientHandshake {
    pub dns_client: DnsClient,
    pub encoder: Encoder,
    pub cmc: u16,
    pub version: ProtocolVersion,
    pub domain: String
}


impl ClientHandshake {
    pub async fn new<S: ToSocketAddrs>(version: ProtocolVersion, domain: String, nameserver: S) -> anyhow::Result<Self> {
        Ok(ClientHandshake {
            dns_client: DnsClient::new("0.0.0.0:8000", nameserver, domain.to_owned(), 250)?,
            encoder: Encoder::new(Default::default())?,
            cmc: rand::random(),
            version,
            domain
        })
    }

}

