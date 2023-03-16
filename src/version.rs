
pub fn send_packet() {}
pub fn data2url() {}

pub fn send_version(version: i32) {
    let seed = rand::random::<u16>();
    let bytes: [u8; 6] = [
        (version >> 24) as u8,
        (version >> 16) as u8,
        (version >>  8) as u8,
        version as u8,
        (seed >> 8) as u8,
        seed as u8
    ];
}
