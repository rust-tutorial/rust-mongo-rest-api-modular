use std::net::TcpStream;
use std::io::Write;

pub fn json(mut stream: &TcpStream, contents: String, status_line: String) {
    let response = format!(
        "{}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).expect("Failed to write");
    stream.flush().expect("Failed to flush");
}
