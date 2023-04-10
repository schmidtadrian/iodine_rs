use crate::{client::Client, util::{b32_5to8, get_data_cmc_char}};


impl Client {

    fn build_header(&mut self) -> String {

        let header = vec![
            // 1 byte user id as hex char
            self.uid_char,
            b32_5to8(
                // 3 bit upstream seq_no
                ((self.out_pkt.seq_no & 0b111) << 2) |
                // 2 bit upper upstream fragment
                ((self.out_pkt.fragment & 0b1111) >> 2)
            ),
            b32_5to8(
                // 2 bit lower upstream fragment
                ((self.out_pkt.fragment & 0b11) << 3) |
                // 3 bit downstream seq_no
                (self.in_pkt.seq_no & 0b111)
            ),
            b32_5to8(
                // 4 bit downstream fragment
                ((self.in_pkt.fragment & 0b1111) << 1) |
                // 1 bit last fragment flag
                self.out_pkt.is_last_fragment() as u8
            ),
            get_data_cmc_char(&mut self.data_cmc)
        ];

        header.iter().collect()
    }

    pub async fn upstream(&mut self) -> anyhow::Result<Option<Vec<u8>>>{

        let data_len = self.out_pkt.data.len() as u32;
        // all data published
        if data_len == 0 || self.out_pkt.offset >= data_len {
            return Ok(None)
        }

        // calc how move raw data we can encode in query
        let start = self.out_pkt.offset as usize;
        let end = self.out_pkt.data.len().min(self.url_max_raw_bytes + start);
        let data = self.encoder.enc(&self.out_pkt.data[start..end]);

        self.out_pkt.sent_len = (end-start) as u32;
        let header = self.build_header();

        #[cfg(debug_assertions)] {
            let lf = match self.out_pkt.is_last_fragment() {
                true => ", LF",
                false => ""
            };
            println!("Send: down {}/{} up {}/{}{}, {} data, {} left bytes",
                self.in_pkt.seq_no, self.in_pkt.fragment,
                self.out_pkt.seq_no, self.out_pkt.fragment,
                lf,
                self.out_pkt.sent_len,
                self.out_pkt.data.len() - self.out_pkt.offset as usize - self.out_pkt.sent_len as usize
            );
        }

        Ok(Some(self.dns_client.query_data(header + &data)?))
    }
}
