use crate::{util::cmc, dns_client::error::DnsError};


impl crate::client::Client {


    pub async fn send_ping(&mut self) -> Result<Vec<u8>, DnsError>{

        // 1 byte user id
        // 1 byte with:
        //      1 bit unused (MSB)
        //      3 bit seq no
        //      4 bit frag
        // 2 byte cmc
        let cmc = cmc(&mut self.cmc);
        let bytes: &[u8; 4] = &[
            self.uid,
            ((self.in_pkt.seq_no & 0b111) << 4) | (self.in_pkt.fragment & 0b1111),
            cmc[0], cmc[1]
        ];

        #[cfg(debug_assertions)]
        println!("PING: {}/{}", self.in_pkt.seq_no, self.in_pkt.fragment);

        let url = "p".to_string() + &self.encoder.encode_default(bytes);
        self.dns_client.query_data(url)
    }

}
