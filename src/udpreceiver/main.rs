use std::{io, net::UdpSocket};

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    println!("UDPreceiver listening on port 8080");

    // socket.connect("127.0.0.1:8080");    

    let mut buf = [0; 1024];

    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let msg = String::from_utf8_lossy(&buf[..amt]);

        println!("{} bytes received from address {}: {}", amt, src, msg);
    }
}