use std::{fs, net::IpAddr};

use crate::constants::{SYMBOLS, DATA_CMC_CHARS};

/// Converts byte into b32 char
pub fn b32_5to8(val: u8) -> char {
    // TODO consider storing SYMBOLS as byte array
    SYMBOLS.to_string().as_bytes()[val as usize % SYMBOLS.len()].into()
}

/// Increments `n` and returns it as a big endian byte array
pub fn cmc(n: &mut u16) -> [u8; 2] {
    let val = (*n).to_be_bytes().to_owned();
    // if we reach u16::max start over again
    *n = n.checked_add(1).unwrap_or(0);
    val
}

/// Converts cmc (2 byte) into 3 base32 chars
pub fn cmc_b32_5to8(n: &mut u16) -> String {
    let val = [
        b32_5to8(((*n >> 10) & 31) as u8),
        b32_5to8(((*n >> 5) & 31) as u8),
        b32_5to8((*n & 31) as u8)
    ];
    *n += 1;
    val.iter().collect()
}

/// Maps the user id into a hex char
/// e.g.
///      1 --> 1
///     10 --> a
///     15 --> f
///     16 --> 0
pub fn uid_to_char(n: u8) -> char {
     format!("{:x}", n%16).chars().next().unwrap()
}


/// Gets data cmc char for `n` & increments `n` afterwards
pub fn get_data_cmc_char(n: &mut u8) -> char {
    let val: char = DATA_CMC_CHARS.chars().nth(*n as usize).unwrap();
    *n += 1;
    if *n >= 36 { *n = 0 }
    val
}


/// Tries to get nameserver from `/etc/resolv.conf`. On failure returns `8.8.8.8`
pub fn get_default_nameserver() -> String {

    let mut nameserver = "8.8.8.8".to_string();
    let resolv = match fs::read_to_string("/etc/resolv.conf") {
        Ok(file) => file,
        Err(_err) => return nameserver
    };

    for line in resolv.lines() {
        if line.contains("nameserver") {
            if let Ok(ip) = line.replace("nameserver ", "").parse::<IpAddr>() {
                nameserver = ip.to_string()
            }
            println!("Using nameserver: {}", nameserver);
            break;
        }
    }

    nameserver
}
