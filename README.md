# iodine_rs
Rust implementation of the [iodine client](https://github.com/yarrick/iodine), to tunnel IPv4 through DNS.

## Prerequisites
- Rust

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

### Limitations
**Bandwith**

We are using [trust-dns-client](https://crates.io/crates/trust-dns-client) which is limited to a dns label size of 63 (as stated in [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035)).
The iodine protocol header uses 4 bits for the fragment count, which means an upstream packet can consists of max. 16 fragments.
Furthermore the upstream data gets base32 encoded (5 bytes are represented by 8 chars).
Therefore we can currently send 63 * (5/8) - 5 = 35 bytes per dns query (minus 5 because of upstream header).
This means the upstream packet size is limited to 16 * 35 = 560 bytes.
To stay within this limits we set a max mtu of 560.

**Blocking**

The read operation on the tun dev blocks execution until it reads some bytes.
This means currently we can get into situations, where we are waiting for downstream data but the read OP is blocking because there are no upstream data.
If this happens a sufficient workaround is to send a ping down the dev (e.g. `ping 192.168.99.1 -c 1`) 

