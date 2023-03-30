
// data in
// password
// loginchallenge

// sending:
// 1 byte l or L
// 1 byte user id received from version handshake
// 16 bytes MD5 hash of: first 32 bytes of password XOR 8 repetitions of the login challenge
// e. g. 12 byte password will be written into bytes 0 till 11. Bytes 12-32 unset
// 2 byte CMC


use std::{cmp::min, net::IpAddr};

use data_encoding::Encoding;
use md5::{Md5, Digest};
use thiserror::Error;
use trust_dns_client::{client::SyncClient, udp::UdpClientConnection};

use crate::client::Client;

impl Client {

    /// Takes password & login challenge. Returns 16 byte md5 hash required for login handshake.
    /// `challenge` - 4 bytes received during version handshake
    /// `password` - password for iodine server
    pub fn calc_login(password: String, challenge: u32, ) -> [u8; 16] {
        // Copies the password into input.
        // This is required, bc iodine server hashes always over 32 byte no matter of password length
        let mut input: [u8; 32] = [0; 32];
        let pw_size = min(32, password.len());
        input[..pw_size].copy_from_slice(&password.as_bytes()[..pw_size]);

        // converts four input bytes into one u32 to XOR it with the login challenge & put it back into the array
        for i in 1..=8 {
            let end = 4*i;
            let start =  end-4;
            // should be safe to unwrap, bc of fix sizes
            let tmp: &mut u32 = &mut u32::from_be_bytes(input[start..end].try_into().unwrap());
            *tmp ^= challenge;
            input[start..end].copy_from_slice(&tmp.to_be_bytes());
        }
        
        let mut hasher = Md5::new();
        hasher.update(input);

        hasher.finalize().try_into().expect("HASH ERR")
    }

    /// On success returns string: `<server_ip>-<client_ip>-<mtu-netmask>`
    pub fn login_handshake(
        password: String,
        challenge: u32,
        uid: u8,
        domain: &String,
        cmc: &mut u16,
        encoder: &Encoding,
        dns: &SyncClient<UdpClientConnection>
    ) -> anyhow::Result<(IpAddr, IpAddr, i32, u32)> {

        //  1 byte user id
        // 16 byte md5 hash
        //  2 byte cmc
        let bytes = [
            uid.to_be_bytes().as_slice(),
            Self::calc_login(password, challenge).as_slice(),
            cmc.to_be_bytes().as_slice()
        ].concat();
        *cmc += 1;

        let url = Self::encode(&bytes, 'l', domain, encoder);
        let msg = Self::create_msg(&None, url);
        let answer = Self::send_query(dns, msg)?;
        let decoded = Self::decode_to_string(answer, encoder)?;

        match decoded {
            s if s.contains("LNAK") => return Err(LoginError::Unauthorized.into()),
            s if s.contains("BADIP") => return Err(LoginError::Uid.into()),
            s if s.is_empty() => return Err(LoginError::Unknown.into()),
            _ => println!("Login successful!")
        }

        // ugly parsing, may a struct is better
        let mut fields = decoded.split('-');
        let server_ip: IpAddr = fields.next().ok_or(LoginError::UnexpectedResponse)?.parse().map_err(|_| LoginError::UnexpectedResponse)?;
        let client_ip: IpAddr = fields.next().ok_or(LoginError::UnexpectedResponse)?.parse().map_err(|_| LoginError::UnexpectedResponse)?;
        let mtu: i32 = fields.next().ok_or(LoginError::UnexpectedResponse)?.parse().map_err(|_| LoginError::UnexpectedResponse)?;
        let netmask: u32 = fields.next().ok_or(LoginError::UnexpectedResponse)?.parse().map_err(|_| LoginError::UnexpectedResponse)?;

        Ok((
            server_ip,
            client_ip,
            mtu,
            netmask
        ))
    }
}

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("Query error: {0}")]
    Query(String),
    #[error("Decode error: {0}")]
    Decode(String),
    #[error("Wrong password")]
    Unauthorized,
    #[error("Bad user id")]
    Uid,
    #[error("Can't handle server response")]
    UnexpectedResponse,
    #[error("Unknown error")]
    Unknown
}

