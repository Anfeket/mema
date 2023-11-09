use serde::{Serialize, Deserialize};

pub mod localdb;

#[derive(Serialize, Deserialize, Clone)]
pub struct Meme {
    pub author: String,
    pub url: String,
    pub media_type: MediaType
}
impl Meme {
    pub fn new(author: String, url: String, media_type: MediaType) -> Self { Self { author, url, media_type } }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum MediaType {
    Video,
    Image,
    GIF,
    Text,
}

#[derive(Debug)]
pub enum DatabaseError {
    NotFound,
    Duplicate,
    ConnectionError { details: String },
    QueryError { details: String },
    DatabaseError { details: String },
    Other(String),
}

pub struct QueryOptions {
    pub author: Option<String>,
    pub media_type: Option<MediaType>,
}

pub trait Database {
    // Create an item and return id
    fn create_item(&mut self, meme: Meme) -> Result<i32, DatabaseError>;

    // Find an item with specified id
    fn get_item(&self, id: i32) -> Result<Option<Meme>, DatabaseError>;

    // Update an item with id and new item
    fn update_item(&mut self, id: i32, meme: Meme) -> Result<(), DatabaseError>;

    // Remove an item with specified id
    fn remove_item(&mut self, id: i32) -> Result<Meme, DatabaseError>;
}
