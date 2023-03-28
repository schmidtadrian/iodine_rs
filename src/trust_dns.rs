use std::str::FromStr;

use anyhow::Context;
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

    pub fn use_edns(&mut self, flag: bool) {
        self.edns = match flag {
            true => {
                let mut edns = Edns::new();
                edns.set_max_payload(4096);
                Some(edns)
            },
            false => None
        };
    }

    pub fn create_msg(&self, url: String) -> Message {
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
    
        if let Some(ref edns) = &self.edns {
            msg.set_edns(edns.to_owned());
        }
    
        msg
    }

    pub fn send_query(&self, msg: Message) -> Result<String, QueryError> {
        let response = self.dns.send(msg).first().ok_or(QueryError::NoResponse)?.to_owned()?;
        let data = response.answers().first().ok_or(QueryError::NoAnswer)?.data().ok_or(QueryError::NoData)?;
        Ok(data.to_string())
        //let response = self.dns.send(msg)[0].context(format!("Failed to send query"))?;
        //match &self.dns.send(msg)[0] {
        //    Ok(response) => 
        //        if let Some(data) = response.answers()[0].data() { Ok(data.to_string()) }
        //        else { Err(QueryError::NoData.into()) },
        //    Err(err) => Err(QueryError::Send(err.to_string()).into()),
        //}
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
