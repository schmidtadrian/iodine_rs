use data_encoding::Encoding;
use trust_dns_client::{client::SyncClient, udp::UdpClientConnection, op::Edns};
use crate::{base32::SYMBOLS, trust_dns::connect, tun::create_dev};


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
    pub encoder: Encoding,
    pub dns: SyncClient<UdpClientConnection>,
    pub edns: Option<Edns>,
    pub tun: tun::platform::Device,
    pub out_pkt: Packet,
    pub in_pkt: Packet,
    pub cmc: u16,
    pub uid: u8
}

impl Client {
    pub fn new(
        version: ProtocolVersion,
        domain: String,
        nameserver: String,
        port: u16,
        password: String
    ) -> anyhow::Result<Client> {

        let mut cmc = rand::random::<u16>();
        let encoder = Self::make_encoding(SYMBOLS).unwrap();
        let dns = connect(nameserver + ":" + &port.to_string()).unwrap(); // REMOVE UNWRAP
        let (challenge, user_id) = Self::version_handshake(&dns, version as u32, &mut cmc, &domain, &encoder)?;
        let (server_ip, client_ip, mtu, netmask) = Self::login_handshake(password, challenge, user_id, &domain, &mut cmc, &encoder, &dns)?;
        let tun = create_dev("tun0".to_string(), client_ip, netmask, mtu, false);
        let edns = Self::edns0_check(&domain, &mut cmc, &encoder, &dns)?;
        Self::set_downstream_encoding(user_id, &domain, &encoder, &dns, &edns)?;
        Self::set_downstream_frag_size(user_id, 696, &domain, &mut cmc, &encoder, &dns, &edns)?;

        let client = Client {
            version,
            domain,
            encoder,
            dns,
            edns,
            tun,
            out_pkt: Default::default(),
            in_pkt: Default::default(),
            cmc,
            uid: user_id
        };

        Ok(client)
    }
}
