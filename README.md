# iodine_rs
Rust implementation of the [iodine client](https://github.com/yarrick/iodine), to tunnel IPv4 through DNS.

## Prerequisites
- Rust
- C compiler
- cmake

## Quickstart
Start the iodine server similar like:
```bash
sudo ./bin/iodined -f -c -P secretpassword 192.168.99.1 <DOMAIN>
```

Start the rust client (root privileges required):
```bash
cargo build
sudo target/debug/iodine_rs
```

## Status
DNS Type `TXT` is currently hard coded. Currently `base32` only, could be easily extended.
- [ ] Handshake
    - [x] Version handshake
    - [x] Login
    - [x] Setup TUN dev
    - [x] Check edns0 support
    - [ ] Set upstream encoding (using default)
    - [x] Set downstream encoding
    - [x] Set fragment size
- [x] Data transfer

### Intermediate DNS server
Nearly all DNS server have some kind of mechanism to block spam or dns tunneling.
We tested with the following Intermediates:
- google (works best!)
- cloudflare (high latency)
- eduroam (doesn't work)

### Further work
- Support more encodings
- Async TUN & DNS (may also send & recv)

