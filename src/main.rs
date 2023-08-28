use std::io;

fn main() -> io::Result<()>{
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buffer = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buffer[..])?;
        let flags = u16::from_be_bytes([buffer[0], buffer[1]]);
        let proto = u16::from_be_bytes([buffer[2], buffer[3]]);
        eprintln!("read {} bytes: {:x?}", nbytes - 4, &buffer[4..nbytes]);
    }
    Ok(())
}
