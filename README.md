# iodine_rs -- IPv4-over-DNS
Iodine_rs is a Rust implementation of the [iodine DNS-tunnel client](https://github.com/yarrick/iodine).
It encodes IPv4 packets in DNS queries and sends them to a DNS server (iodine server).
This can be used to bypass prevailing network restrictions.

## Prerequisites
To use this client in public networks, one need:
- Control over a real domain (e.g. `test.com`).
- An iodine server running on a machine with a public IPv4 address.
- A subdomain (e.g. `t1.test.com`) delegated to the iodine server.

To build the client, one need:
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

## How does it work?
**Client side (upstream)**:
* The client reads data from a TUN device, which represents a common interface to other applications.
* The data gets compressed using gzip.
* The compressed data is chopped into fragments.
* The fragments are encoded into base64.
* The client sends DNS queries of the following format to the iodine server:\
`<base64 encoded fragment>.t1.test.com`

**Server side**
* The server receives DNS queries and decodes the base64 fragments.
* The decoded fragments get reassembled.
* The data gets decompressed.
* The resulting IPv4 packet is forwarded to its destination.
* The response from the destination is encoded into a DNS response, in the same way, the client does encode DNS queries and sent back to the client.

**Client side (downstream)**
* The client receives DNS responses, decodes, reassembles, and decompresses them in the same way the server does.
* The reassembled IPv4 packets are sent back to the TUN device.


## Status
### Performance
| Upstream BW | Description |
| --- | --- |
| 1 Mbit/s | within a LAN |
| 220 Kbit/s | direct connection to a publicly hosted iodine server |
| 189 Kbit/s | with Google DNS as an intermediate DNS server |

### Intermediate DNS server
Nearly all DNS server have some kind of mechanism to block spam or DNS tunneling.
We tested with the following Intermediates:
- google (works best!)
- cloudflare (high latency)
- eduroam (doesn't work)

# Further work
- Support more encodings
- Async TUN & DNS (may also send & recv)

