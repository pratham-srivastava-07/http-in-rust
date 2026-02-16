use std::io::{self, Write};
use std::net::UdpSocket;

fn main() -> io::Result<()> {
    // Bind to any available local port
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // Connect to the UDP server
    socket.connect("127.0.0.1:8080")?;

    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        
        print!("> ");
        io::stdout().flush()?; 

        input.clear();

        
        if let Err(e) = stdin.read_line(&mut input) {
            eprintln!("read error: {}", e);
            continue;
        }

        
        if let Err(e) = socket.send(input.as_bytes()) {
            eprintln!("send error: {}", e);
        }
    }
}
