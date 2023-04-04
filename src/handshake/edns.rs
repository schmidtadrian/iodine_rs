use crate::{handshake::{client::ClientHandshake, constants::DOWN_CODEC_CHECK}, util::{b32_5to8, cmc_b32_5to8}};

impl ClientHandshake {
    pub fn edns_check(&mut self) -> anyhow::Result<()>{
        self.dns_client.use_edns(false);
        let url = format!("yt{}{}.{}", b32_5to8(1), cmc_b32_5to8(&mut self.cmc), &self.domain);
        let response = self.dns_client.query_data(url)?;
        let data = self.encoder.decode(response)?;

        let use_edns = data.eq(&DOWN_CODEC_CHECK);
        println!("Using edns0: {}", use_edns);

        if use_edns { self.dns_client.use_edns(true) }

        Ok(())
    }
}
