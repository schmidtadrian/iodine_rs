use bitfield::bitfield;
use serde::{Serialize, Deserialize};

const TYPE: u16 = 65432;

#[derive(Serialize, Deserialize)]
pub struct Query {
    pub id: u16,
    pub flags: u16,
    pub qd_count: u16,
    pub an_count: u16,
    pub ns_count: u16,
    pub ar_count: u16,
    //pub url: String,
    //pub q_type: u16,
    //pub class: u16
}

//impl Query {
//    pub fn serialize(&self) -> Vec<u8> {
//        let a = self.url.as_bytes();
//        let b = self.id.to_be_bytes();
//        let c = a.concat();
//        [
//            self.id.to_be_bytes(),
//            self.flags.to_be_bytes(),
//            self.qd_count.to_be_bytes(),
//            self.an_count.to_be_bytes(),
//            self.ns_count.to_be_bytes(),
//            self.ar_count.to_be_bytes(),
//            self.url.as_bytes().concat()
//            self.q_type.to_be_bytes(),
//            self.class.to_be_bytes()
//        ].concat()
//    }
//}

bitfield! {
    #[derive(Serialize, Deserialize)]
    pub struct DnsHeader([u8]);
    u8;
    // first & second byte
    pub u16, get_id,      set_id:       15, 0;
/*  // BIG ENDIAN ORDER
    // fields in third byte
    pub get_qr,      set_qr:          16;
    pub get_opcode,  set_opcode:  20, 17;
    pub get_aa,      set_aa:          21;
    pub get_tc,      set_tc:          22;
    pub get_rd,      set_rd:          23;
    // fields in fourth byte
    pub get_ra,      set_ra:          24;
    pub get_unused,  set_unused:      25;
    pub get_ad,      set_ad:          26;
    pub get_cd,      set_cd:          27;
    pub get_rcode,   set_rcode:   31, 28;
*/
    // LITTLE ENDIAN ORDER
    // fields in third byte
    pub get_rd,      set_rd:          16;   // recursion desired
    pub get_tc,      set_tc:          17;   // truncated message
    pub get_aa,      set_aa:          18;   // authoritive answer
    pub get_opcode,  set_opcode:  22, 19;   // purpose of message
    pub get_qr,      set_qr:          23;   // response flag
    // fields in fourth byte
    pub get_rcode,   set_rcode:   27, 24;   // response code
    pub get_cd,      set_cd:          28;   // checking disabled by resolver
    pub get_ad,      set_ad:          29;   // authentic data from named
    pub get_unused,  set_unused:      30;   // unused bits
    pub get_ra,      set_ra:          31;   // recursion available

    // remaining bytes
    pub u16, get_qdcount, set_qdcount: 47, 32;   // num of question entries
    pub u16, get_ancount, set_ancount: 63, 48;   // num of answer entries
    pub u16, get_nscount, set_nscount: 79, 64;   // num of authority entries
    pub u16, get_arcount, set_arcount: 95, 80;   // num of resource entries
}

#[repr(C, packed)]
#[derive(Serialize, Clone, Copy)]
pub struct DnsBody {
    t: u16,
    ns_c: u16,
    //root: u8,
    //opt: u16,
    //payload_size: u16,
    //edns_version: u16,
    //z: u16,
    //data_length: u16
}

//#[repr(C, packed)]
#[derive(Serialize)]
pub struct DnsPacket {
    a: DnsHeader<[u8; 15]>,
    url: String,
    b: DnsBody
}

pub fn dns_encode(url: String, mut id: u16, mut id_prev: u16, mut id_prev_2: u16) -> Vec<u8> {
    id_prev_2 = id_prev;
    id_prev = id;
    id += 7727;

    if id == 0 {
        // 0 is used as "no-query" by server
        id = 7727
    }

    let mut header = DnsHeader([0; 15]);
    header.set_id(id.to_be());
    header.set_qr(false);
    header.set_opcode(0);
    header.set_aa(false);
    header.set_tc(false);
    header.set_rd(true);
    header.set_ra(false);
    header.set_qdcount(1_u16.to_be());
    header.set_ancount(91_u16.to_be());
    header.set_nscount(92_u16.to_be());
    header.set_arcount(93_u16.to_be());
    //header.set_arcount(1_u16.to_be());

    let body = DnsBody {
        t: TYPE,
        ns_c: 1,
        //root: 0,
        //opt: 41,
        //payload_size: 4096,
        //edns_version: 0,
        //z: 32768,
        //data_length: 0 
    };

    //DnsPacket {
    //    a: header,
    //    url: url.clone(),
    //    b: body,
    //}

    //header
    let q = Query {
        id: id.to_be(),
        flags: 0x0100_u16.to_be(),
        qd_count: 1_u16.to_be(),
        an_count: 0_u16.to_be(),
        ns_count: 0_u16.to_be(),
        ar_count: 1_u16.to_be(),
        //url: url.into_bytes(),
        //q_type: TYPE.to_be(),
        //class: 1_u16.to_be()
    };

    let s_head = bincode::serialize(&q).unwrap();
    println!("{}, find {}", url, url.find('.').unwrap());
    let s_url = [url.len().to_be_bytes().to_vec(), url.into_bytes()].concat();
    let s_foot = bincode::serialize(&body).unwrap();
    println!("Head: {:?}", s_head);
    println!("URL: {:?}", s_url);
    println!("Foot: {:?}", s_foot);
    let x = [bincode::serialize(&q).unwrap(), s_url, bincode::serialize(&body).unwrap()].concat();
    println!("{:?}", x);
    x
}
