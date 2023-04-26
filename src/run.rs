use std::time::{Duration, Instant};

use crate::client::Client;


impl Client {
    pub async fn run(&mut self, ping_interval: u64) {

        let timeout: Duration = Duration::from_secs(ping_interval);
        let mut now: Instant;

        let mut ping_at = Instant::now().checked_add(timeout).unwrap();
        let mut unhandled_downstream: Option<Vec<u8>> = None;

        loop {

            now = Instant::now();
            if now > ping_at {
                match self.send_ping().await {
                    Ok(resp) => unhandled_downstream = Some(resp) ,
                    Err(err) => eprintln!("PING ERROR: {}", err)
                }
                ping_at = now.checked_add(timeout).unwrap();
                continue;
            }

            if unhandled_downstream.is_none() {
                if let Err(_err) =  self.read_tun().await {
                    //eprintln!("{}", err);
                }

                unhandled_downstream = match self.upstream().await {
                    Ok(option) => match option {
                        Some(resp) => Some(resp),
                        None => continue,
                    }
                    Err(err) => {
                        eprintln!("upstream err: {}", err);
                        return;
                    },
                };
            }


            if let Some(data) = unhandled_downstream {
                match self.handle_downstream(data, &mut ping_at).await {
                    Ok(opt) => match opt {
                        Some(_) => {},
                        None => eprintln!("Received invalid data"),
                    },
                    Err(err) => eprintln!("downstream err: {}", err),
                }
                unhandled_downstream = None;
            }
        }
    }
}
