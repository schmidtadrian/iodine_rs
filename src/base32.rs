
use crate::client::Client;


impl Client {
    pub fn build_hostname(&self, data: &[u8], domain: &String, max_len: usize) {
        let avail_space: usize = usize::min(max_len, 4096) - domain.len() - 8;
        let encode = "A"; //self.encoder.encode(data[..usize::min(avail_space, data.len())].into()) + "." + domain;
        println!("DEC: {:?}", data);
        println!("ENC: {}", encode);
    }
}
