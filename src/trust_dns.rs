use thiserror::Error;
use trust_dns_client::{udp::UdpClientConnection, client::{SyncClient, Client}, rr::{Name, RecordType, DNSClass}, op::{Message, Edns, Query, MessageType}};


/// `socket` - e. g. "1.1.1.1:53", no support for hostnames
pub fn connect(socket: String) -> anyhow::Result<SyncClient<UdpClientConnection>>{
    let address = socket.parse().unwrap();
    let conn = UdpClientConnection::new(address)?;

    // and then create the Client
    Ok(SyncClient::new(conn))
}

impl crate::client::Client {

    pub fn create_msg(edns: &Option<Edns>, url: String) -> Message {
        let mut msg = Message::new();
        msg.add_query({
            let mut query = Query::query(Name::from_ascii(url).unwrap(), RecordType::TXT);
            query.set_query_class(DNSClass::IN);
            query
        })
        .set_id(rand::random::<u16>())
        .set_message_type(MessageType::Query)
        .set_op_code(trust_dns_client::op::OpCode::Query)
        .set_recursion_desired(true);
    
        if let Some(ref edns) = &edns {
            msg.set_edns(edns.to_owned());
        }
    
        msg
    }

    pub fn create(&self, url: String) -> Message {
        Self::create_msg(&self.edns, url)
    }

    pub fn send(&self, msg: Message) -> Result<String, QueryError> {
        Self::send_query(&self.dns, msg)
    }

    pub fn send_query(client: &SyncClient<UdpClientConnection>, msg: Message) -> Result<String, QueryError> {
        let response = client.send(msg).first().ok_or(QueryError::NoResponse)?.to_owned()?;
        let data = response.answers().first().ok_or(QueryError::NoAnswer)?.data().ok_or(QueryError::NoData)?;
        Ok(data.to_string())
    }

    pub fn use_edns(flag: bool) -> Option<Edns> {
        match flag {
            true => {
                let mut edns = Edns::new();
                edns.set_max_payload(4096);
                Some(edns)
            },
            false => None
        }
    }

    pub fn send_ping(&self) {
        // 1 byte user id
        // 1 byte with:
        //      1 bit unused (MSB)
        //      3 bit seq no
        //      4 bit frag
        // 2 byte cmc
        let bytes: &[u8; 4] = &[
            0,
            ((self.in_pkt.seq_no & 7) << 4) | (self.in_pkt.fragment & 15),
            0,
            0,
        ];

        //let url = self.encoder(bytes, 'p', &self.domain);
        //println!("PING: {}", url);
    }

}



#[derive(Error, Debug)]
pub enum QueryError {
    //#[error("Error sending query: {0}")]
    //Send(String),
    #[error("Error sending query: {0}")]
    Send(#[from] trust_dns_client::error::ClientError),
    #[error("No response received")]
    NoResponse,
    #[error("Response contains no records")]
    NoAnswer,
    #[error("Response contains no data")]
    NoData,
}
