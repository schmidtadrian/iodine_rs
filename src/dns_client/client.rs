use std::net::{UdpSocket, ToSocketAddrs};
use bytes::Bytes;

use super::{encode::create_query, decode::get_rdata, error::DnsError};

pub struct DnsClient {
    socket: UdpSocket,
    chunk_id: u16,
    pub use_edns: bool
}

impl DnsClient {
    pub fn new<T: ToSocketAddrs>(client_addr: T, server_addr: T) -> Result<Self, DnsError> {
        let socket = UdpSocket::bind(client_addr).map_err(|_| DnsError::Socket("Couldn't bind to socket".to_string()))?;
        socket.connect(server_addr).map_err(|_| DnsError::Socket("Couldn't connect to server".to_string()))?;
        Ok(DnsClient { socket, chunk_id: rand::random(), use_edns: false })
    }
    
    pub fn send(&self, bytes: &Bytes) -> Result<Vec<u8>, DnsError> {
        let mut buffer = [0u8; 1500];
        self.socket.send(bytes).map_err(|_| DnsError::Disconnected)?;
        let (len, addr) = self.socket.recv_from(&mut buffer).map_err(|_| DnsError::Receive)?;
        match addr == self.socket.peer_addr().map_err(|_| DnsError::Disconnected)? {
            true => Ok(buffer[..len].to_vec()),
            false => Err(DnsError::Timeout)
        }
    }

    pub fn query_data(&mut self, url: String) -> Result<Vec<u8>, DnsError>{
        let query = create_query(self.chunk_id(), url, self.use_edns);
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
