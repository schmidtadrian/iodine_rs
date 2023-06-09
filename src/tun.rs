
use std::net::IpAddr;
use std::process;
use std::io::{Read, Write};
use std::time::Duration;

use anyhow::Context;
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::timeout;


/// Creates TUN dev.
/// Panicks on invalid parameters or insufficient privileges
pub fn create_dev(name: String, ip: IpAddr, netmask: u32, mtu: i32, pkt_info: bool) -> tun::AsyncDevice {
    // cidr netmask to byte array
    let mask: [u8; 4] = (u32::MAX << netmask).to_ne_bytes();
    let mut config = tun::Configuration::default();
    config.name(&name)
        .address(ip)
        .netmask((mask[3], mask[2], mask[1], mask[0]))
        .mtu(mtu)
        .up();
    
    #[cfg(target_os = "linux")]
    config.platform(|config| {
        config.packet_information(pkt_info);
    });

    match tun::create_as_async(&config).with_context(|| "Failed to create tun device".to_string()) {
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
    pub fn is_sending(&mut self) -> bool {
        !self.out_pkt.data.is_empty()
    }


    pub async fn read_tun(&mut self) -> anyhow::Result<Option<()>> {
        // read from tun

        if self.is_sending() {
            println!("Not reading from tun bc sending");
            return Ok(None)
        }
        let mut in_buf = [0; 2*1024];

        // early exit if we are currently sending data or read 0 bytes
        // read blocks if tun is empty
        let in_size = match timeout(Duration::from_millis(50), self.tun.read(&mut in_buf)).await {
            Ok(Ok(size)) => size,
            Ok(Err(err)) => {
                eprintln!("{}", err);
                return Err(err.into());
            },
            Err(err) => {
                return Err(err.into());
            },
        };

        if in_size == 0 {
            return Ok(None)
        }

        // compress 
        // for small buffers the encoded data is bigger than before
        // TODO I guess its better to keep encoder & resetting it instead of creating a new one each time
        let mut enc = ZlibEncoder::new(Vec::new(), Compression::new(8));
        enc.write_all(&in_buf[..in_size])?;
        let out_buf = enc.finish()?;

        self.out_pkt.reset_out(&out_buf);

        #[cfg(debug_assertions)]
        println!("Written {}/{} bytes to upstream data (raw/enc)", in_size, self.out_pkt.data.len());

        Ok(Some(()))
    }


    /// Decompresses `self.in_pkt.data` and writes it to `self.tun`
    /// On success `self.in_pkt.data` gets cleared
    pub async fn write_tun(&mut self) -> anyhow::Result<()> {

        // TODO I guess its better to keep decoder & resetting it instead of creating a new one each time
        let mut dec = ZlibDecoder::new(self.in_pkt.data.as_slice());
        let mut buf = Vec::new();
        dec.read_to_end(&mut buf)?;

        #[cfg(debug_assertions)]
        println!("Decompressed: {} to {} bytes", self.in_pkt.data.len(), buf.len());

        self.tun.write_all(&buf).await?;
        self.in_pkt.data.clear();
        Ok(())
    }
}
