use std::{net::{UdpSocket, ToSocketAddrs}, time::Duration};
use bytes::Bytes;

use super::{encode::url_to_qname, decode::get_rdata, error::DnsError};

pub struct DnsClient {
    socket: UdpSocket,
    chunk_id: u16,
    pub use_edns: bool,
    /// convert domain to qname only once & reuse it
    pub qname_suffix: Vec<u8>,
}

impl DnsClient {
    pub fn new<C, S>(client_addr: C, server_addr: S, domain: String, read_timeout: u64) -> Result<Self, DnsError>
    where
        C: ToSocketAddrs,
        S: ToSocketAddrs
    {
        let socket = UdpSocket::bind(client_addr)
            .map_err(|_| DnsError::Socket("Couldn't bind to socket".to_string()))?;
        socket.connect(server_addr)
            .map_err(|_| DnsError::Socket("Couldn't connect to server".to_string()))?;

        if let Err(err) = socket.set_read_timeout(Some(Duration::from_millis(read_timeout))) {
            eprintln!("Couldn't set socket read timeout. {}", err);
        }

        Ok(DnsClient {
            socket,
            chunk_id: rand::random(),
            use_edns: false,
            qname_suffix: url_to_qname(domain)
        })
    }
    
    pub fn send(&self, bytes: &Bytes) -> Result<Vec<u8>, DnsError> {
        for i in 0..3 {
            let mut buffer = [0u8; 64*1024];
            self.socket.send(bytes).map_err(|_| DnsError::Disconnected)?;

            for j in 0..3 {
                if let Ok((len, addr)) = self.socket.recv_from(&mut  buffer) {
                    if addr == self.socket.peer_addr().map_err(|_| DnsError::Disconnected)? {
                        return Ok(buffer[..len].to_vec())
                    }
                    continue
                }
                println!("No data received (try: {})", j);
            }

            println!("Resending data (try: {})", i);
        }
        Err(DnsError::Timeout)
    }

    pub fn query_data(&mut self, url: String) -> Result<Vec<u8>, DnsError>{
        let chunk_id = self.chunk_id();
        let query = self.create_query(chunk_id, url, self.use_edns);
        let response = self.send(&query)?;
        get_rdata(&response)
    }

    /// adds 7727 to chunk id & returns new value
    /// on overflow we start from the beginning
    pub fn chunk_id(&mut self) -> u16 {
        self.chunk_id = match self.chunk_id.checked_add(7727) {
            Some(val) => val,
            None => 7727-(u16::MAX-self.chunk_id)
        };
        self.chunk_id
    }
}
