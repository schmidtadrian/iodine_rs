use std::str::FromStr;

use trust_dns_client::{udp::UdpClientConnection, client::{SyncClient, Client}, rr::Name};


/// `socket` - e. g. "1.1.1.1:53", no support for hostnames
pub fn connect(socket: String) -> anyhow::Result<SyncClient<UdpClientConnection>>{
    let address = socket.parse().unwrap();
    let conn = UdpClientConnection::new(address)?;

    // and then create the Client
    Ok(SyncClient::new(conn))
}

pub fn query(name: &str, client: &SyncClient<UdpClientConnection>) -> anyhow::Result<String> {
    let name = Name::from_str(name).unwrap();
    let response = client.query(&name, trust_dns_client::rr::DNSClass::IN, trust_dns_client::rr::RecordType::TXT)?;
    let answer = response.answers();

    if let Some(data) = answer[0].data() {
        Ok(data.to_string())
    } else {
        Ok("".to_string())
    }
}

