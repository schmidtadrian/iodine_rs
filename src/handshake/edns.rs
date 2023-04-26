use crate::{handshake::{client::ClientHandshake, constants::DOWN_CODEC_CHECK}, util::{b32_5to8, cmc_b32_5to8}};


impl ClientHandshake {


    pub async fn edns_check(&mut self) -> anyhow::Result<()>{

        self.dns_client.use_edns = false;
        let url = format!("y{}{}{}",
            self.encoder.up_enc_info.url_char,
            b32_5to8(1),
            cmc_b32_5to8(&mut self.cmc)
        );

        let response = self.dns_client.query_data(url)?;
        let data = self.encoder.decode_byte(response)?;

        let use_edns = data.eq(&DOWN_CODEC_CHECK);
        println!("Using edns0: {}", use_edns);

        if use_edns { self.dns_client.use_edns = true }

        Ok(())
    }
}
