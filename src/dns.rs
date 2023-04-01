use thiserror::Error;
use trust_dns_client::client::Client;
use trust_dns_client::error::ClientError;
use trust_dns_client::op::MessageType;
use trust_dns_client::proto::error::ProtoError;
use trust_dns_client::rr::DNSClass;
use trust_dns_client::rr::Name;
use trust_dns_client::op::Query;
use trust_dns_client::op::Message;
use trust_dns_client::rr::RecordType;
use trust_dns_client::{client::SyncClient, udp::UdpClientConnection, op::Edns};


pub struct DnsClient {
    client: SyncClient<UdpClientConnection>,
    edns: Option<Edns>,
    chunk_id: u16
}

impl DnsClient {
    /// `socket` - e. g. "1.1.1.1:53", no support for hostnames
    pub fn new(socket: String) -> Result<Self, DnsError> {
        let addr = socket.parse().unwrap();
        // mapping error, bc trust_dns_client::client::client::Client in DnsClient::query_msg
        // also throws ClientError
        let conn = UdpClientConnection::new(addr).map_err(DnsError::Connection)?;

        Ok(DnsClient { client: SyncClient::new(conn), edns: None, chunk_id: rand::random() })
    }

    pub fn chunk_id(&mut self) -> u16 {
        self.chunk_id = match self.chunk_id.checked_add(7727) {
            Some(val) => val,
            None => 7727-(u16::MAX-self.chunk_id)
        };
        self.chunk_id
    }

    pub fn new_msg(&mut self, url: String) -> Result<Message, DnsError> {
        let mut msg = Message::new();
        msg.add_query({
            let mut query = Query::query(Name::from_ascii(url)?, RecordType::TXT);
            query.set_query_class(DNSClass::IN);
            query
        })
        .set_id(self.chunk_id())
        .set_message_type(MessageType::Query)
        .set_op_code(trust_dns_client::op::OpCode::Query)
        .set_recursion_desired(true);

        if let Some(edns) = &self.edns {
            msg.set_edns(edns.to_owned());
        }

        Ok(msg)
    }

    pub fn query_msg(&self, msg: Message) -> Result<String, DnsError> {
        let response = self.client.send(msg).first().ok_or(DnsError::NoResponse)?.to_owned()?;
        let data = response.answers().first().ok_or(DnsError::NoAnswer)?.data().ok_or(DnsError::NoData)?;
        Ok(data.to_string())
    }

    pub fn query(&mut self, url: String) -> Result<String, DnsError> {
        let msg = self.new_msg(url)?;
        self.query_msg(msg)
    }

    pub fn use_edns(&mut self, flag: bool) {
        self.edns = match flag {
            true => {
                let mut edns = Edns::new();
                edns.set_max_payload(4096);
                Some(edns)
            },
            false => None
        }
    }

}

#[derive(Error, Debug)]
pub enum DnsError {
    #[error("Couldn't connect to server! {0}")]
    Connection(ClientError),
    #[error("Error sending query: {0}")]
    Send(#[from] trust_dns_client::error::ClientError),
    #[error("Couldn't encode data in dns query: {0}")]
    InvalidCharater(#[from] ProtoError),
    #[error("No response received")]
    NoResponse,
    #[error("Response contains no records")]
    NoAnswer,
    #[error("Response contains no data")]
    NoData,
}

