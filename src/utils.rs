use std::io::Read;
use std::net::TcpStream;


pub fn get_lines_channel(mut stream: TcpStream) -> std::io::Result<()> {
    //  let mut file = File::open("message.txt")?;
    let mut buffer = [0u8; 8];
    // file.read_exact(&mut buffer)?;
    println!("{:?}", buffer);

    let mut current_line = String::new();

    loop {
        let bytes_read = stream.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        // converting only the bytes that were actually read
        let chunks = String::from_utf8_lossy(&buffer[..bytes_read]);

        //splitting by new line 
        let parts: Vec<&str> = chunks.split('\n').collect();

        for i in 0..parts.len() {
            if i < parts.len() - 1 {
                current_line.push_str(parts[i]);
                println!("{}", current_line);
                current_line.clear();
            } else {
                current_line.push_str(parts[i]);
            }
        }
        // Ok(())

    }

    if !current_line.is_empty() {
        println!("{}", current_line);
    }

    Ok(())
}