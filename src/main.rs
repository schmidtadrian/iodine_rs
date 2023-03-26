use std::io::Read;
use data_encoding::Specification;
use dns::dns_encode;
use down_enc::DOWN_CODEC_CHECK;
use tokio::net::UdpSocket;
use trust_dns::{connect, query};
use trust_dns_client::client::Client;

use crate::tun::create_tun;

mod tun;
mod version;
mod base32;
mod dns;
mod trust_dns;
mod client;
mod login;
mod down_enc;

const IF_NAME: &str = "tun1";


//#[tokio::main]
//async fn main() -> std::io::Result<()> {
fn main() {

    let client = match client::Client::new(
        client::ProtocolVersion::V502,
        "t2.adrian-s.de".to_string(),
        //"40.113.151.92".to_string(),
        "127.0.0.1".to_string(),
        53,
    ) {
            Ok(client) => client,
            Err(err) => return eprintln!("{}", err)
        };
    client.init("secretpassword".to_string());
    match client.edns0_check() {
        Ok(data) => println!("Using edns: {}", data),
        Err(err) => println!("{}", err)
    }


    //let ver_payload: &[u8] = &[0,0,5,2,69,103];
    //let id = 0; //rand::random::<u16>();
    //let url = encode_url(ver_payload, 'v', "t2.adrian-s.de".to_string());
    //let client = connect();
    //let resp = query(url, &client);
    //decode(resp);

//    let packet = dns_encode(encode_url(ver_payload, 'v', "t2.adrian-s.de".to_string()), id, 0, 0);
//    
//    //println!("PACKET: {:#?}", packet);
//
//    //println!("HEADER: {:?}", header.get_qr());
//    let sock = UdpSocket::bind("0.0.0.0:8080").await?;
//
//    let remote_addr = "40.113.151.92:53";
//    sock.connect(remote_addr).await?;
//    //let bin_msg = bincode::serialize(&packet).unwrap();
//    //let bin_str = bincode::serialize(&packet.url).unwrap();
//    //println!("{:?}", bin_str);
//    sock.send(&packet).await?;
//    let mut buf = [0; 1024];
//    loop {
//        let len = sock.recv(&mut buf).await?;
//        println!("{:?} bytes received from {:?}", len, remote_addr);
//
//        let len = sock.send(&buf[..len]).await?;
//        println!("{:?} bytes sent", len);
//    }

    //let a: &[u8] = &[0,0,5,2,69,103];

    //let hex = {
    //    let mut spec = Specification::new();
    //    spec.symbols.push_str("abcdefghijklmnopqrstuvwxyz012345");
    //    spec.encoding().unwrap()
    //};
    //println!("{}", hex.encode(a));
    //println!("Encoded: {}", base32::encode_url(a, 'v', "t2.adrian-s.de".to_string()));
    //Ok(())
}
