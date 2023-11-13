pub mod db;
use crate::db::*;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct AppState {
    database: Arc<Mutex<db::localdb::LocalDatabase>>,
}

#[tokio::main]
async fn main() {
    let database = db::localdb::LocalDatabase::new("pipik.data".to_string());
    let database = match database {
        Ok(database) => {
            println!("All well and ready!");
            database
        }
        Err(error) => {
            eprintln!("man shit fucked up: {:?}", error);
            return;
        }
    };
    let state = AppState {
        database: Arc::new(Mutex::new(database)),
    };

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/items", get(get_items).post(post_item))
        .route("/items/:id", get(get_item).delete(delete_item))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 6969));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn to_json(item: Meme) -> String {
    serde_json::to_value(item).unwrap().to_string()
}

async fn hello_world() -> &'static str {
    "Hello world!"
}

async fn get_items(State(db): State<AppState>) -> impl IntoResponse {
    let database = db.database.lock().unwrap();
    let items = database.get_items();
    let res = serde_json::to_value(items).unwrap();
    res.to_string()
}

async fn post_item(State(db): State<AppState>, Json(payload): Json<Meme>) -> impl IntoResponse {
    let mut database = db.database.lock().unwrap();
    database.create_item(payload).unwrap();
}

async fn get_item(Path(id): Path<i32>, State(db): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let database = db.database.lock().unwrap();
    let item = database.get_item(id).unwrap();
    if item.is_none() {
        return Err(StatusCode::NOT_FOUND)
    } else {
        return Ok(to_json(item.unwrap()))
    }
}

async fn delete_item(Path(id): Path<i32>, State(db): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let mut database = db.database.lock().unwrap();
    match database.remove_item(id) {
        Err(_) => {
            Err(StatusCode::NOT_FOUND)
        }
        Ok(item) => {
            Ok(to_json(item))
        }
    }
}
