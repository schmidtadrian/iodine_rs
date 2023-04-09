use bytes::{Bytes, BytesMut, BufMut};
use super::constants::{DNS_HDR_SIZE, PERIODE, EDNS_EXT_SIZE, EDNS_EXT};


pub fn create_query(id: u16, url: String, edns: bool) -> Bytes {
    
    let qname = url_to_qname(url);
    // body size is qname len + 2 byte qtype + 2 byte qclass
    let mut query_size = DNS_HDR_SIZE + qname.len() + 4;
    let mut ar = 0;

    if edns {
        query_size += EDNS_EXT_SIZE;
        ar = 1;
    }
    let mut bytes = BytesMut::with_capacity(query_size);

    /*
     * DNS HEADER 
     * ID       <ID>
     * QR       0
     * OPCODE   0000
     * AA       0
     * TC       0
     * RD       1
     * RA       0
     * Z        000
     * RCODE    0000
     * QDCOUNT  0000_0001
     * ANCOUNT  0000_0000
     * NSCOUNT  0000_0000
     * ARCOUNT  0 or 1
     */
    bytes.put_u16(id);
    bytes.put_u8(1);
    bytes.put_u8(0);
    bytes.put_u16(1);
    bytes.put_u16(0);
    bytes.put_u16(0);
    bytes.put_u16(ar);

    // dns body
    bytes.put_slice(&qname);
    bytes.put_u16(16);   // QTYPE  TXT
    bytes.put_u16(1);    // QCLASS IN
    if edns { bytes.put_slice(EDNS_EXT) }

    bytes.freeze()
}


/// replaces periodes with the size of the following label
/// no fancy validation, shit in shit out!
pub fn url_to_qname(url: String) -> Vec<u8> {

    let mut data = url.as_bytes().to_vec();
    // insert periode for first label
    data.insert(0, PERIODE);

    for i in 0..data.len()-1 {
        if data[i] != PERIODE { continue }

        let mut count = 0;
        for v in data.iter().skip(i+1) {
            if *v != PERIODE {
                count += 1;
                continue
            }
            break
        }
        data[i] = count
    }

    // qname is null terminated
    data.push(0x00);
    data
}

