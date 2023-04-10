use std::time::{Duration, Instant};
use crate::client::Client;


pub struct DownstreamHeader {
    pub downstream_seqno: u8,
    pub downstream_frag: u8,
    pub upstream_ack_seqno: u8,
    pub upstream_ack_frag: u8,
    pub last_fragment: bool
}

impl Client {
    fn parse_header(&self, data: [u8; 2]) -> DownstreamHeader {
        DownstreamHeader {
            downstream_seqno: (data[1] >> 5) & 7,
            downstream_frag: (data[1] >> 1) & 15,
            upstream_ack_seqno: (data[0] >> 4) & 7,
            upstream_ack_frag: data[0] & 15,
            last_fragment: (data[1] & 1) != 0
        }
    }

    pub async fn handle_downstream(&mut self, data: Vec<u8>, ping_at: &mut Instant, ) -> anyhow::Result<Option<()>> {

        // only continue for ping and data messages
        //if !record.name().to_ascii().starts_with(
        //    ['p', 'P', self.uid_char, self.uid_char.to_ascii_uppercase()]) { 
        //    return Ok(None);
        //}
        if data.is_empty() { return Ok(None); }

        //let data = record.data().ok_or(DnsError::NoData)?.to_string();
        let decoded = self.encoder.decode_byte(data)?;

        // responses < 2 bytes are incorrect!
        // TODO return error
        // TODO remove questionmark. Could be that decoded data has a len of 5 but is not a BADIP
        // response
        if decoded.len() < 2 { return Ok(None) }
        if decoded.len() == 5 && std::str::from_utf8(&decoded)?.contains("BADIP") { return Ok(None) }

        // decode the data
        let header = self.parse_header(decoded[..2].try_into()?);

        #[allow(clippy::unnecessary_operation)]
        // clippy wants to remove the labeled block. the block is required to use the break
        // statement!
        'block: {
            if decoded.len() > 2 {
                println!("GOT DATA");
                // on a new seq no forget the old stuff
                if header.downstream_seqno != self.in_pkt.seq_no {
                    //println!("NEW SEQ NO --> FORGET OLD STUFF");
                    self.in_pkt.reset_in(header.downstream_seqno, header.downstream_frag)
                } else if header.downstream_frag <= self.in_pkt.fragment {
                    //println!("Ignoring duplicate frag no");
                    break 'block
                } else {
                    self.in_pkt.fragment = header.downstream_frag
                }
                self.in_pkt.data.extend_from_slice(&decoded[2..]);

                if header.last_fragment { 
                    //println!("LAST FRAG!");
                    self.write_tun().await?
                }
                if self.in_pkt.data.is_empty() {
                    *ping_at = Instant::now().checked_add(Duration::from_millis(0)).unwrap();
                } else {
                    #[cfg(debug_assertions)]
                    println!("Server has more to send!");

                    *ping_at = Instant::now().checked_add(Duration::from_millis(0)).unwrap();
                }
            }
        };

        if self.is_sending() {

            #[cfg(debug_assertions)]
            println!("Got ack for {},{} - expecting {},{} ({} data bytes)",
                header.upstream_ack_seqno, header.upstream_ack_frag,
                self.out_pkt.seq_no, self.out_pkt.fragment,
                decoded.len()
            );

            if header.upstream_ack_seqno == self.out_pkt.seq_no
                && header.upstream_ack_frag == self.out_pkt.fragment
            {
                // prev sent fragment has arrived
                self.out_pkt.offset += self.out_pkt.sent_len;

                if self.out_pkt.offset >= self.out_pkt.data.len() as u32 {
                    // packet completed
                    println!("Packet {}/{} completed", self.out_pkt.seq_no, self.out_pkt.fragment);
                    self.out_pkt.offset = 0;
                    self.out_pkt.len = 0;
                    self.out_pkt.sent_len = 0;
                    self.out_pkt.data.clear()
                }
                else { 
                    // we have more to send
                    self.out_pkt.fragment += 1;
                }
            }
        } else {
            #[cfg(debug_assertions)]
            println!("Got ack for {},{} - expecting {},{} ({} data bytes) PING RESP",
                header.upstream_ack_seqno, header.upstream_ack_frag,
                self.out_pkt.seq_no, self.out_pkt.fragment,
                decoded.len()
            );
        }

        Ok(Some(()))
    }
}
