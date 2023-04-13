use std::net::IpAddr;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {

    /// FQDN delegated to the iodine server
    pub domain: String,

    /// IP of the relaying nameserver. If absent /etc/resolv.conf is used
    pub nameserver: Option<IpAddr>,

    /// Password
    #[arg(short, long)]
    pub password: String,

    /// Max downstream fragment size
    #[arg(short, long, default_value_t = 696)]
    pub downstream: u16,

    /// Ping interval (secs)
    #[arg(short, long, default_value_t = 3)]
    pub interval: u64,

    /// Server port
    #[arg(short = 'P', long, default_value_t = 53)]
    pub port: u16,

    /// Tunnel interface name
    #[arg(short = 'n', long = "name", default_value = "dns0")]
    pub interface: String
}
