use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ListOptions {
    pub field: String,
    pub sort: String,
    pub key: String,
}
