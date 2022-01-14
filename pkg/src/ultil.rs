use std::collections::HashMap;
use std::fmt::Display;
use std::io::{Error, Read};
use std::net::TcpStream;

#[derive(Debug)]
pub enum ParseBodyError {
    ReadStream(std::io::Error),
    EmptyBody,
}

impl Display for ParseBodyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseBodyError::ReadStream(io_error) => write!(f, "{}", io_error),
            ParseBodyError::EmptyBody => write!(f, "body is empty"),
        }
    }
}

impl std::error::Error for ParseBodyError {}

impl From<std::io::Error> for ParseBodyError {
    fn from(err: std::io::Error) -> Self {
        ParseBodyError::ReadStream(err)
    }
}

pub fn parse_http_body(raw_body: String) -> Result<String, ParseBodyError> {
    let v: Vec<&str> = raw_body.split("\r\n\r\n").collect();
    match v.last() {
        None => Err(ParseBodyError::EmptyBody),
        Some(body) => Ok(body.to_string()),
    }
}

pub fn parse_route(method: String, path: String) -> String {
    format!("{} {} HTTP/1.1\r\n", method, path)
}

pub fn load_query(q: String) -> HashMap<String, String> {
    let mut m: HashMap<String, String> = HashMap::new();
    let vec: Vec<&str> = q.split("?").collect();
    match vec.last() {
        None => m,
        Some(val) => {
            let v: Vec<&str> = val.split("&").collect();
            for i in 0..v.len() {
                let k: Vec<&str> = v[i].split("=").collect();
                if k.len() % 2 == 0 {
                    m.insert(k[0].to_string(), k[1].to_string());
                } else {
                    m.insert(k[0].to_string(), "".to_string());
                }
            }
            m
        }
    }
}

pub fn get_param(s: String) -> String {
    let tokens: Vec<&str> = s.split("/").collect();
    match tokens.last() {
        None => "".to_string(),
        Some(val) => val.to_string()
    }
}
