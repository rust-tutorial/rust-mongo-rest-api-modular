use std::collections::HashMap;
use std::convert::Infallible;
use std::io::ErrorKind;
use std::str::FromStr;

use async_trait::async_trait;
use bson::doc;
use bson::oid::{Error, ObjectId};
use futures::stream::TryStreamExt;
use mongodb::results::{DeleteResult, InsertOneResult};
use mongodb::{
    bson::to_document, bson::Document, options::FindOptions, Collection, Cursor, Database,
};
use warp::http::StatusCode;
use warp::reject::Reject;

use crate::usecase::book::book::Book;
use crate::usecase::book::book_filter::ListOptions;

#[async_trait]
pub trait Service {
    async fn create(&self, book: Book) -> mongodb::error::Result<InsertOneResult>;
    async fn load(
        &self,
        list_filter: HashMap<String, String>,
    ) -> mongodb::error::Result<Cursor<Document>>;
    async fn delete(&self, id: ObjectId) -> mongodb::error::Result<DeleteResult>;
}

#[derive(Clone)]
pub struct BookService {
    collection: Collection<Document>,
}

pub fn new_book_service(db: Database) -> BookService {
    BookService {
        collection: db.collection::<Document>("books"),
    }
}

#[async_trait]
impl Service for BookService {
    async fn create(&self, book: Book) -> mongodb::error::Result<InsertOneResult> {
        let doc = to_document(&book).expect("Error parsing");
        self.collection.insert_one(doc, None).await
    }

    async fn load(
        &self,
        list_filter: HashMap<String, String>,
    ) -> mongodb::error::Result<Cursor<Document>> {
        let mut filter: Document = Document::new();
        for k in list_filter.clone() {
            if k.0 == "sort" {
                //println!("F: {}", list_filter.clone()["field"].clone());
                filter = match k.1.as_str() {
                    "asc" => doc! { list_filter.clone()["field"].clone(): 1 },
                    "desc" => doc! { list_filter.clone()["field"].clone(): -1 },
                    _ => Document::new(),
                };
            } else if k.0 == "field" {
                continue;
            } else {
                filter.insert(k.0, k.1);
            }
        }
        let mut f = FindOptions::default();
        f.sort = Option::from(filter.clone());
        self.collection.find(None, f).await
    }

    async fn delete(&self, id: ObjectId) -> mongodb::error::Result<DeleteResult> {
        let doc = doc! {"_id": id};
        self.collection.delete_one(doc, None).await
    }
}
