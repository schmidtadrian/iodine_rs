
impl crate::client::Client {

    pub fn send_ping(&self) {
        // 1 byte user id
        // 1 byte with:
        //      1 bit unused (MSB)
        //      3 bit seq no
        //      4 bit frag
        // 2 byte cmc
        let bytes: &[u8; 4] = &[
            0,
            ((self.in_pkt.seq_no & 7) << 4) | (self.in_pkt.fragment & 15),
            0,
            0,
        ];

        //let url = self.encoder(bytes, 'p', &self.domain);
        //println!("PING: {}", url);
    }

}

