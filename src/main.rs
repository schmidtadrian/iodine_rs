use std::time::{Duration, SystemTime};
use client::Client;

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


//#[tokio::main]
//async fn main() -> std::io::Result<()> {
fn main() {

    #[cfg(debug_assertions)]
    println!("Debug mode");

    //client.read_tun();
    ////client.enc.build_hostname(b"AAA", &client.domain, 255);
    //client.send_ping();
    let mut client = match Client::new(
        client::ProtocolVersion::V502,
        "t2.adrian-s.de".to_string(),
        //"127.0.0.1".to_string(),
        //"192.168.178.48".to_string(),
        "40.113.151.92".to_string(),
        53,
        "secretpassword".to_string()) {
            Ok(client) => client,
            Err(err) => return eprintln!("{}", err)
        };


    const TIMEOUT: Duration = Duration::from_secs(3);
    let mut now = SystemTime::now();
    loop {
        if now.elapsed().unwrap() > TIMEOUT {
            if let Err(err) = client.send_ping() {
                eprintln!("PING ERROR: {}", err);
            }
            now = SystemTime::now();
            continue;
        }
        // read tun
        // None --> did not read & dont send upstream data
        // Some --> compressed data from tun into client.out_pkt.data
        if let Err(err) =  client.read_tun() {
            eprintln!("{}", err);
        }

        let (header, record) = match client.upstream() {
            Ok(option) => match option {
                Some((hdr, rec)) => (hdr, rec),
                None => continue,
            }
            Err(err) => {
                eprintln!("upstream err: {}", err);
                return;
            },
        };
        match client.handle_downstream(header, record) {
            Ok(opt) => match opt {
                Some(_) => {},
                None => eprintln!("Received invalid data"),
            },
            Err(err) => eprintln!("downstream err: {}", err),
        }
    }
}
