use std::net::{TcpListener, SocketAddr};
use crate::utils::get_lines_channel;
pub mod utils;


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