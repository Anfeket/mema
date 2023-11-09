pub mod db;
use crate::db::*;

fn main() {
    let meme = Meme::new("anfeket".to_string(), "google.com".to_string(), MediaType::Text);
    let database = db::localdb::LocalDatabase::new("pipik.data".to_string());
    let mut database = match database {
        Ok(database) => {
            println!("All well and ready!");
            database
        },
        Err(error) => {
            eprintln!("man shit fucked up: {:?}", error);
            return;
        }
    };
    let id = database.create_item(meme).unwrap();
    let result = database.get_item(id).unwrap().unwrap();
    let text = format!("Author: {}\nUrl: {}", result.author, result.url);
    println!("{}", text);
}
