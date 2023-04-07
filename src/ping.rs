use trust_dns_client::{op::{DnsResponse, Header}, rr::Record};

use crate::{util::cmc, dns::DnsError};

impl crate::client::Client {

    pub fn send_ping(&mut self) -> Result<(Header, Record), DnsError>{
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

        #[cfg(debug_assertions)]
        println!("PING: {}/{}", self.in_pkt.seq_no, self.in_pkt.fragment);

        let url = self.encoder.encode(bytes, 'p', &self.domain);
        self.dns_client.query_record(url)
    }

}
