use crate::handshake::client::ClientHandshake;
use crate::encoder::Encoder;
use crate::dns::DnsClient;
use crate::tun::create_dev;


#[derive(Clone, Copy)]
pub enum ProtocolVersion {
    V502 = 0x00000502
}

#[derive(Default, Debug, Clone)]
pub struct Packet {
    pub len: u32,
    pub sent_len: u32,
    pub offset: u32,
    pub data: Vec<u8>,
    pub seq_no: u8,
    pub fragment: u8
}

impl Packet {
    pub fn inc_seq_no(&mut self) -> &Self {
        self.seq_no = (self.seq_no+1) & 7;
        self
    }
}

pub struct Client {
    pub version: ProtocolVersion,
    pub domain: String,
    pub encoder: Encoder,
    pub dns_client: DnsClient,
    pub tun: tun::platform::Device,
    pub out_pkt: Packet,
    pub in_pkt: Packet,
    pub cmc: u16,
    pub uid: u8,
    pub frag_size: u16
}



impl Client {
    pub fn new(
        version: ProtocolVersion,
        domain: String,
        nameserver: String,
        port: u16,
        password: String
    ) -> anyhow::Result<Client> {

        let mut handshake = ClientHandshake::new(version, domain.to_string(), nameserver, port.to_string())?;
        let (challenge, user_id) = handshake.version_handshake()?;
        let (server_ip, client_ip, mtu, netmask) = handshake.login_handshake(password, challenge, user_id)?;
        let tun = create_dev("tun0".to_string(), client_ip, netmask, mtu, false);
        handshake.edns_check()?;
        handshake.set_downstream_encoding(user_id)?;
        let frag_size = handshake.set_downstream_frag_size(user_id, 696)?;

        Ok(Client {
            version, domain, tun, uid: user_id, frag_size, 
            out_pkt: Default::default(), in_pkt: Default::default(),
            encoder: handshake.encoder, dns_client: handshake.dns_client, cmc: handshake.cmc
        })
    }
}
