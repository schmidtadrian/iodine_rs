pub const DNS_HDR_SIZE: usize = 12;
pub const EDNS_EXT_SIZE: usize = 11;
pub const PERIODE: u8 = 0x2E;
pub const DNS_RESPONSE_FLAGS: &[u8] = &[0x84, 0x00];

// iodine dns answer name is always a pointer.
// 2 byte NAME pointer
// 2 byte TYPE
// 2 byte CLASS
// 4 byte TTL
// 2 byte RDLENGTH
pub const DNS_ANSWER_RLENGTH_INDEX: usize = 11;

pub const EDNS_EXT: &[u8] = &[
    0x00,           // NAME
    0x00, 0x29,     // TYPE OPT(41)
    0x10, 0x00,     // UDP payload size: 4096
    0x00,           // Higher bits extended RCODE: 0
    0x00,           // EDNS0 version: 0
    0x80, 0x00,     // Z (Accepts DNSSEC security RRs)
    0x00, 0x00      // Data length: 0
];
