use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use pkg::threadpool::ThreadPool;
use pkg::ultil::{parse_http_body, parse_route};

use crate::app::route::create_routes;
use crate::usecase::book::book::Book;
use crate::usecase::book::book_handler::{BookHandler, new_book_handler};
use crate::usecase::book::book_service::{BookService, new_book_service};

mod app;
mod config;
mod usecase;

#[tokio::main]
async fn main() {
    pkg::logger::init().expect("Error init log");
    let cfg = config::ApplicationConfig::load_yaml_config("./config/Settings.yaml".to_string());
    let db = pkg::database::connect(cfg.app_name.clone(), cfg.uri.clone(), cfg.db.clone()).await;
    let listener = TcpListener::bind(format!("127.0.0.1:{}", cfg.port.clone()))
        .expect("Failed to bind address");
    let pool = ThreadPool::new(cfg.thread_capacity);
    println!("HTTP server started at {}", cfg.port.clone());
    for stream in listener.incoming() {
        let c = db.clone();
        let stream = stream.expect("Connection failed");
        pool.execute(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let book_service = new_book_service(c);
            let book_handler = new_book_handler(Box::new(book_service));
            rt.block_on(create_routes(&stream, &book_handler));
        });
    }
    println!("Shutting down.");
}
