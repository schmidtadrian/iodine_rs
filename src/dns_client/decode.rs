use super::{constants::{DNS_RESPONSE_FLAGS, DNS_HDR_SIZE, DNS_ANSWER_RLENGTH_INDEX}, error::DnsError};


/// extracts data field from response
pub fn get_rdata(data: &[u8]) -> Result<Vec<u8>, DnsError> {

    if let Err(err) = is_valid_response(data) {
        eprintln!("{}", err);
        return Ok(vec![])
    }

    // skip hdr & search for end of query part
    // query is zero terminated followed by 2 bytes for qtype & 2 bytes for qclass
    let answer_pos = DNS_HDR_SIZE + 4 + data
        .iter()
        .skip(DNS_HDR_SIZE)
        .position(|e| *e == 0x00)
        .ok_or(DnsError::Malformed)?;

    // RLENGTH has a size of 2 byte & tells us the length of the following data
    let rlength_pos = answer_pos + DNS_ANSWER_RLENGTH_INDEX;
    let data_len = read_u16(data, rlength_pos).map_err(|_| DnsError::Malformed)? as usize;
    let r_data = &data[rlength_pos+2..rlength_pos+2 + data_len];
    let mut response = Vec::with_capacity(r_data.len());

    // with edns its common, that the data section has multiple txt entries
    // a txt entry start with 1 byte indicating its length followed by the data
    // we need to concat the data
    let mut start: usize;
    let mut end: usize = 0;
    while end < r_data.len() {
        start = end+1;
        end = start + r_data[end] as usize;
        response.extend_from_slice(&r_data[start..end]);
    }

    Ok(response)
}

/// checks if flags are eq to `DNS_RESPONSE_FLAGS`
pub fn is_valid_response(response: &[u8]) -> Result<(), DnsError> {
    // TODO could also check TYPE & CLASS
    let is_response = response[2] >> 7 == 1;
    let is_error = response[3] & 0b1111 != 0;
    let has_answer = (response[6] > 0) || (response[7] > 0);

    if !is_response { return Err(DnsError::NoResponse) }
    if is_error { return Err(DnsError::ErrorResponse) }
    if !has_answer { return Err(DnsError::NoAnswer) }
    Ok(())
}

/// reads u16 from slice
fn read_u16(array: &[u8], pos: usize) -> anyhow::Result<u16> { //(array[pos] as u16) << 8 | array[pos+1] as u16
    Ok(u16::from_be_bytes(array[pos..pos+2].try_into()?))
}
