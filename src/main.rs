use std::net::{TcpListener, SocketAddr};
use std::io::Write; // Needed for stream.write_all
pub mod utils;
mod internal;
use crate::internal::requests::request::request_from_reader;
use crate::internal::thread_pool::ThreadPool;
use crate::internal::response::Response;

fn main() -> std::io::Result<()> {
    let address = SocketAddr::from(([127, 0, 0, 1], 8080));

    let listener = TcpListener::bind(&address)?;
    let pool = ThreadPool::new(4); // Create a thread pool with 4 workers

    for stream in listener.incoming() {
        let mut stream = stream?;
        
        pool.execute(move || {
            println!("connection accepted");
            
            match request_from_reader(&mut stream) {
                Ok(request) => {
                    println!("{}", request);
                    
                    // Create a basic HTTP response
                    let mut response = Response::new(200, "OK");
                    response.set_header("Content-Type", "text/plain");
                    response.set_body(b"Hello from your custom Rust HTTP Server!".to_vec());

                    // Send response
                    if let Err(e) = stream.write_all(&response.to_bytes()) {
                        eprintln!("Failed to send response: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read request: {}", e);
                }
            }

            println!("connection closed");
        });
    } 

    Ok(())
}

