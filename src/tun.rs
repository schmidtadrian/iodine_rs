
use std::process;

use anyhow::Context;

/// Creates TUN dev.
/// Panicks on invalid parameters or insufficient privileges
pub fn create_tun(name: String, ip: String, netmask: String, pkt_info: bool) -> tun::platform::Device {
    // cidr netmask to byte array
    let mask: [u8; 4] = (u32::MAX << netmask.parse::<u8>().unwrap()).to_be_bytes();
    let mut config = tun::Configuration::default();
    config.name(&name)
        .address(&ip)
        .netmask((mask[0], mask[1], mask[2], mask[3]))
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

