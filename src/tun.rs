
use std::net::IpAddr;
use std::process;
use std::io::{Read, Write};

use anyhow::Context;
use flate2::Compression;
use flate2::write::ZlibEncoder;

use crate::client::Packet;

/// Creates TUN dev.
/// Panicks on invalid parameters or insufficient privileges
pub fn create_dev(name: String, ip: IpAddr, netmask: u32, mtu: i32, pkt_info: bool) -> tun::platform::Device {
    // cidr netmask to byte array
    let mask: [u8; 4] = (u32::MAX << netmask).to_be_bytes();
    let mut config = tun::Configuration::default();
    config.name(&name)
        .address(ip)
        .netmask((mask[0], mask[1], mask[2], mask[3]))
        .mtu(mtu)
        .up();
    
    #[cfg(target_os = "linux")]
    config.platform(|config| {
        config.packet_information(pkt_info);
    });

    match tun::create(&config).with_context(|| "Failed to create tun device".to_string()) {
        Ok(dev) => {
            println!("Created dev {} with ip {}/{}", name, ip, netmask);
            dev
        },
        Err(_err) => {
            println!("{:?}", _err);
            process::exit(exitcode::NOPERM)
        }
    }
}


impl crate::client::Client {
    pub fn read_tun(&mut self) {
        // read from tun
        //let mut in_buf = [0; 64*1024];
        //let in_len = self.tun.as_mut().unwrap().read(&mut in_buf).unwrap();
        let in_buf = "AAA".as_bytes();

        // compress 
        let mut enc = ZlibEncoder::new(Vec::new(), Compression::new(9));
        enc.write_all(in_buf).unwrap();
        let out_buf = enc.finish().unwrap();

        self.out_pkt = Packet { data: out_buf, ..*self.out_pkt.inc_seq_no() };
        //println!("Dec: {:?}", in_buf);
        //println!("Enc: {:?}", out_buf);
    }
}
