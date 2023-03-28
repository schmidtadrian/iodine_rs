use crate::base32::SYMBOLS;

pub fn b32_5to8(val: usize) -> char {
    SYMBOLS.to_string().as_bytes()[val%SYMBOLS.len()].into()
}
