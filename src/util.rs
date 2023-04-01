use crate::constants::SYMBOLS;

/// Converts byte into b32 char
pub fn b32_5to8(val: u8) -> char {
    SYMBOLS.to_string().as_bytes()[val as usize % SYMBOLS.len()].into()
}

/// Increments `n` and returns it as a big endian byte array
pub fn cmc(n: &mut u16) -> [u8; 2] {
    let val = (*n).to_be_bytes().to_owned();
    *n += 1;
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
