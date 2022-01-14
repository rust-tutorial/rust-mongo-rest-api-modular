use std::collections::HashMap;
use std::io::Read;
use std::net::TcpStream;

use pkg::response::json;
use pkg::ultil::{load_query, parse_http_body, parse_route, get_param};

use crate::usecase::book::book_handler::{BookHandler, Handler};

pub async fn create_routes(mut stream: &TcpStream, h: &dyn Handler) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read stream");
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let mut request_content = String::from_utf8_lossy(&buffer[..]).to_string();
    request_content = request_content.trim_end_matches(char::from(0)).to_string();

    let url: Vec<&str> = request_content.split(" ").collect();
    let m = if url.len() > 2 {
        load_query(url[1].to_string().clone())
    } else {
        HashMap::new()
    };
    //println!("{:?}", m);
    let add = parse_route("POST".to_string(), "/book/add".to_string());
    let list = parse_route("GET".to_string(), "/book".to_string());
    if buffer.starts_with(add.as_bytes()) {
        match parse_http_body(request_content) {
            Ok(val) => {
                h.add(stream, val).await;
            }
            Err(err) => json(stream, err.to_string(), "HTTP/1.1 400 OK".to_string()),
        }
    } else if buffer.starts_with(list.as_bytes()) || buffer.contains(&b'&') {
        h.list(stream, m.clone()).await;
    } else if buffer.starts_with(b"DELETE") {
        let id = get_param(url[1].to_string().clone().clone());
        h.delete(stream, id).await;
    } else {
        json(stream, "Not Found".to_string(), "HTTP/1.1 404".to_string());
    }
}
