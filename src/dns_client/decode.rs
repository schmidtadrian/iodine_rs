use super::{constants::{DNS_RESPONSE_FLAGS, DNS_HDR_SIZE, DNS_ANSWER_RDATA_INDEX}, error::DnsError};


/// extracts data field from response
pub fn get_rdata(data: &[u8]) -> Result<Vec<u8>, DnsError> {

    if !is_valid_response(data) { return Err(DnsError::Malformed) }

    // skip hdr & search for end of query part
    // query is zero terminated followed by 2 bytes for qtype & 2 bytes for qclass
    let answer_start_index = DNS_HDR_SIZE + 4 + data
        .iter()
        .skip(DNS_HDR_SIZE)
        .position(|e| *e == 0x00)
        .ok_or(DnsError::Malformed)?;

    // TXT data reponse start with 1 byte TXT len followed by actual data
    let rdata_index = answer_start_index + DNS_ANSWER_RDATA_INDEX;
    let txt_len = data[rdata_index] as usize;
    Ok(data[rdata_index+1..rdata_index+1+txt_len].to_vec())
}

/// checks if flags are eq to `DNS_RESPONSE_FLAGS`
pub fn is_valid_response(response: &[u8]) -> bool {
    // TODO could also check TYPE & CLASS
    &response[2..4] == DNS_RESPONSE_FLAGS
}
