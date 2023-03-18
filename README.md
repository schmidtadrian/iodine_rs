# iodine_rs
Rust implementation of the [iodine client](https://github.com/yarrick/iodine), to tunnel IPv4 through DNS.

## Prerequisites
- Rust

## Quickstart
Start the iodine server similar like:
```bash
sudo ./bin/iodined -f -c -P secretpassword 192.168.99.1 \<DOMAIN\>
```

Start the rust client (root privileges required):
```bash
cargo build
sudo target/debug/iodine_rs
```

## Status
DNS Type `TXT` is currently hard coded. Currently `base32` only, could be easily extended.
- [x] Version handshake
- [x] Login
- [x] Setup TUN dev
