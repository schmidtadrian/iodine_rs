use clap::Parser;
use client::Client;

use crate::args::Args;
use crate::util::get_default_nameserver;


mod args;
mod constants;
mod tun;
mod ping;
mod client;
mod util;
mod encoder;
mod downstream;
mod upstream;
mod run;
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


    let mut client = match Client::handshake(
        client::ProtocolVersion::V502,
        args.domain,
        args.nameserver.unwrap_or_else(get_default_nameserver),
        args.port,
        args.password,
        args.downstream,
        args.interface
    ).await {
            Ok(client) => client,
            Err(err) => return eprintln!("{}", err)
        };

    client.run(args.interval).await;

}
