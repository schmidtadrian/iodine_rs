use std::time::Duration;

use trust_dns_client::{client::SyncClient, udp::UdpClientConnection, op::Edns};
use crate::{base32::Base32Encoder, trust_dns::connect, tun::create_tun};


#[derive(Clone, Copy)]
pub enum ProtocolVersion {
    V502 = 0x00000502
}

pub struct Client {
    pub version: ProtocolVersion,
    pub domain: String,
    pub enc: Base32Encoder,
    pub dns: SyncClient<UdpClientConnection>,
    pub edns: Option<Edns>
}

impl Client {
    pub fn new(version: ProtocolVersion, domain: String, nameserver: String, port: u16) -> anyhow::Result<Client> {
        Ok(Client {
            version,
            domain,
            enc: Base32Encoder::new()?,
            dns: connect(nameserver + ":" + &port.to_string())?,
            edns: None
        })
    }

    pub fn init(&mut self, password: String) {
        // 1. connect dns client
        // 2. version handshake
        let (challenge, uid) = match self.send_version() {
            Ok(data) => data,
            Err(err) => return eprintln!("{}", err)
        };
        // 3. login
        let (_server_ip, client_ip, _mtu, netmask) = match self.login_handshake(password, challenge, uid) {
            Ok(data) => data,
            Err(err) => return eprintln!("{}", err)
        };
        // 4. setup tun
        create_tun("tun0".to_string(), client_ip, netmask, false);
        //loop {
        //   std::thread::sleep(Duration::from_secs(1));
        //}
        
        match self.edns0_check() {
            Ok(data) => println!("Using edns: {}", data),
            Err(err) => println!("{}", err)
        }

        if let Err(err) = self.set_downstream_encoding(uid) {
            println!("{}", err);
        }

        if let Err(err) = self.set_downstream_frag_size(uid, 768) {
            println!("{}", err);
        }
    }
}

