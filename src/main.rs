use crate::util::get_default_nameserver;
use std::time::{Duration, Instant};
use clap::Parser;
use client::Client;

use crate::args::Args;


mod args;
mod constants;
mod tun;
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


#[tokio::main]
async fn main() {
    let args = Args::parse();

    #[cfg(debug_assertions)]
    println!("Debug mode");

    let mut client = match Client::new(
        client::ProtocolVersion::V502,
        args.domain,
        args.nameserver.unwrap_or_else(get_default_nameserver),
        args.port,
        args.password,
        args.downstream
    ).await {
            Ok(client) => client,
            Err(err) => return eprintln!("{}", err)
        };


    let timeout: Duration = Duration::from_secs(args.interval);
    let mut now: Instant;

    let mut ping_at = Instant::now().checked_add(timeout).unwrap();
    let mut unhandled_downstream: Option<Vec<u8>> = None;

    loop {
        now = Instant::now();
        if now > ping_at {
            match client.send_ping().await {
                Ok(resp) => {
                    //println!("Saving Ping response!");
                    unhandled_downstream = Some(resp)
                },
                Err(err) => eprintln!("PING ERROR: {}", err)
            }
            ping_at = now.checked_add(timeout).unwrap();
            continue;
        }
        // read tun
        // None --> did not read & dont send upstream data
        // Some --> compressed data from tun into client.out_pkt.data
        if unhandled_downstream.is_none() {
            if let Err(_err) =  client.read_tun().await {
                //eprintln!("{}", err);
            }

            unhandled_downstream = match client.upstream().await {
                Ok(option) => match option {
                    Some(resp) => Some(resp),
                    None => continue,
                }
                Err(err) => {
                    eprintln!("upstream err: {}", err);
                    return;
                },
            };
        }


        if let Some(data) = unhandled_downstream {
            //println!("handle downstream");
            match client.handle_downstream(data, &mut ping_at).await {
                Ok(opt) => match opt {
                    Some(_) => {},
                    None => eprintln!("Received invalid data"),
                },
                Err(err) => eprintln!("downstream err: {}", err),
            }
            unhandled_downstream = None;
        }
    }
}
