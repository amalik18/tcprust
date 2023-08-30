use std::collections::HashMap;
use std::io;
use std::net::Ipv4Addr;

mod tcp;

struct TcpState {}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct TcpPair {
    // (IPaddr, port#)
    src: (Ipv4Addr, u16),
    dest: (Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
    let mut connections: HashMap<TcpPair, tcp::State> = Default::default();
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buffer = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buffer[..])?;
        let eth_flags = u16::from_be_bytes([buffer[0], buffer[1]]);
        let eth_proto = u16::from_be_bytes([buffer[2], buffer[3]]);
        if eth_proto != 0x0800 {
            // [Link to Proto number](https://www.iana.org/assignments/ieee-802-numbers/ieee-802-numbers.xhtml)
            // we don't want anything but IPv4 packets
            continue;
        };
        match etherparse::Ipv4HeaderSlice::from_slice(&buffer[4..nbytes]) {
            Ok(ip_packet) => {
                let src_addr = ip_packet.source_addr();
                let dest_addr = ip_packet.destination_addr();
                if ip_packet.protocol() != 0x06 {
                    // We don't want anything but TCP packets
                    continue;
                }
                match etherparse::TcpHeaderSlice::from_slice(&buffer[4 + ip_packet.slice().len()..])
                {
                    Ok(tcp_segment) => {
                        let data = 4 + ip_packet.slice().len() + tcp_segment.slice().len();
                        connections
                            .entry(TcpPair {
                                src: (src_addr, tcp_segment.source_port()),
                                dest: (dest_addr, tcp_segment.destination_port()),
                            })
                            .or_default()
                            .on_packet(ip_packet, tcp_segment, &buffer[data..]);
                    }
                    Err(e) => {
                        eprintln!("Ignoring this tcp segment {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Ignoring this ip packet {:?}", e);
            }
        }
    }
    Ok(())
}
