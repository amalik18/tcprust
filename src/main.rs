use std::io;

fn main() -> io::Result<()>{
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buffer = [0u8; 1504];
    let nbytes = nic.recv(&mut buffer[..])?;
    eprintln!("read {} bytes: {:x?}", nbytes, &buffer[..nbytes]);
    Ok(())
}
