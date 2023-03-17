use trust_dns_client::{client::SyncClient, udp::UdpClientConnection};
use crate::{base32::Base32Encoder, trust_dns::connect};


#[derive(Clone, Copy)]
pub enum ProtocolVersion {
    V502 = 0x00000502
}

pub struct Client {
    pub version: ProtocolVersion,
    pub domain: String,
    pub enc: Base32Encoder,
    pub dns: SyncClient<UdpClientConnection>
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

    pub fn init(&self, password: String) {
        // 1. connect dns client
        // 2. version handshake
        let (challenge, uid) = match self.send_version() {
            Ok(data) => data,
            Err(err) => return eprint!("{}", err)
        };
        // 3. login
        if let Err(err) = self.login_handshake(password, challenge, uid) {
            eprintln!("{}", err)
        }
        // 4. request ip
        // 5. setup tun
    }
}

