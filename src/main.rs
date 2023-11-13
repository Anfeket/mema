pub mod db;
use std::sync::{Arc, Mutex};

use crate::db::*;

#[derive(Clone)]
struct State {
    database: Arc<Mutex<db::localdb::LocalDatabase>>
}

fn main() {
    let meme = Meme::new("anfeket".to_string(), "google.com".to_string(), MediaType::Text);
    let database = db::localdb::LocalDatabase::new("pipik.data".to_string());
    let database = match database {
        Ok(database) => {
            println!("All well and ready!");
            database
        },
        Err(error) => {
            eprintln!("man shit fucked up: {:?}", error);
            return;
        }
    };
    let state = State{
        database: Arc::new(Mutex::new(database))
    };
    //let id = database.create_item(meme).unwrap();
    let mut database = state.database.lock().unwrap();
    let id = database.create_item(meme).unwrap();
    let result = database.get_item(id).unwrap().unwrap();
    let text = format!("Author: {}\nUrl: {}", result.author, result.url);
    println!("{}", text);
}
