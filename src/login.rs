
// data in
// password
// loginchallenge

// sending:
// 1 byte l or L
// 1 byte user id received from version handshake
// 16 bytes MD5 hash of: first 32 bytes of password XOR 8 repetitions of the login challenge
// e. g. 12 byte password will be written into bytes 0 till 11. Bytes 12-32 unset
// 2 byte CMC


use std::cmp::min;

use md5::{Md5, Digest};
use thiserror::Error;

use crate::{client::Client, trust_dns::query};

impl Client {
    
    /// Takes password & login challenge. Returns 16 byte md5 hash required for login handshake.
    /// `challenge` - 4 bytes received during version handshake
    /// `password` - password for iodine server
    pub fn calc_login(&self, password: String, challenge: u32) -> [u8; 16] {

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

    /// On success returns string: "server_ip-client_ip-mtu-netmask"
    pub fn login_handshake(&self, password: String, challenge: u32, uid: u8) -> anyhow::Result<(String, String, String, String)> {
        let hash = self.calc_login(password, challenge);
        let mut bytes: [u8; 19] = [0; 19];
        bytes[0] = uid;
        bytes[1..=16].copy_from_slice(&hash);
        bytes[17] = 0;
        bytes[18] = 0;
        let url = self.enc.encode(&bytes, 'l', &self.domain);

        let answer = match query(&url, &self.dns) {
            Ok(answer) => answer,
            Err(err) => return Err(LoginError::Query(err.to_string()).into())
        };

        let response_data = match self.enc.decode(answer) {
            Ok(data) => data,
            Err(err) => return Err(LoginError::Decode(err.to_string()).into())
        };


        let s = std::str::from_utf8(&response_data).unwrap_or("");
        match s {
            x if x.contains("LNAK") => return Err(LoginError::Unauthorized.into()),
            x if x.contains("BADIP") => return Err(LoginError::Uid.into()),
            x if x.is_empty() => return Err(LoginError::Unknown.into()),
            _ => println!("Login success! {}", s)
        }
        let mut fields = s.split('-');
        Ok((
            fields.next().unwrap_or("").to_string(), // server ip
            fields.next().unwrap_or("").to_string(), // client ip
            fields.next().unwrap_or("").to_string(), // mtu
            fields.next().unwrap_or("").to_string(), // netmask
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
    #[error("Unknown error")]
    Unknown
}

