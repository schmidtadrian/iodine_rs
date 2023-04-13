use flate2::Compression;
use flate2::write::ZlibEncoder;

use crate::dns_client::client::DnsClient;
use crate::downstream;
use crate::handshake::client::ClientHandshake;
use crate::encoder::Encoder;
use crate::tun::create_dev;
use crate::util::uid_to_char;


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
    pub fn inc_seq_no(&mut self) -> u8 {
        self.seq_no = (self.seq_no+1) & 7;
        self.seq_no
    }

    pub fn is_last_fragment(&self) -> bool {
        self.sent_len == self.data.len() as u32 - self.offset
    }

    pub fn reset_out(&mut self, data: &Vec<u8>) {
        self.len = 0;
        self.sent_len = 0;
        self.offset = 0;
        self.data.clear();
        self.data.extend(data);
        self.seq_no = self.inc_seq_no();
        self.fragment = 0;
    }

    pub fn reset_in(&mut self, seq_no: u8, fragment: u8) {
        self.len = 0;
        self.sent_len = 0;
        self.offset = 0;
        self.data.clear();
        self.seq_no = seq_no;
        self.fragment = fragment;
    }
}

pub struct Client {
    pub version: ProtocolVersion,
    pub domain: String,
    pub encoder: Encoder,
    pub dns_client: DnsClient,
    pub compressor: ZlibEncoder<Vec<u8>>,
    pub tun: tun::AsyncDevice,
    pub out_pkt: Packet,
    pub in_pkt: Packet,
    pub cmc: u16,
    pub data_cmc: u8,
    pub uid: u8,
    pub uid_char: char,
    pub frag_size: u16,
    /// max number of bytes fitting into dns query
    pub url_max_raw_bytes: usize
}



impl Client {
    pub async  fn new(
        version: ProtocolVersion,
        domain: String,
        nameserver: IpAddr,
        port: u16,
        password: String,
        downstream: u16
    ) -> anyhow::Result<Client> {

        let mut handshake = ClientHandshake::new(version, domain.to_string(), SocketAddr::new(nameserver, port)).await?;
        let (challenge, user_id) = handshake.version_handshake().await?;
        let (server_ip, client_ip, mtu, netmask) = handshake.login_handshake(password, challenge, user_id).await?;
        let tun = create_dev("tun0".to_string(), client_ip, netmask, mtu, true);
        handshake.edns_check().await?;
        handshake.set_downstream_encoding(user_id).await?;
        let frag_size = handshake.set_downstream_frag_size(user_id, downstream).await?;

        // max dns len is 255, minus null terminating byte, minus domain len, minus dot before
        // domain, minus upstream header len, minus buffer for label size
        let url_max_raw_bytes = ((254-domain.len()-1-5-4) as f32 *0.625).floor() as usize;

        Ok(Client {
            version,
            domain,
            tun,
            uid: user_id,
            uid_char: uid_to_char(user_id),
            frag_size,
            compressor: ZlibEncoder::new(Vec::new(), Compression::new(9)),
            out_pkt: Packet { data: Vec::with_capacity(64*1024), ..Default::default() },
            in_pkt: Packet { data: Vec::with_capacity(64*1024), ..Default::default() },
            encoder: handshake.encoder,
            dns_client: handshake.dns_client,
            cmc: handshake.cmc,
            data_cmc: 0,
            url_max_raw_bytes
        })
    }
}
