use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn hello_response() -> &'static str {
    "HTTP/1.1 200 OK\r\ncontent-type: text/plain; charset=utf-8\r\ncontent-length: 16\r\n\r\nhello from rust\n"
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0_u8; 1024];
    let _bytes_read = stream.read(&mut buffer)?;
    stream.write_all(hello_response().as_bytes())?;
    stream.flush()
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("rust hello server listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        handle_connection(stream?)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::hello_response;

    #[test]
    fn hello_response_contains_plain_text_greeting() {
        let response = hello_response();

        assert!(response.starts_with("HTTP/1.1 200 OK"));
        assert!(response.ends_with("hello from rust\n"));
    }
}
