/// server uses custom base32 alphabet
pub const SYMBOLS: &str = "abcdefghijklmnopqrstuvwxyz012345";
pub const DATA_CMC_CHARS: &str = "abcdefghijklmnopqrstuvwxyz0123456789";

/// unfortunately dns lib is limited to a max label len of 63
/// because base32 encodes 5 bytes in 8 chars we can calc the max data_len by:
/// 63 * (5/8) - 5 = 39
/// minus 5 because of upstream header
pub const MAX_URL_RAW_DATA_SIZE: usize = 35;
