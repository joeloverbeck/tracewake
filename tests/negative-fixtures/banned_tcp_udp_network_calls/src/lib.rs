use std::io;
use std::net::{TcpStream, UdpSocket};

pub fn touch_network() -> io::Result<()> {
    let _tcp = TcpStream::connect("127.0.0.1:9")?;
    let _udp = UdpSocket::bind("127.0.0.1:0")?;
    Ok(())
}
