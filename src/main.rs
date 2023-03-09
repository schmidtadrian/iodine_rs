use std::io::Read;
use crate::tun::create_tun;

mod tun;

const IF_NAME: &str = "tun1";

fn main() {
    //TODO params as cli args
    let mut dev = create_tun(IF_NAME, "10.0.0.1", "255.255.255.0", false);

    let mut buf = [0; 4096];

	loop {
		let amount = dev.read(&mut buf).unwrap();
		//println!("{:?}", &buf[0 .. amount]);
        println!("Read {} bytes", amount);
    }
}

