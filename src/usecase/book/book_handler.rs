use std::collections::HashMap;
use std::convert::Infallible;
use std::io::Write;
use std::net::TcpStream;
use std::str::FromStr;

use async_trait::async_trait;
use bson::Document;
use bson::extjson::de::Error::InvalidObjectId;
use bson::oid::ObjectId;
use futures::TryStreamExt;
use log::{info, trace, warn};
use mongodb::Cursor;
use mongodb::error::Error;
use mongodb::results::{DeleteResult, InsertOneResult};
use pkg::response::json;
use pkg::ultil::{parse_http_body, ParseBodyError};
use warp::{Rejection, Reply};
use warp::http::StatusCode;
use warp::reject::Reject;
use warp::reply::with_status;

use crate::usecase::book::book::{Book, BookRequest};
use crate::usecase::book::book_filter::ListOptions;
use crate::usecase::book::book_service::Service;

#[async_trait]
pub trait Handler {
    async fn add(&self, mut stream: &TcpStream, body: String);
    async fn list(&self, mut stream: &TcpStream, list_option: HashMap<String, String>);
    async fn delete(&self, stream: &TcpStream, id: String);
}

pub struct BookHandler {
    service: Box<dyn Service + Send + Sync>,
}

pub fn new_book_handler(s: Box<dyn Service + Send + Sync>) -> BookHandler {
    BookHandler { service: s }
}

#[async_trait] // Currently async trait is not supported but the restriction will be removed in the future
impl Handler for BookHandler {
    async fn add(&self, stream: &TcpStream, body: String) {
        match serde_json::from_str::<BookRequest>(body.as_str()) {
            Ok(book_request) => {
                println!("{:#?}", book_request);
                let book = book_request.request_to_book();
                let w = self.service.create(book).await;
                match w {
                    Ok(result) => {
                        info!("inserted {} successfully", result.inserted_id.to_string());
                        json(
                            stream,
                            "book added".to_string(),
                            pkg::status_code::SUCCESS.to_string(),
                        );
                    }
                    Err(_) => json(
                        stream,
                        "internal server error".to_string(),
                        pkg::status_code::INTERNAL_SERVER_ERROR.to_string(),
                    ),
                }
            }
            Err(e) => {
                warn!("Unable to insert: {}", e);
                json(
                    stream,
                    "invalid post body".to_string(),
                    pkg::status_code::BAD_REQUEST.to_string(),
                );
                println!("{:#?}", e);
            }
        }
    }

    async fn list(&self, stream: &TcpStream, list_option: HashMap<String, String>) {
        let mut response = Vec::new();
        let result = self.service.load(list_option).await;
        match result {
            Ok(mut cursor) => {
                while let Some(doc) = cursor.try_next().await.expect("Error reading doc") {
                    let result = bson::from_bson::<Book>(bson::Bson::Document(doc));
                    match result {
                        Ok(book) => response.push(book.book_to_response()),
                        Err(err) => {
                            warn!("Error: {:?}", err);
                            json(
                                stream,
                                "Deserialize failed".to_string(),
                                pkg::status_code::INTERNAL_SERVER_ERROR.to_string(),
                            )
                        }
                    }
                }
                json(
                    stream,
                    serde_json::to_string(&response).unwrap(),
                    pkg::status_code::SUCCESS.to_string(),
                );
            }
            Err(err) => {
                warn!("Error: {:?}", err);
                json(
                    stream,
                    "Error reading cursor".to_string(),
                    pkg::status_code::INTERNAL_SERVER_ERROR.to_string(),
                )
            }
        }
    }

    async fn delete(&self, stream: &TcpStream, id: String) {
        let obj_id = ObjectId::from_str(&id);
        match obj_id {
            Ok(val) => {
                let result = self.service.delete(val).await;
                match result {
                    Ok(delete_result) => {
                        info!("Deleted {} document", delete_result.deleted_count);
                        json(
                            stream,
                            "Deleted Successfully".to_string(),
                            pkg::status_code::SUCCESS.to_string(),
                        )
                    }
                    Err(err) => {
                        warn!("{:?}", err);
                        json(
                            stream,
                            "Error reading cursor".to_string(),
                            pkg::status_code::INTERNAL_SERVER_ERROR.to_string(),
                        )
                    }
                }
            }
            Err(err) =>
                {
                    warn!("{:?}", err);
                    json(
                        stream,
                        "Invalid object ID".to_string(),
                        pkg::status_code::BAD_REQUEST.to_string(),
                    )
                }
        }
    }
}
