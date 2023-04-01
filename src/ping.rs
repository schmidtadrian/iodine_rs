use crate::util::cmc;

impl crate::client::Client {

    pub fn send_ping(&mut self) -> anyhow::Result<()>{
        // 1 byte user id
        // 1 byte with:
        //      1 bit unused (MSB)
        //      3 bit seq no
        //      4 bit frag
        // 2 byte cmc
        let cmc = cmc(&mut self.cmc);
        let bytes: &[u8; 4] = &[
            self.uid,
            ((self.in_pkt.seq_no & 7) << 4) | (self.in_pkt.fragment & 15),
            cmc[0], cmc[1]
        ];

        let url = self.encoder.encode(bytes, 'p', &self.domain);
        println!("PING: {}", url);
        let response = self.dns_client.query(url)?;
        let data = self.encoder.decode(response)?;
        let down_seq = (data[1] >> 5) & 7;
        let frag_no = (data[1] >> 1) & 15;
        let ack_seq = (data[0] >> 4) & 7;
        let ack_frag = data[0] & 15;
        println!("PONG: decode={:?}, seq={}, frag={}, ack_seq={}, ack_frag={}", data, down_seq, frag_no, ack_seq, ack_frag);

        Ok(())
    }

}
