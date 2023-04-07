use std::{cmp::min, net::IpAddr, str::Split};

use md5::{Md5, Digest};
use thiserror::Error;
use crate::util::cmc;
use super::client::ClientHandshake;

impl ClientHandshake {

    /// Takes password & login challenge. Returns 16 byte md5 hash required for login handshake.
    /// `challenge` - 4 bytes received during version handshake
    /// `password` - password for iodine server
    pub fn calc_login(password: String, challenge: u32, ) -> [u8; 16] {
        // Copies the password into input.
        // This is required, bc iodine server hashes always over 32 byte no matter of password length
        let mut input: [u8; 32] = [0; 32];
        let pw_size = min(32, password.len());
        input[..pw_size].copy_from_slice(&password.as_bytes()[..pw_size]);

        // converts 4 input bytes into one u32 to XOR it with the login challenge & put it back into the array
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
    pub async fn login_handshake(
        &mut self,
        password: String,
        challenge: u32,
        uid: u8,
    ) -> anyhow::Result<(IpAddr, IpAddr, i32, u32)> {

        //  1 byte user id
        // 16 byte md5 hash
        //  2 byte cmc
        let bytes = &[
            uid.to_be_bytes().as_slice(),
            &Self::calc_login(password, challenge),
            &cmc(&mut self.cmc)
        ].concat();

        let url = self.encoder.encode(bytes, 'l', &self.domain);
        let response = self.dns_client.query_data(url).await?;
        let data = self.encoder.decode_to_string(response)?;


        match data {
            s if s.contains("LNAK") => return Err(LoginError::Unauthorized.into()),
            s if s.contains("BADIP") => return Err(LoginError::Uid.into()),
            s if s.is_empty() => return Err(LoginError::Unknown.into()),
            _ => println!("Login successful!")
        }

        let fields = &mut data.split('-');
        // ugly split parsing
        fn parse_split<T: std::str::FromStr>(f: &mut Split<char>) -> Result<T, LoginError> {
            f.next().ok_or(LoginError::UnexpectedResponse)?.parse().map_err(|_| LoginError::UnexpectedResponse)
        }

        let server_ip = parse_split::<IpAddr>(fields)?;
        let client_ip = parse_split::<IpAddr>(fields)?;
        let mtu = parse_split::<i32>(fields)?;
        let netmask = parse_split::<u32>(fields)?;
        Ok((server_ip, client_ip, mtu, netmask))
    }
}

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("Wrong password")]
    Unauthorized,
    #[error("Bad user id")]
    Uid,
    #[error("Can't handle server response")]
    UnexpectedResponse,
    #[error("Unknown error")]
    Unknown
}
