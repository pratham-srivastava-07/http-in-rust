use std::net::{SocketAddr, TcpListener};
use httpfromtcp::utils::get_lines_channel;


fn main() -> std::io::Result<()> {
    let address = SocketAddr::from(([127, 0, 0, 1], 8080));

    let listener = TcpListener::bind(&address)?;

    for stream in listener.incoming() {

        let stream = stream?;
        println!("connection accepted");
        get_lines_channel(stream)?;
        println!("connection closed");
    }

    Ok(())
}

