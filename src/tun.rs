
use std::process;

use anyhow::Context;

pub fn create_tun(name: String, ip: String, netmask: String, pkt_info: bool) -> tun::platform::Device {
    //TODO panicks on invalid address & netmask
    //TODO handle netmask, need to convert from cidr
    let mut config = tun::Configuration::default();
    config.name(name)
        .address(ip)
        .netmask((255, 255, 255, 0))
        .up();
    
    #[cfg(target_os = "linux")]
    config.platform(|config| {
        config.packet_information(pkt_info);
    });

    match tun::create(&config).with_context(|| "Failed to create tun device".to_string()) {
        Ok(dev) => {
            println!("OK");
            dev
        },
        Err(_err) => {
            println!("{:?}", _err);
            process::exit(exitcode::NOPERM)
        }
    }
}

