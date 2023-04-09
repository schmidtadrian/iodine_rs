use std::time::{Duration, SystemTime, Instant};
use client::Client;
use trust_dns_client::{rr::Record, op::Header};

use crate::dns_client::encode;

mod constants;
mod tun;
mod dns;
mod ping;
mod client;
mod util;
mod encoder;
mod downstream;
mod upstream;
mod handshake {
    pub mod client;
    pub mod version;
    pub mod login;
    pub mod edns;
    pub mod downstream;
    pub mod constants;
}
mod dns_client {
    pub mod encode;
    pub mod decode;
    pub mod client;
    pub mod error;
    pub mod constants;
}


//fn main() {
#[tokio::main]
async fn main() {

//    #[cfg(debug_assertions)]
//    println!("Debug mode");

    let mut client = match Client::new(
        client::ProtocolVersion::V502,
        "t2.adrian-s.de".to_string(),
        //"127.0.0.1".to_string(),
        "192.168.178.48".to_string(),
        //"40.113.151.92".to_string(),
        53,
        "secretpassword".to_string()).await {
            Ok(client) => client,
            Err(err) => return eprintln!("{}", err)
        };
//
//
//    const TIMEOUT: Duration = Duration::from_secs(3);
//    let mut now: SystemTime;// = SystemTime::now();
//
//    let mut ping_at = SystemTime::now().checked_add(TIMEOUT).unwrap();
//    let mut unhandled_downstream: Option<(Header, Record)> = None;
//
//    loop {
//        now = SystemTime::now();
//        if now > ping_at {
//            match client.send_ping().await {
//                Ok(resp) => {
//                    println!("Saving Ping response!");
//                    unhandled_downstream = Some(resp)
//                },
//                Err(err) => eprintln!("PING ERROR: {}", err)
//            }
//            ping_at = now.checked_add(TIMEOUT).unwrap();
//            continue;
//        }
//        // read tun
//        // None --> did not read & dont send upstream data
//        // Some --> compressed data from tun into client.out_pkt.data
//        if unhandled_downstream.is_none() {
//            if let Err(err) =  client.read_tun().await {
//                eprintln!("{}", err);
//            }
//
//            unhandled_downstream = match client.upstream().await {
//                Ok(option) => match option {
//                    Some(resp) => Some(resp),
//                    None => continue,
//                }
//                Err(err) => {
//                    eprintln!("upstream err: {}", err);
//                    return;
//                },
//            };
//        }
//
//
//        if let Some((header, record)) = unhandled_downstream {
//            println!("handle downstream");
//            match client.handle_downstream(header, record, &mut ping_at).await {
//                Ok(opt) => match opt {
//                    Some(_) => {},
//                    None => eprintln!("Received invalid data"),
//                },
//                Err(err) => eprintln!("downstream err: {}", err),
//            }
//            unhandled_downstream = None;
//        }
//    }
}
