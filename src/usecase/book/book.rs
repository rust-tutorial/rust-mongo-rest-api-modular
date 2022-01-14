use bson::oid::ObjectId;
use bson::serde_helpers::serialize_hex_string_as_object_id;
use bson::DateTime;
use chrono::MIN_DATETIME;
use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize, Serialize)]
pub struct Book {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub author: String,
    #[serde(with = "bson::serde_helpers::bson_datetime_as_rfc3339_string")]
    pub release: DateTime,
}

impl Book {
    pub fn book_to_response(&self) -> BookResponse {
        BookResponse {
            id: self.id.to_hex(),
            title: self.title.clone(),
            author: self.author.clone(),
            release: self.release.clone(),
        }
    }
}

impl Default for Book {
    fn default() -> Book {
        Book {
            id: ObjectId::new(),
            title: "".to_string(),
            author: "".to_string(),
            release: DateTime::from(MIN_DATETIME),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BookRequest {
    pub title: String,
    pub author: String,
    #[serde(with = "bson::serde_helpers::bson_datetime_as_rfc3339_string")]
    pub release: DateTime,
}

impl BookRequest {
    pub fn request_to_book(&self) -> Book {
        Book {
            id: ObjectId::new(),
            title: self.title.clone(),
            author: self.author.clone(),
            release: self.release.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BookResponse {
    pub id: String,
    pub title: String,
    pub author: String,
    #[serde(with = "bson::serde_helpers::bson_datetime_as_rfc3339_string")]
    pub release: DateTime,
}
