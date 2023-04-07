use trust_dns_client::{rr::Record, op::Header};

use crate::{client::Client, util::{b32_5to8, get_data_cmc_char}, constants::MAX_URL_RAW_DATA_SIZE};

impl Client {

    pub fn build_hostname(&self, data: &[u8], max_len: usize) -> (String, u32) {
        //let avail_space: usize = usize::min(max_len, 4096) - domain.len() - 8;

        let data_len = usize::min(max_len, data.len());
        //#[cfg(debug_assertions)]
        //println!("Sending {} bytes from out_pkt.data, {} bytes left", data_len, data.len()-data_len);

        (
            self.encoder.encode_data(data[..data_len].into(), &self.domain),
            data_len as u32
        )
    }

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

    pub async fn upstream(&mut self) -> anyhow::Result<Option<(Header, Record)>>{

        let data_len = self.out_pkt.data.len() as u32;
        // all data published
        if data_len == 0 || self.out_pkt.offset >= data_len {
            return Ok(None)
        }
        let (data, len) = self.build_hostname(&self.out_pkt.data[self.out_pkt.offset as usize..], MAX_URL_RAW_DATA_SIZE);
        self.out_pkt.sent_len = len;
        
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
                self.out_pkt.sent_len, self.out_pkt.data.len() - self.out_pkt.offset as usize - self.out_pkt.sent_len as usize
            );
            //println!("{}{}", header, &data);
        }

        Ok(Some(self.dns_client.query_record(header + &data).await?))
    }
}
